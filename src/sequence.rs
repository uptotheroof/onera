use time::Timestamp;

#[derive(Debug)]
pub struct Sequence {
    pub name: String,
    pub status: String,
    pub started: Option<Timestamp>,
    pub stopped: Option<Timestamp>,
    pub duration: Option<Timestamp>,
}

impl Sequence {
    pub fn new<T: Into<String>>(name: T, status: T) -> Sequence {
        Sequence {
            name: name.into(),
            status: status.into(),
            started: None,
            stopped: None,
            duration: None,
        }
    }
}
