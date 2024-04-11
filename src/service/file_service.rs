use actix_web::Error;
use tar::Archive;
use std::{fs, io::Read};

use crate::controller::SystemError;

pub struct FileService;
impl FileService {
    // 根据地址获得压缩文件
    pub fn get_file_by_url(url : String) -> Result<Vec<u8>, SystemError>{
        let res = Self::get_file(url);
        match res {
            Ok(r) => return Ok(r),
            Err(e) => return Err(SystemError::IoErr),
        }
    }
    fn get_file(url : String) -> Result<Vec<u8>, Error>{
        //log::info!("{:?}", url);
        let mut open_res = fs::File::open(url)?;
        let mut file_contents = Vec::new();
        // let mut tar_archive = Archive::new(open_res);
        // for entry in tar_archive.entries()? {
        //     let mut entry = entry?;
        //     entry.read_to_end(&mut file_contents)?;
        // }
        open_res.read_to_end(&mut file_contents)?;
        return Ok(file_contents);
    }
}