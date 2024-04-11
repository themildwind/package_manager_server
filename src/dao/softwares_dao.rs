use std::{future::IntoFuture, result};

use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DeleteResult, EntityTrait, QueryFilter, QueryOrder, TryIntoModel};
use semver::Version;

use crate::{controller::SystemError, entities::{self, softwares}};

#[derive(Debug)]
pub struct SoftwaresDao;

impl SoftwaresDao{
    // 对用户
    // 根据名称和版本查地址
    pub async fn get_file_url_by_archive_version(archive: &str, version: &str,db : &DatabaseConnection) -> Result<String, SystemError> {
        let result  = crate::entities::softwares::Entity::find()
            .filter(crate::entities::softwares::Column::Archive.eq(archive))
            .filter(crate::entities::softwares::Column::Version.eq(version))
            .one(db).await;
        match result {
            Ok(opt) => {
                match opt {
                    Some(s) => {
                        return Ok(s.download.map(|s| s).unwrap());
                    },
                    None => {
                        return Err(SystemError::DbErr(sea_orm::DbErr::RecordNotFound("".to_string())));
                    }
                }
            },
            Err(e) => {
                return Err(SystemError::DbErr(e));
            }
        }
         
    }
    // 根据名称和版本查软件信息
    pub async fn get_information_by_archive_version(archive: &str, version: &str,db : &DatabaseConnection) -> Result<entities::softwares::Model, SystemError> {
        let result  = crate::entities::softwares::Entity::find()
            .filter(crate::entities::softwares::Column::Archive.eq(archive))
            .filter(crate::entities::softwares::Column::Version.eq(version))
            .one(db).await;
        match result {
            Ok(opt) => {
                match opt {
                    Some(s) => {
                        return Ok(s);
                    },
                    None => {
                        return Err(SystemError::DbErr(sea_orm::DbErr::RecordNotFound("".to_string())));
                    }
                }
            },
            Err(e) => {
                return Err(SystemError::DbErr(e));
            }
        }
    }
    // 根据名称查版本信息列表
    pub async fn get_version_list_by_archive(archive: &str,db : &DatabaseConnection) -> Result<Vec<crate::entities::softwares::Model>, SystemError> {
        let result = crate::entities::softwares::Entity::find()
            .filter(crate::entities::softwares::Column::Archive.eq(archive))
            .all(db).await;
        match result {
            Ok(opt) => {
                if opt.len() == 0{
                    return Err(SystemError::DbErr(sea_orm::DbErr::RecordNotFound("".to_string())));
                }
                else {
                    return Ok(opt);
                }
            },
            Err(e) => {
                return Err(SystemError::DbErr(e));
            }
        }
    }
    // 根据名称查最新版本
    pub async fn get_latest_version_by_archive(archive: &str,db : &DatabaseConnection) -> Result<crate::entities::softwares::Model, SystemError> {
        // 查找版本号最大的结果
        let result = crate::entities::softwares::Entity::find()
            .filter(crate::entities::softwares::Column::Archive.eq(archive))
            .order_by_desc(crate::entities::softwares::Column::VersionMajor)
            .order_by_desc(crate::entities::softwares::Column::VersionMinor)
            .order_by_desc(crate::entities::softwares::Column::VersionPatch)
            .one(db).await;
            
            match result {
                Ok(opt) => {
                    match opt {
                        Some(s) => {
                            return Ok(s);
                        },
                        None => {
                            return Err(SystemError::DbErr(sea_orm::DbErr::RecordNotFound("".to_string())));
                        }
                    }
                },
                Err(e) => {
                    return Err(SystemError::DbErr(e));
                }
            }
    }
    // 对管理员
    // 新增软件
    pub async fn add_software(software: &crate::entities::softwares::ActiveModel,db : &DatabaseConnection) -> Result<crate::entities::softwares::Model, SystemError> {
        let result = crate::entities::softwares::ActiveModel::insert(software.clone(), db).await;
        match result {
            Ok(r) => {
                return Ok(r);
            },
            Err(e) => {
                return Err(SystemError::DbErr(e));
            }
        }
    }
    // 根据名称和版本删除软件
    pub async fn delete_software_by_archive_version(archive: &str,version: &str,db : &DatabaseConnection) -> Result<DeleteResult, SystemError> {
        let active_model = crate::entities::softwares::ActiveModel {
            archive : sea_orm::ActiveValue::Set(archive.to_string()),
            version : sea_orm::ActiveValue::Set(version.to_string()),
            ..Default::default()
        };
        let result = active_model.delete(db).await;
        match result {
            Ok(r) => {
                return Ok(r);
            },
            Err(e) => {
                return Err(SystemError::DbErr(e));
            }
        }
    }
    // 根据名称删除所有版本软件
    pub async fn delete_software_by_archive(archive: &str,db : &DatabaseConnection) -> Result<DeleteResult, SystemError> {
        let result = crate::entities::softwares::Entity::delete_many()
            .filter(crate::entities::softwares::Column::Archive.eq(archive.to_string()))
            .exec(db)
            .await;
        match result {
            Ok(r) => {
                return Ok(r);
            },
            Err(e) => {
                return Err(SystemError::DbErr(e));
            }
        }
    }
    // 更新软件信息
    pub async fn update_software(software: &crate::entities::softwares::ActiveModel,db : &DatabaseConnection) -> Result<crate::entities::softwares::Model, SystemError> {
        let result = software.clone().update(db).await;
        match result {
            Ok(r) => {
                return Ok(r);
            },
            Err(e) => {
                return Err(SystemError::DbErr(e));
            }
        }
    }
    // 修改软件是否可用
    

}

pub enum SoftwareDaoError {
    //
    NotFound,
}