use chrono::{DateTime, FixedOffset, Utc};

pub fn custom_timezone() -> DateTime<FixedOffset> {
    custom_timezone_with_offset(0)
}

pub fn custom_timezone_with_offset(offset_in_hours: i32) -> DateTime<FixedOffset> {
    let offset_in_seconds = offset_in_hours * 3600;
    let offset = FixedOffset::east_opt(offset_in_seconds).unwrap();
    Utc::now().with_timezone(&offset)
}

pub fn custom_timezone_with_fromat() -> String {
    custom_timezone_with_offset(0)
        .format("%Y-%m-%d %H:%M:%S")
        .to_string()
}
