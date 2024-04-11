use sea_orm::{DatabaseConnection, DeleteResult, Set};
use crate::{controller::SystemError, dao::softwares_dao::SoftwaresDao, entities::{self}};
pub struct SoftwaresService ;
impl SoftwaresService {
    
    // 对用户
    // 根据名称和版本查地址
    pub async fn get_file_url_by_archive_version(archive: &str, version: &str,db : &DatabaseConnection) -> Result<String, SystemError> {
        return SoftwaresDao::get_file_url_by_archive_version(archive, version, db).await;
    }
    // 根据名称和版本查软件信息
    pub async fn get_information_by_archive_version(archive: &str, version: &str,db : &DatabaseConnection) -> Result<entities::softwares::Model, SystemError> {
        return SoftwaresDao::get_information_by_archive_version(archive, version, db).await; 
    }
    // 根据名称查版本信息列表
    pub async fn get_version_list_by_archive(archive: &str,db : &DatabaseConnection) -> Result<Vec<crate::entities::softwares::Model>, SystemError> {
        
        return SoftwaresDao::get_version_list_by_archive(archive, db).await;
    }
    // 根据名称查最新版本
    pub async fn get_latest_version_by_archive(archive: &str,db : &DatabaseConnection) -> Result<crate::entities::softwares::Model, SystemError> {
        return SoftwaresDao::get_latest_version_by_archive(archive, db).await;
    }
    // 对管理员
    // 新增软件
    pub async fn add_software(software: &crate::entities::softwares::ActiveModel,db : &DatabaseConnection) -> Result<crate::entities::softwares::Model, SystemError> {
        return SoftwaresDao::add_software(software, db).await;
    }
    // 删除软件
    pub async fn delete_software(archive: &str,version: &str,db : &DatabaseConnection) -> Result<DeleteResult, SystemError> {
        let result;
        if version == "0.0.0" {
            result = SoftwaresDao::delete_software_by_archive(archive, db).await;
        }else {
            result = SoftwaresDao::delete_software_by_archive_version(archive, version, db).await;
        }
        return result;
    }
    // 更新软件信息
    pub async fn update_software(software: &crate::entities::softwares::Model,db : &DatabaseConnection) -> Result<crate::entities::softwares::Model, SystemError> {
        let mut active_model = crate::entities::softwares::ActiveModel{
            archive : sea_orm::ActiveValue::Set(software.archive.clone()),
            version : sea_orm::ActiveValue::Set(software.version.clone()),
            version_major : sea_orm::ActiveValue::Set(software.version_major),
            version_minor : sea_orm::ActiveValue::Set(software.version_minor),
            version_patch : sea_orm::ActiveValue::Set(software.version_patch),
            ..Default::default()
        };
        if software.component.is_some() {
            active_model.component = Set(software.component.clone());
        }
        if software.origin.is_some() {
            active_model.origin = Set(software.origin.clone());
        }
        if software.label.is_some() {
            active_model.label = Set(software.label.clone());
        }
        if software.architecture.is_some() {
            active_model.architecture = Set(software.architecture.clone());
        }
        if software.download.is_some() {
            active_model.download = Set(software.download.clone());
        }
        if software.others.is_some() {
            active_model.others = Set(software.others.clone());
        }
        if software.flag.is_some() {
            active_model.flag = Set(software.flag.clone());
        }
        return SoftwaresDao::update_software(&active_model, db).await;
    }
    // 修改软件是否可用
}