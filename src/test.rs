include!("./log.rs");

pub fn init_test_log() {
    init_log("./common/config/log_cfg.yml")
}

pub fn init_test() -> tokio::runtime::Runtime {
    init_test_log();
    tokio::runtime::Runtime::new().unwrap()
}

