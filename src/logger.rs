use chrono::Local;
use std::fs::File;
use std::io::Write;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

static LOG_FILE: OnceLock<Mutex<File>> = OnceLock::new();

pub static FLAGS: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();

fn getflags() -> std::sync::MutexGuard<'static, HashMap<String, String>> {
    FLAGS.get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap()
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
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
    // Get all flags at once and release the lock immediately
    let debug_flag = {
        let flags = getflags();
        flags.get("debug").cloned()
    };
    
    let ignore_flag = {
        let flags = getflags();
        flags.get("ignore").cloned()
    };
    
    let precise_flag = {
        let flags = getflags();
        flags.get("precise").cloned()
    };
    
    let datetime_flag = {
        let flags = getflags();
        flags.get("datetime").cloned()
    };
    
    let ansi_flag = {
        let flags = getflags();
        flags.get("ansi").cloned()
    };
    
    let disk_flag = {
        let flags = getflags();
        flags.get("disk").cloned()
    };

    if level == Levels::Debug {
        if let Some(flag) = debug_flag {
            if flag == "false" {
                return;
            }
        }
    }

    if let Some(flag) = ignore_flag {
        if let Ok(ignore_value) = flag.parse::<i32>() {
            if ignore_value > 0 && (level as i32) > 0 && (level as i32) <= ignore_value {
                return;
            }
        }
    }

    let mut time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    if let Some(flag) = precise_flag {
        if flag == "true" {
            time = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
        }
    }

    if let Some(flag) = datetime_flag {
        if flag != "true" {
            time = "".to_string();
        }
    }
    
    let strlevel = match level {
        Levels::Debug => "DEBUG",
        Levels::Info  => "INFO",
        Levels::Warning => "WARNING",
        Levels::Error => "ERROR",
        Levels::Fatal => "FATAL",
    };
    
    let ansicolor;
    
    if let Some(flag) = ansi_flag {
        if flag == "true" {
            ansicolor = match level {
                Levels::Debug => "\x1b[0m",
                Levels::Info  => "\x1b[36m",
                Levels::Warning => "\x1b[33m",
                Levels::Error => "\x1b[31m",
                Levels::Fatal => "\x1b[101m\x1b[30m",
            };
        } else {
            ansicolor = "";
        }
    } else {
        ansicolor = "";
    }

    println!("{ansicolor}{time} [{strlevel}]: {message}\x1b[0m @ {macro_file}:{macro_line}\n");

    let write_to_disk = disk_flag.map(|f| f == "true").unwrap_or(false);

    if write_to_disk {
        std::thread::scope(|s| {
            s.spawn(move || {
                if let Some(mutex) = LOG_FILE.get() {
                    if let Ok(mut file) = mutex.lock() {
                        let _ = writeln!(file, "{time} [{strlevel}]: {message} @ {macro_file}:{macro_line}");
                    }
                }
            });
        });
    }


}

pub fn log_setoutfile(outputfile: File) {
    let _ = LOG_FILE.set(Mutex::new(outputfile));
    getflags().insert("disk".to_string(), "true".to_string());
}

pub fn log_setflag(key: &str, value: &str) {
    getflags().insert(key.to_string(), value.to_string());
}