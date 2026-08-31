mod logger;
use std::fs::File;
use crate::logger::{internal_log, log_setoutfile, Levels};

fn main() -> std::io::Result<()> {
    let outputfile = File::create("log.txt")?;
    log_setoutfile(outputfile);
    
    log!(Levels::Debug, "Welcome{}", ", formatting!");
    log!(Levels::Info, "Welcome");
    log!(Levels::Warning, "Welcome");
    log!(Levels::Error, "Welcome");
    log!(Levels::Fatal, "Welcome");
    
    Ok(())
}