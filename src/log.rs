fn init_log(config: &str) {
    if !log::log_enabled!(log::Level::Error) {
        log4rs::init_file(config, Default::default())
            .expect(&format!("Cannot read logging configuration from {}", config));
    }
}

fn init_test_log() {
    init_log("./common/config/log_cfg.yml")
}

macro_rules! dump {
    ($data: expr) => {
        {
            let mut dump = String::new();
            for i in 0..$data.len() {
                dump.push_str(
                    &format!(
                        "{:02x}{}", 
                        $data[i], 
                        if (i + 1) % 16 == 0 { '\n' } else { ' ' }
                    )
                )
            }
            dump
        }
    };
    (debug, $target:expr, $msg:expr, $data:expr) => {
        if log::log_enabled!(log::Level::Debug) {
            log::debug!(target: $target, "{}:\n{}", $msg, dump!($data))
        }
    };
    (trace, $target:expr, $msg:expr, $data:expr) => {
        if log::log_enabled!(log::Level::Trace) {
            log::trace!(target: $target, "{}:\n{}", $msg, dump!($data))
        }
    }
}