use scrcpy_client::client::ScrcpyClient;
use scrcpy_client::util::Adb;
use tokio::runtime::Runtime;

#[test]
fn test_socket_server() {
    let devices = Adb::cmd_devices().unwrap();
    if devices.len() < 1 {
        println!("no devices!");
    } else {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let client: ScrcpyClient = ScrcpyClient::new(devices[0].clone(), 27183);

            client.forward_server_port();

            client.push_server_file();

            //new server thread
            let server_thread = client.shell_start_server();

            //new several tokio thread
            client.open_client_socket();
            // 解码器
            // gui展示解码画面
            // gui展示调试信息
            // 开启其他socket...

            // join server thread to main thread
            server_thread.join().unwrap();
        });
    }
}

#[test]
fn test_push_server_file() {
    let devices = Adb::cmd_devices().unwrap();
    if devices.len() < 1 {
        println!("no devices!");
    } else {
        let client: ScrcpyClient = ScrcpyClient::new(devices[0].clone(), 27183);
        println!("{:?}", client);
        client.push_server_file();
    }
}

#[test]
fn test_reverse_server_port() {
    let devices = Adb::cmd_devices().unwrap();
    if devices.len() < 1 {
        println!("no devices!");
    } else {
        let client: ScrcpyClient = ScrcpyClient::new(devices[0].clone(), 27183);
        client.reverse_server_port();
    }
}

#[test]
fn test_forward_server_port() {
    let devices = Adb::cmd_devices().unwrap();
    if devices.len() < 1 {
        println!("no devices!");
    } else {
        let client: ScrcpyClient = ScrcpyClient::new(devices[0].clone(), 27183);
        client.forward_server_port();
    }
}

#[test]
fn test_shell_start_server() {
    let devices = Adb::cmd_devices().unwrap();
    if devices.len() < 1 {
        println!("no devices!");
    } else {
        let client: ScrcpyClient = ScrcpyClient::new(devices[0].clone(), 27183);
        let h = client.shell_start_server();
        h.join().unwrap();
    }
}

// #[test]
// fn test_client_fullly() {
//     let devices = Adb::cmd_devices().unwrap();
//     if devices.len() < 1 {
//         println!("no devices!");
//     } else {
//         let client: ScrcpyClient = ScrcpyClient::new(devices[0].clone(), 27183);
//         client.push_server_file();
//         client.reverse_server_port();
//         // todo: open socket
//         let h = client.shell_start_server();
//         h.join().unwrap();
//     }
// }
