use log::{Level, LevelFilter};
use std::io::Write;

use crate::utils::time::custom_timezone_with_fromat;

pub fn init() {
    env_logger::Builder::new()
        .format(|buf, record| {
            std::env::set_var("RUST_BACKTRACE", "1");
            let formatted_time = custom_timezone_with_fromat();
            let level = record.level();
            let target = record.target();
            let args = record.args();

            // Apply color based on log level
            let level_color = match level {
                Level::Error => "\x1b[31m", // Red
                Level::Warn => "\x1b[33m",  // Yellow
                Level::Info => "\x1b[32m",  // Green
                Level::Debug => "\x1b[34m", // Blue
                Level::Trace => "\x1b[37m", // White
            };

            let reset_color = "\x1b[0m"; // Reset color

            // Format the log message with colors
            let message = format!(
                "[{}] {}[{}]{} - {} - {}",
                formatted_time, // Timestamp with WIB timezone
                level_color,    // Apply color for the log level
                level,          // Log level
                reset_color,    // Reset the color
                target,         // Log target (e.g., module)
                args            // The actual log message
            );

            // Write the formatted message with a newline
            writeln!(buf, "{}", message)
        })
        .filter_level(LevelFilter::Info) // Adjust level as needed
        .parse_filters(&std::env::var("RUST_LOG").unwrap_or_default()) // Override dengan RUST_LOG
        .init();
}
