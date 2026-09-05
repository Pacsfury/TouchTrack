mod logger;
use std::fs::File;
use crate::logger::{internal_log, log_setoutfile, Levels, log_setflag};

fn main() -> std::io::Result<()> {
    let outputfile = File::create("log.txt")?;
    log_setoutfile(outputfile);
    log_setflag("debug", "true");
    log_setflag("disk", "true");
    log_setflag("ansi", "true");
    
    log!(Levels::Info, "Started logging session");
    log!(Levels::Debug, "This is the last debug message you will see");
    log_setflag("debug", "false");
    log!(Levels::Warning, "This is a warning");
    log!(Levels::Debug, "This will be hided");
    log!(Levels::Error, "{} went wrong", "Something");
    log!(Levels::Fatal, "Ending logging showcase session");
    
    Ok(())
}