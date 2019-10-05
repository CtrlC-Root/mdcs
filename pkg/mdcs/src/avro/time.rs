use std::convert::TryFrom;
use std::time::SystemTime;


// https://avro.apache.org/docs/current/spec.html#Timestamp+%28millisecond+precision%29
pub fn timestamp() -> i64 {
    let system_millis = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Failed to retrieve unix timestamp")
        .as_millis();

    return i64::try_from(system_millis)
        .expect("Failed to convert unix timestamp to signed long");
}
