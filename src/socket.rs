use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::task::JoinHandle;

use crate::video;

pub fn connect_video_socket(port: u16, first: bool) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port))
            .await
            .unwrap();

        if first {
            handle_device_metadata(&mut stream).await;
        }
        video::handle_video_stream(&mut stream).await;
    })
}

pub async fn handle_device_metadata(stream: &mut TcpStream) {
    println!("Handle device metedata");
    let mut buf_one_byte = [0; 1];
    stream
        .read_exact(&mut buf_one_byte)
        .await
        .expect("Dummy byte not fonund!");
    println!("Dummy byte received");

    let mut buf = [0; 64];
    match stream.read_exact(&mut buf).await {
        Ok(_n) => {
            let device_name = std::str::from_utf8(&buf);
            println!("Device Name: {}", device_name.unwrap());
        }
        Err(e) => {
            eprintln!("{}", e);
            panic!("Failed to read device name!");
        }
    };
}
