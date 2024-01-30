use crate::util::AppError;

use super::{ResHelper, ResourceName};
use std::{
    io::BufRead,
    process::{Child, Command, Stdio},
};

#[derive(Clone, Debug)]
pub struct Device {
    pub id: String,
    pub status: String,
}

impl Device {
    /// execute "adb push" to push file from src to des
    pub fn cmd_push(&self, src: &str, des: &str) -> Result<String, AppError> {
        let mut adb_command = Adb::cmd_base();
        let res = adb_command
            .args(&["-s", &self.id, "push", src, des])
            .output();

        match res {
            Ok(output) => {
                return Ok(String::from_utf8(output.stdout).unwrap());
            }
            Err(e) => {
                return Err(AppError {
                    type_name: "Adb".to_string(),
                    message: e.to_string(),
                })
            }
        }
    }

    /// execute "adb reverse" to reverse the device port to local port
    pub fn cmd_reverse(&self, remote: &str, local: &str) -> Result<(), AppError> {
        let mut adb_command = Adb::cmd_base();
        let res = adb_command
            .args(&["-s", &self.id, "reverse", remote, local])
            .output();
        if let Err(e) = res {
            return Err(AppError {
                type_name: "Adb".to_string(),
                message: e.to_string(),
            });
        } else {
            return Ok(());
        }
    }

    /// execute "adb shell" to execute shell command on the device
    pub fn cmd_shell(&self, shell_args: &[&str]) -> Result<Child, AppError> {
        let mut adb_command = Adb::cmd_base();
        let mut args = vec!["-s", &self.id, "shell"];
        args.extend_from_slice(shell_args);
        let res = adb_command.args(args).stdout(Stdio::piped()).spawn();
        match res {
            Ok(child)=>{
                return Ok(child);
            },
            Err(e)=>{
                return Err(AppError {
                    type_name: "Adb".to_string(),
                    message: e.to_string(),
                });
            }
        }
    }
}

pub struct Adb;

/// Module to execute adb command and fetch output.
/// But some output of command won't be output, like adb service startup information.
impl Adb {
    fn cmd_base() -> Command {
        Command::new(ResHelper::get_file_path(ResourceName::Adb))
    }

    /// execute "adb devices" and return devices list
    pub fn cmd_devices() -> Result<Vec<Device>, AppError> {
        let mut adb_command = Adb::cmd_base();
        let res = adb_command.args(&["devices"]).output();

        match res {
            Ok(output) => {
                let mut devices_vec: Vec<Device> = Vec::new();
                let mut lines = output.stdout.lines();
                // skip first line
                lines.next();

                // parse string to Device
                for line in lines {
                    if let Ok(s) = line {
                        let device_info: Vec<&str> = s.split('\t').collect();
                        if device_info.len() == 2 {
                            devices_vec.push(Device {
                                id: device_info[0].to_string(),
                                status: device_info[1].to_string(),
                            })
                        }
                    }
                }
                return Ok(devices_vec);
            }
            Err(e) => {
                return Err(AppError {
                    type_name: "Adb".to_string(),
                    message: e.to_string(),
                })
            }
        }
    }

    /// execute "adb kill-server"
    pub fn cmd_kill_server() -> Result<(), AppError> {
        let mut adb_command = Adb::cmd_base();
        let res = adb_command.args(&["kill-server"]).output();
        if let Err(e) = res {
            return Err(AppError {
                type_name: "Adb".to_string(),
                message: e.to_string(),
            });
        } else {
            return Ok(());
        }
    }

    /// execute "adb start-server"
    pub fn cmd_start_server() -> Result<(), AppError> {
        let mut adb_command = Adb::cmd_base();
        let res = adb_command.args(&["start-server"]).output();
        if let Err(e) = res {
            return Err(AppError {
                type_name: "Adb".to_string(),
                message: e.to_string(),
            });
        } else {
            return Ok(());
        }
    }
}
