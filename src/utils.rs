use anyhow::{self, Result};
use colored::Colorize;
use env_logger::{Builder, Env};
use log::{Level, Record};
use std::io::Write;

pub fn setup_logger() -> Result<()> {
    let mut builder =
        Builder::from_env(Env::default().default_filter_or("error,revm_minimal_example=info"));

    builder.format(|buf, record: &Record| {
        let timestamp = chrono::Local::now().format("[%H:%M:%S%.3f]");
        let level = match record.level() {
            Level::Trace => "TRACE".cyan(),
            Level::Debug => "DEBUG".magenta(),
            Level::Info => "INFO".green(),
            Level::Warn => "WARN".red(),
            Level::Error => "ERROR".bright_red(),
        };

        writeln!(buf, "{}[{}] {}", timestamp, level, record.args())
    });

    builder.init();

    Ok(())
}
