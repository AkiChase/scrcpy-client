use scrcpy_client::util::ResHelper;

#[test]
fn test_res_init() {
    if let Err(val) = ResHelper::res_init() {
        println!("{}", val);
    }else {
        println!("res init success");
    }
}

