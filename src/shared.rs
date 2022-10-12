use chrono::{DateTime, Local, TimeZone, Utc};

pub(crate) fn unix_time_ms(unix: u64) -> DateTime<Local> {
    let secs = unix.div_floor(1_000);
    let nanos = (unix % 1_000) * 1_000_000;
    DateTime::<Local>::from(Utc.timestamp(secs as i64, nanos as u32))
}
pub(crate) fn unix_time_sec(unix: u64) -> DateTime<Local> {
    DateTime::<Local>::from(Utc.timestamp(unix as i64, 0))
}
pub(crate) fn parse_rfc3339(str: &str) -> DateTime<Local> {
    DateTime::<Local>::from(DateTime::parse_from_rfc3339(str).unwrap())
}
