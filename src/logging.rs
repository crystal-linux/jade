use crate::uwu;
use flexi_logger::{style, DeferredNow, LogSpecification, Logger};
use lazy_static::lazy_static;
use log::{Level, LevelFilter};
use std::env;
use std::io::Write;

lazy_static! {
    static ref UWU: bool = env::var("JADE_UWU").map(|v| v == "true").unwrap_or(false);
    static ref UWU_DEBUG: bool = env::var("JADE_UWU_DEBUG")
        .map(|v| v == "true")
        .unwrap_or(false);
}

pub fn init(verbosity: usize) {
    let log_specification = match verbosity {
        0 => LogSpecification::builder()
            .default(LevelFilter::Info)
            .build(),
        1 => LogSpecification::builder()
            .default(LevelFilter::Debug)
            .build(),
        _ => LogSpecification::builder()
            .default(LevelFilter::Trace)
            .build(),
    };
    Logger::with(log_specification)
        .format(format_log_entry)
        .start()
        .unwrap();
}

/// Formats a log entry with color
fn format_log_entry(
    w: &mut dyn Write,
    now: &mut DeferredNow,
    record: &log::Record,
) -> std::io::Result<()> {
    let msg = record.args().to_string();
    let level = record.level();
    let msg = apply_uwu(level, msg);
    let (h, m, s) = now.now().time().as_hms();
    write!(
        w,
        "[ {} ] {}:{}:{} {}",
        style(level).paint(level.to_string()),
        h,
        m,
        s,
        msg
    )
}

/// Applies uwu if the required environment variables are set
fn apply_uwu(level: Level, msg: String) -> String {
    match level {
        Level::Error | Level::Warn | Level::Info => {
            if *UWU {
                uwu!(msg)
            } else {
                msg
            }
        }
        Level::Debug | Level::Trace => {
            if *UWU_DEBUG {
                uwu!(msg)
            } else {
                msg
            }
        }
    }
}
