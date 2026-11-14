mod configreader;
mod logger;
#[cfg(test)]
mod test;
use crate::logger::{
    Levels, internal_log, load_config, log_getflag, log_getstats, log_setflag, log_setoutfile,
    log_settarget,
};
use std::fs::File;

fn main() -> std::io::Result<()> {
    load_config("touchtrack.conf")?;
    let outputfile = File::create("log.txt")?;
    log_setoutfile(outputfile);
    log_settarget("initializer");

    log!(
        Levels::Info,
        "Debug messages will be shown: {:?}",
        log_getflag("debug").unwrap()
    );

    log!(Levels::Info, "Started logging session");
    log!(Levels::Debug, "This is the last debug message you will see");
    log_setflag("debug", "false");
    log_settarget("tests");
    log!(Levels::Info, "Starting a lot of messages to test speed...");
    log!(Levels::Warning, "This is a warning");
    log!(Levels::Debug, "This will be hided");
    log!(Levels::Error, "{} went wrong", "Something");
    log!(Levels::Warning, "This is a warning");
    log!(Levels::Debug, "This will be hided");
    log!(Levels::Error, "{} went wrong", "Something");
    log!(Levels::Warning, "This is a warning");
    log!(Levels::Debug, "This will be hided");
    log!(Levels::Error, "{} went wrong", "Something");
    log!(Levels::Warning, "This is a warning");
    log!(Levels::Debug, "This will be hided");
    log!(Levels::Error, "{} went wrong", "Something");
    log!(Levels::Warning, "This is a warning");
    log!(Levels::Debug, "This will be hided");
    log!(Levels::Error, "{} went wrong", "Something");
    log!(Levels::Fatal, "Ending logging showcase session");
    log_getstats(true);
    log_getstats(false);
    Ok(())
}
