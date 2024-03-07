use super::{ResHelper, ResourceName};
use std::{
    io::BufRead,
    process::{Child, Command, Stdio},
};

use anyhow::{Context, Ok, Result};

#[derive(Clone, Debug)]
pub struct Device {
    pub id: String,
    pub status: String,
}

impl Device {
    /// execute "adb push" to push file from src to des
    pub fn cmd_push(&self, src: &str, des: &str) -> Result<String> {
        let mut adb_command = Adb::cmd_base();
        let res = adb_command
            .args(&["-s", &self.id, "push", src, des])
            .output()
            .with_context(|| format!("Failed to execute 'adb push {} {}'", src, des))?;
        Ok(String::from_utf8(res.stdout).unwrap())
    }

    /// execute "adb reverse" to reverse the device port to local port
    pub fn cmd_reverse(&self, remote: &str, local: &str) -> Result<()> {
        let mut adb_command = Adb::cmd_base();
        adb_command
            .args(&["-s", &self.id, "reverse", remote, local])
            .output()
            .with_context(|| format!("Failed to execute 'adb reverse {} {}'", remote, local))?;
        Ok(())
    }

    /// execute "adb forward" to forward the local port to the device
    pub fn cmd_forward(&self, local: &str, remote: &str) -> Result<()> {
        let mut adb_command = Adb::cmd_base();
        adb_command
            .args(&["-s", &self.id, "forward", local, remote])
            .output()
            .with_context(|| format!("Failed to execute 'adb forward {} {}'", local, remote))?;
        Ok(())
    }

    /// execute "adb shell" to execute shell command on the device
    pub fn cmd_shell(&self, shell_args: &[&str]) -> Result<Child> {
        let mut adb_command = Adb::cmd_base();
        let mut args = vec!["-s", &self.id, "shell"];
        args.extend_from_slice(shell_args);
        Ok(adb_command
            .args(args)
            .stdout(Stdio::piped())
            .spawn()
            .context("Failed to execute 'adb shell'")?)
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
    pub fn cmd_devices() -> Result<Vec<Device>> {
        let mut adb_command = Adb::cmd_base();
        let output = adb_command
            .args(&["devices"])
            .output()
            .context("Failed to execute 'adb devices'")?;

        let mut devices_vec: Vec<Device> = Vec::new();
        let mut lines = output.stdout.lines();
        // skip first line
        lines.next();

        // parse string to Device
        for line in lines {
            if let std::result::Result::Ok(s) = line {
                let device_info: Vec<&str> = s.split('\t').collect();
                if device_info.len() == 2 {
                    devices_vec.push(Device {
                        id: device_info[0].to_string(),
                        status: device_info[1].to_string(),
                    });
                }
            }
        }
        Ok(devices_vec)
    }

    /// execute "adb kill-server"
    pub fn cmd_kill_server() -> Result<()> {
        let mut adb_command = Adb::cmd_base();
        adb_command
            .args(&["kill-server"])
            .output()
            .context("Failed to execute 'adb kill-server'")?;
        Ok(())
    }

    /// execute "adb reverse --remove-all"
    pub fn cmd_reverse_remove() -> Result<()> {
        let mut adb_command = Adb::cmd_base();
        adb_command
            .args(&["reverse", " --remove-all"])
            .output()
            .context("Failed to execute 'adb reverse --remove-all'")?;
        Ok(())
    }

    /// execute "adb forward --remove-all"
    pub fn cmd_forward_remove() -> Result<()> {
        let mut adb_command = Adb::cmd_base();
        adb_command
            .args(&["forward", " --remove-all"])
            .output()
            .context("Failed to execute 'adb forward --remove-all'")?;
        Ok(())
    }

    /// execute "adb start-server"
    pub fn cmd_start_server() -> Result<()> {
        let mut adb_command = Adb::cmd_base();
        adb_command
            .args(&["start-server"])
            .output()
            .context("Failed to execute 'adb start-server'")?;
        Ok(())
    }
}
