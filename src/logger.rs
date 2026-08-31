use chrono::Local;
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;
use std::sync::OnceLock;

static LOG_FILE: OnceLock<Mutex<File>> = OnceLock::new();

#[derive(Copy, Clone)]
pub enum Levels {
    Debug   = 0,
    Info    = 1,
    Warning = 2,
    Error   = 3,
    Fatal   = 4
}

#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)*) => {{
        let message = format!($($arg)*);
        internal_log($level, &message, file!(), line!());
    }};
}

pub fn internal_log(level: Levels, message: &str, macro_file: &str, macro_line: u32) {
    let time  = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    let strlevel = match level {
        Levels::Debug => "DEBUG",
        Levels::Info  => "INFO",
        Levels::Warning => "WARNING",
        Levels::Error => "ERROR",
        Levels::Fatal => "FATAL",
    };
    
    let ansicolor = match level {
        Levels::Debug => "\x1b[0m",
        Levels::Info  => "\x1b[36m",
        Levels::Warning => "\x1b[33m",
        Levels::Error => "\x1b[31m",
        Levels::Fatal => "\x1b[101m\x1b[30m",
    };
    
    println!("{ansicolor}{time} [{strlevel}]: {message}\x1b[0m @ {macro_file}:{macro_line}\n");

    if let Some(mutex) = LOG_FILE.get() {
        if let Ok(mut file) = mutex.lock() {
            let _ = writeln!(file, "{time} [{strlevel}]: {message} @ {macro_file}:{macro_line}");
        }
    }
}

pub fn log_setoutfile(outputfile: File) {
    let _ = LOG_FILE.set(Mutex::new(outputfile));
}
