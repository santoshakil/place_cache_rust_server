use anyhow::Result;
use log::{info, LevelFilter};
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};
use std::path::PathBuf;

use crate::db::DOCDIR;

pub fn init_log4rs() -> Result<()> {
    let path = PathBuf::from(DOCDIR.read().unwrap().clone());
    let file = path.join("logs.log");
    let file_path = file.as_os_str().to_str().unwrap();

    let p = PatternEncoder::new("{d(%H:%M:%S)} - {l} - {m}\n");
    let stderr = ConsoleAppender::builder().encoder(Box::new(p)).build();

    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {l} - {m}\n")))
        .build(file_path)
        .unwrap();

    let level = LevelFilter::Info; // Trace < Debug < Info < Warn < Error
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(Appender::builder().build("stderr", Box::new(stderr)))
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(level),
        )
        .unwrap();

    if let Err(err) = log4rs::init_config(config) {
        info!("Error: {}", err);
    }

    info!("Application started");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_log4rs() {
        assert!(init_log4rs().is_ok());
    }
}
