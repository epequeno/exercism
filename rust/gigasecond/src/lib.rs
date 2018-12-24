extern crate chrono;
use chrono::{DateTime, Duration, UTC};


pub fn after(s: DateTime<UTC>) -> DateTime<UTC> {
    s + Duration::seconds(10i64.pow(9))
}
