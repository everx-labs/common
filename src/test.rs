include!("./log.rs");

pub fn init_test_log() {
    init_log("./common/config/log_cfg.yml")
}

// Some tests are tokio::test that do not need explicit runtime
#[allow(dead_code)]
pub fn init_test() -> tokio::runtime::Runtime {
    init_test_log();
    tokio::runtime::Runtime::new().unwrap()
}

