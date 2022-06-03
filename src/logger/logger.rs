use std::env;
use std::fmt::{Display, Debug};

pub enum LogLevel {
    Debug,
    Info,
    Log,
    Warn,
    Error,
}

impl LogLevel {
    pub fn to_number(&self) -> u8 {
        match self {
            LogLevel::Debug => 0,
            LogLevel::Info => 1,
            LogLevel::Log => 2,
            LogLevel::Warn => 3,
            LogLevel::Error => 4,
        }
    }

    pub fn from_number(number: u8) -> LogLevel {
        match number {
            0 => LogLevel::Debug,
            1 => LogLevel::Info,
            2 => LogLevel::Log,
            3 => LogLevel::Warn,
            4 => LogLevel::Error,
            _ => {
                println!("{}: {}", "LogLevel to high", number);
                println!("{}", "Chosen LogLevel::Log");
                LogLevel::Log
            }
        }
    }

    pub fn from_string(string: &str) -> LogLevel {
        let lower_string = &string.to_lowercase()[..];

        match lower_string {
            "debug" => LogLevel::Debug,
            "info" => LogLevel::Info,
            "log" => LogLevel::Log,
            "warn" => LogLevel::Warn,
            "error" => LogLevel::Error,
            _ => {
                println!("{}: {}", "LogLevel unknown", string);
                println!("{}", "Chosen LogLevel::Log");
                LogLevel::Log
            }
        }
    }

    pub fn from_env() -> LogLevel {
        let log_level = env::var_os("LOG_LEVEL");

        match log_level {
            Some(level) => {
                let var = level.to_str().unwrap();
                LogLevel::from_string(var)
            },
            None => {
                println!("{}", "Could not get ENV for log level. LogLevel::Log chosen.");
                LogLevel::Log
            }
        }
    }
}

pub trait Loggable: Debug + Display {}
impl<T> Loggable for T where T: Debug + Display {}

pub struct Logger {
    log_level: LogLevel,
    log_level_number: u8,
}

impl Logger {
    pub fn new(log_level: Option<LogLevel>) -> Self {
        let final_log_level = log_level.unwrap_or(LogLevel::from_env());
        let log_level_number = final_log_level.to_number();

        Self {
            log_level: final_log_level,
            log_level_number,
        }
    }

    pub fn debug<T>(&self, msg: T) -> () where T: Loggable {
        if self.log_level_number <= LogLevel::Debug.to_number() {
            println!("{}", msg)
        }
    }

    pub fn info<T>(&self, msg: T) -> () where T: Loggable {
        if self.log_level_number <= LogLevel::Info.to_number() {
            println!("{}", msg)
        }
    }

    pub fn log<T>(&self, msg: T) -> () where T: Loggable {
        if self.log_level_number <= LogLevel::Log.to_number() {
            println!("{}", msg)
        }
    }

    pub fn warn<T>(&self, msg: T) -> () where T: Loggable {
        if self.log_level_number <= LogLevel::Warn.to_number() {
            println!("{}", msg)
        }
    }

    pub fn error<T>(&self, msg: T) -> () where T: Loggable {
        if self.log_level_number <= LogLevel::Error.to_number() {
            println!("{}", msg)
        }
    }
}