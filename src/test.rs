use crate::logger::{
    Levels, internal_log, log_getflag, log_setflag, log_setoutfile, log_settarget,
    log_stats_snapshot,
};
use std::fs::File;
use std::io::Read;
use std::sync::Mutex;

static TEST_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn test_ansi() {
    let _guard = TEST_LOCK.lock().unwrap();
    log_setflag("ansi", "true");
    log_setflag("datetime", "false");
    log_setflag("disk", "false");

    assert_eq!(log_getflag("ansi").as_deref(), Some("true"));
    internal_log(Levels::Info, "ANSI output test", file!(), line!());
}

#[test]
fn test_counters() {
    let _guard = TEST_LOCK.lock().unwrap();
    log_setflag("debug", "true");
    log_setflag("ignore", "debug");
    log_setflag("ansi", "false");
    log_setflag("datetime", "false");
    log_setflag("disk", "false");

    let before = log_stats_snapshot();
    internal_log(Levels::Debug, "counter test", file!(), line!());
    internal_log(Levels::Info, "counter test", file!(), line!());
    internal_log(Levels::Warning, "counter test", file!(), line!());
    internal_log(Levels::Error, "counter test", file!(), line!());
    internal_log(Levels::Fatal, "counter test", file!(), line!());
    let after = log_stats_snapshot();

    assert_eq!(
        [
            after[0] - before[0],
            after[1] - before[1],
            after[2] - before[2],
            after[3] - before[3],
            after[4] - before[4],
        ],
        [1; 5]
    );
}

#[test]
fn test_file() {
    let _guard = TEST_LOCK.lock().unwrap();
    let path =
        std::env::temp_dir().join(format!("touchtrack_logger_test_{}.log", std::process::id()));
    let outputfile = File::create(&path).unwrap();
    log_setoutfile(outputfile);
    log_settarget("file-test");
    log_setflag("ansi", "false");
    log_setflag("datetime", "false");

    let marker = format!("file output test {}", std::process::id());
    internal_log(Levels::Info, &marker, file!(), line!());

    let mut contents = String::new();
    File::open(&path)
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();
    assert!(contents.contains("<file-test>"));
    assert!(contents.contains(&marker));
    std::fs::remove_file(path).unwrap();
}

#[test]
fn test_flags() {
    let _guard = TEST_LOCK.lock().unwrap();
    log_setflag("debug", "false");
    log_setflag("ignore", "warning");

    assert_eq!(log_getflag("debug").as_deref(), Some("false"));
    assert_eq!(log_getflag("ignore").as_deref(), Some("warning"));
}
