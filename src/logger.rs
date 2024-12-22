
use std::sync::Once;

static INIT: Once = Once::new();


pub struct Logger;

pub enum LogLevel {
    Debug,
    Error,
    Warn,
    Info,
    Trace
}

impl Logger {
    pub fn init() {
        INIT.call_once(|| {});
    }
    pub fn log(level: LogLevel, message: &str) {
        let color = match level {
            LogLevel::Debug => "\x1b[32m",
            LogLevel::Error => "\x1b[31m",
            LogLevel::Warn => "\x1b[33m",
            LogLevel::Info => "\x1b[34m",
            LogLevel::Trace => "\x1b[35m"
        };

        let reset = "\x1b[0m";

        let level_str = match level {
            LogLevel::Debug => "DEBUG",
            LogLevel::Error => "ERROR",
            LogLevel::Warn => "WARN",
            LogLevel::Info => "INFO",
            LogLevel::Trace => "TRACE"
        };

        println!("{}[{}]{}: {}", color, level_str, reset, message);
    }
    
}
