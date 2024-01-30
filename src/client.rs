use rand::Rng;
use std::{io::BufRead, thread};

/**
 * the client of scrcpy
 */
use super::util::{Device, ResHelper, ResourceName};

#[derive(Debug)]
pub struct ScrcpyClient {
    pub device: Device,
    pub version: String,
    pub scid: String,
}

impl ScrcpyClient {
    pub fn new(device: Device) -> ScrcpyClient {
        let file_name = ResHelper::get_file_path(ResourceName::ScrcpyServer);
        let version = &file_name[file_name.rfind('v').unwrap() + 1..];

        // 16bit scid (no need to 31bit)
        let mut rng = rand::thread_rng();
        let scid = rng.gen_range(0..(1 << 16));

        Self {
            device,
            version: version.to_string(),
            scid: scid.to_string(),
        }
    }

    /// push server file to current device
    pub fn push_server_file(&self) {
        let res = self.device.cmd_push(
            ResHelper::get_file_path(ResourceName::ScrcpyServer),
            "/data/local/tmp/scrcpy-server.jar",
        );

        match res {
            Ok(info) => {
                println!("Successfully push server files.\n{}", info);
            }
            Err(e) => {
                eprintln!("Failed to push server files.\n{}", e);
            }
        }
    }

    /// reverse the device port to local port
    pub fn reverse_server_port(&self) {
        if let Err(e) = self
            .device
            .cmd_reverse(&format!("localabstract:scrcpy_{}", self.scid), "tcp:27183")
        {
            eprintln!("Failed to reverse port.\n{}", e);
        } else {
            println!("Successfully reverse port.");
        }
    }

    /// spawn a new thread to start scrcpy server
    pub fn shell_start_server(&self) -> thread::JoinHandle<()> {
        let res = self.device.cmd_shell(&[
            "CLASSPATH=/data/local/tmp/scrcpy-server.jar",
            "app_process",
            "/",
            "com.genymobile.scrcpy.Server",
            &self.version,
            &format!("scid={}", self.scid),
        ]);

        thread::spawn(|| {
            match res {
                Err(e) => {
                    eprintln!("start failed.\n{}", e);
                }
                Ok(mut child) => {
                    println!("starting server...");
                    let out = child.stdout.take().unwrap();
                    let mut out = std::io::BufReader::new(out);
                    let mut s = String::new();

                    while let Ok(_) = out.read_line(&mut s) {
                        // break at the end of program
                        if let Ok(Some(_)) = child.try_wait() {
                            break;
                        }
                        print!("{}", s);
                        // clear string to store new line only
                        s.clear();
                    }
                }
            }
        })
    }
}
