use chrono::{offset::FixedOffset, Utc};
use log::{Level, LevelFilter};
use std::io::Write;

pub fn init_logger() {
    env_logger::Builder::new()
        .format(|buf, record| {
            let wib_time = Utc::now().with_timezone(&FixedOffset::east_opt(7 * 3600).unwrap());
            let formatted_time = wib_time.format("%Y-%m-%d %H:%M:%S").to_string();
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
        .filter_level(LevelFilter::Debug) // Adjust level as needed
        .init();
}
