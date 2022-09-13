use std::time::SystemTime;

pub fn now() -> ::prost_types::Timestamp {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    ::prost_types::Timestamp {
        seconds: i64::try_from(now.as_secs()).unwrap(),
        nanos: i32::try_from(now.as_nanos()).unwrap_or(0),
    }
}
