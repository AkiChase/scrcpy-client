use tokio::{io::AsyncReadExt, net::TcpStream};

/// read video stream
pub async fn handle_video_stream(stream: &mut TcpStream) {
    println!("Reading vedio codec metadata");
    let mut buf = [0; 12];
    let codec_id: &str;
    let width: u32;
    let height: u32;
    match stream.read_exact(&mut buf).await {
        Ok(n) => {
            if n != 12 {
                panic!("Invalid codec metadata length:{}", n);
            }
            codec_id = std::str::from_utf8(&buf[0..4]).unwrap();
            width = u32::from_be_bytes(buf[4..8].try_into().unwrap());
            height = u32::from_be_bytes(buf[8..12].try_into().unwrap());
            println!("codec id:{}, vedio size:{}*{}", codec_id, width, height);
        }
        Err(e) => panic!("Failed to read vedio codec metadata; err = {:?}", e),
    };

    println!("Reading vedio data");
    let mut buf = [0; 1024];
    loop {
        match stream.read(&mut buf).await {
            Ok(0) => {
                println!("Video transmission ended!");
                return;
            }
            Ok(n) => {
                println!("read video {} byte", n);
            }
            Err(e) => panic!("Failed to read vedio data; err = {:?}", e),
        };
    }
}
