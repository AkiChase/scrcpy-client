use std::path::Path;

use super::AppError;

pub enum ResourceName {
    Adb,
    ScrcpyServer,
}

pub struct ResHelper;

impl ResHelper {
    pub fn res_init() -> Result<(), AppError> {
        for p in [Path::new("res"), Path::new("res/adb")] {
            if !p.exists() {
                return Err(AppError {
                    type_name: "ResHelper".to_string(),
                    message: format!("Resource missing! {}", p.to_str().unwrap()),
                });
            }
        }

        Ok(())
    }

    pub fn get_file_path(file_name: ResourceName) -> Result<&'static str, AppError> {
        match file_name {
            ResourceName::Adb => Ok("res/adb"),
            ResourceName::ScrcpyServer => Ok("res/scrcpy-server-v2.3.1")
        }
    }
}
