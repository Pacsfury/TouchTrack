use chrono::Local;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};

static LOG_FILE: OnceLock<Mutex<File>> = OnceLock::new();

pub static FLAGS: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();

fn getflags() -> std::sync::MutexGuard<'static, HashMap<String, String>> {
    FLAGS
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap()
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Levels {
    Debug = 0,
    Info = 1,
    Warning = 2,
    Error = 3,
    Fatal = 4,
}

fn parse_level(value: &str) -> Option<Levels> {
    match value.to_ascii_lowercase().as_str() {
        "debug" => Some(Levels::Debug),
        "info" => Some(Levels::Info),
        "warning" | "warn" => Some(Levels::Warning),
        "error" => Some(Levels::Error),
        "fatal" => Some(Levels::Fatal),
        _ => None,
    }
}

static CDEBUG: AtomicUsize = AtomicUsize::new(0);
static CINFO: AtomicUsize = AtomicUsize::new(0);
static CWARNING: AtomicUsize = AtomicUsize::new(0);
static CERROR: AtomicUsize = AtomicUsize::new(0);
static CFATAL: AtomicUsize = AtomicUsize::new(0);

static TARGET: OnceLock<Mutex<String>> = OnceLock::new();

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
    match level {
        Levels::Debug => {
            CDEBUG.fetch_add(1, Ordering::Relaxed);
        }
        Levels::Info => {
            CINFO.fetch_add(1, Ordering::Relaxed);
        }
        Levels::Warning => {
            CWARNING.fetch_add(1, Ordering::Relaxed);
        }
        Levels::Error => {
            CERROR.fetch_add(1, Ordering::Relaxed);
        }
        Levels::Fatal => {
            CFATAL.fetch_add(1, Ordering::Relaxed);
        }
    }

    if ignore_flag
        .as_deref()
        .and_then(parse_level)
        .is_some_and(|minimum| level < minimum)
    {
        return;
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
        Levels::Info => "INFO",
        Levels::Warning => "WARNING",
        Levels::Error => "ERROR",
        Levels::Fatal => "FATAL",
    };

    let ansicolor;

    if let Some(flag) = ansi_flag {
        if flag == "true" {
            ansicolor = match level {
                Levels::Debug => "\x1b[0m",
                Levels::Info => "\x1b[36m",
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
    let dtarget = if let Some(mutex) = TARGET.get() {
        let target_str = mutex.lock().unwrap();
        format!("<{}> ", *target_str)
    } else {
        String::from("")
    };

    println!(
        "{ansicolor}{dtarget}{time} [{strlevel}]: {message}\x1b[0m @ {macro_file}:{macro_line}\n"
    );

    let write_to_disk = disk_flag.map(|f| f == "true").unwrap_or(false);

    if write_to_disk {
        std::thread::scope(|s| {
            s.spawn(move || {
                if let Some(mutex) = LOG_FILE.get() {
                    if let Ok(mut file) = mutex.lock() {
                        let _ = writeln!(
                            file,
                            "{dtarget}{time} [{strlevel}]: {message} @ {macro_file}:{macro_line}"
                        );
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

pub fn log_getflag(key: &str) -> Option<String> {
    let flags = getflags();
    return flags.get(key).cloned();
}

pub fn log_settarget(newtarget: &str) {
    let mutex = TARGET.get_or_init(|| Mutex::new(String::new()));
    *mutex.lock().unwrap() = newtarget.to_string();
}

pub fn log_getstats(verbose: bool) {
    let debug = CDEBUG.load(Ordering::Relaxed);
    let info = CINFO.load(Ordering::Relaxed);
    let warning = CWARNING.load(Ordering::Relaxed);
    let error = CERROR.load(Ordering::Relaxed);
    let fatal = CFATAL.load(Ordering::Relaxed);

    if verbose {
        println!(
            "\x1b[0mDebug: {}\n\x1b[36mInfo: {}\n\x1b[33mWarning: {}\n\x1b[31mError: {}\n\x1b[101m\x1b[30mFatal: {}\x1b[0m\n\n",
            debug, info, warning, error, fatal
        );
    } else {
        println!("{} {} {} {} {}", debug, info, warning, error, fatal);
    }
}
