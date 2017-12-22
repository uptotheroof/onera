use chrono::{DateTime, Utc};

pub type Timestamp = DateTime<Utc>;

pub fn timestamp() -> Timestamp {
    Utc::now()
}
