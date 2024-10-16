/// This is the size at which a new file should be created.
const TRIGGER_FILE_SIZE: u64 = 5000 * 1024;

/// Number of archive log files to keep
const LOG_FILE_COUNT: u32 = 10;

use std::path::PathBuf;

use log::{/*debug, error, info, trace, warn,*/ info, LevelFilter, SetLoggerError};
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        rolling_file::policy::compound::{
            roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger, CompoundPolicy,
        },
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};

/// Initialize logger
/// arguments = level 
/// Log level
/// 0 = trace 
/// 1 = debug 
/// 2 = info 
/// 3 = warnings
/// 4 = errors only
/// 5 = off  
pub fn new(loglevel : u8) -> Result<(), SetLoggerError> {
    let level = log::LevelFilter::Info;
    let file_log_level;
    // Decide level level for output
    match loglevel { 
        0 => file_log_level = LevelFilter::Trace, 
        1 => file_log_level = LevelFilter::Debug, 
        2 => file_log_level = LevelFilter::Info, 
        3 => file_log_level = LevelFilter::Warn, 
        4 => file_log_level = LevelFilter::Error, 
        5 => file_log_level = LevelFilter::Off, 
        _ => file_log_level = LevelFilter::Trace,
    }
    // Build file name from APP name (from main.rs)
    let con = format!("{}.log", super::APP_NAME);
    let file_name: &str = con.as_str();
    // Build archive file pattern/name from APP name (from main.rs)
    let con = format!("{}.{{}}.log", super::APP_NAME);
    let archive_file_pattern: &str = con.as_str();
    // Build a stderr logger.
    let stderr = ConsoleAppender::builder().target(Target::Stderr).build();

    let binding = logarchivepath(archive_file_pattern).unwrap();
    let archive_path = binding.to_str().unwrap(); 
    let binding = logfilepath(file_name).unwrap();
    let file_path = binding.to_str().unwrap();     

    // Create a policy to use with the file logging
    let trigger = SizeTrigger::new(TRIGGER_FILE_SIZE);
    let roller = FixedWindowRoller::builder()
        .base(0) // Default Value (line not needed unless you want to change from 0 (only here for demo purposes)
        .build(archive_path, LOG_FILE_COUNT) // Roll based on pattern and max 3 archive files
        .unwrap();
    let policy = CompoundPolicy::new(Box::new(trigger), Box::new(roller));

    // Logging to log file. (with rolling)
    let logfile = log4rs::append::rolling_file::RollingFileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S%.3f)} : {l} - {m} >> {M} \n")))
        .build(file_path, Box::new(policy))
        .unwrap();

    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .build("stderr", Box::new(stderr)),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(file_log_level),
        )
        .unwrap();

    // Use this to change log levels at runtime.
    // This means you can change the default log level to trace
    // if you are trying to debug an issue and need more logs on then turn it off
    // once you are done.
    let _handle = log4rs::init_config(config)?;
    info!(
        "See '{}' for log and 'archive' directory for archived logs",
        file_path
    );
    Ok(())
}

/// Generate OS related path to local config directory
///  
/// Linux   --> $HOME/.local/share/<project_path> 
/// 
/// Windows --> {FOLDERID_LocalAppData}/<project_path>/data
/// 
pub fn logfilepath(filename : &str) -> Option<PathBuf> {
    if let Some(result) = directories::ProjectDirs::from(super::APP_QUALIFER, super::APP_ORGANIZATION,  super::APP_NAME) {
        let mut path = result.data_local_dir().to_path_buf();
        path.push("log");
        if let Ok(_) = std::fs::create_dir_all(&path) {
            path.push(filename);    
            Some(path)
        }
        else {
            None
        }
    }
    else {
        None             
    }
}

/// Generate OS related path to local config directory
///  
/// Linux   --> $HOME/.local/share/<project_path> 
/// 
/// Windows --> {FOLDERID_LocalAppData}/<project_path>/data
/// 
pub fn logarchivepath(filename : &str) -> Option<PathBuf> {
    if let Some(result) = directories::ProjectDirs::from(super::APP_QUALIFER, super::APP_ORGANIZATION,  super::APP_NAME) {
        let mut path = result.data_local_dir().to_path_buf();
        path.push("log");
        path.push("archive");
        if let Ok(_) = std::fs::create_dir_all(&path) {
            path.push(filename);    
            Some(path)
        }
        else {
            None
        }
    }
    else {
        None             
    }
}