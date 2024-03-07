use rand::Rng;
use std::{io::BufRead, thread, time::Duration};
use tokio::task::JoinHandle;

use crate::{
    socket,
    util::{Device, ResHelper, ResourceName},
};

/**
 * the client of scrcpy
 */
#[derive(Debug)]
pub struct ScrcpyClient {
    pub device: Device,
    pub version: String,
    pub scid: String,
    pub port: u16,
}

impl ScrcpyClient {
    pub fn new(device: Device, port: u16) -> ScrcpyClient {
        let file_name = ResHelper::get_file_path(ResourceName::ScrcpyServer);
        let version = &file_name[file_name.rfind('v').unwrap() + 1..];

        // 16bit scid (no need to 31bit)
        let mut rng = rand::thread_rng();
        let scid = format!("{:08x}", rng.gen_range(0..(1 << 16)));

        Self {
            device,
            version: version.to_string(),
            scid,
            port,
        }
    }

    /// start socket server and accept connection
    pub fn open_client_socket(&self) {
        println!("Waiting 1s before connect to scrcpy server...");
        thread::sleep(Duration::from_secs(1));

        let port = self.port;
        socket::connect_video_socket(port, true);
    }

    /// push server file to current device
    pub fn push_server_file(&self) {
        let info = self
            .device
            .cmd_push(
                ResHelper::get_file_path(ResourceName::ScrcpyServer),
                "/data/local/tmp/scrcpy-server.jar",
            )
            .unwrap();

        println!("Successfully push server files.\n{}", info);
    }

    /// forward the local port to the device
    pub fn forward_server_port(&self) {
        self.device
            .cmd_forward("tcp:27183", &format!("localabstract:scrcpy_{}", self.scid))
            .unwrap();
        println!("Successfully forward port");
    }

    /// reverse the device port to the local port
    pub fn reverse_server_port(&self) {
        self.device
            .cmd_reverse(&format!("localabstract:scrcpy_{}", self.scid), "tcp:27183")
            .unwrap();
        println!("Successfully reverse port");
    }

    /// spawn a new thread to start scrcpy server
    pub fn shell_start_server(&self) -> JoinHandle<()> {
        let mut child = self
            .device
            .cmd_shell(&[
                "CLASSPATH=/data/local/tmp/scrcpy-server.jar",
                "app_process",
                "/",
                "com.genymobile.scrcpy.Server",
                &self.version,
                &format!("scid={}", self.scid),
                "audio=false",
                "control=false",
                "tunnel_forward=true",
            ])
            .unwrap();

        tokio::task::spawn_blocking(move || {
            println!("Starting scrcpy server...");
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
        })
    }
}
