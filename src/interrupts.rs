use time::Timestamp;

#[derive(Debug)]
pub struct Interrupts {
    pub name: String,
    pub status: String,
    pub started: Option<Timestamp>,
    pub stopped: Option<Timestamp>,
    pub duration: Option<Timestamp>,
}

impl Interrupts {
    pub fn new<T: Into<String>>(name: T, status: T) -> Interrupts {
        Interrupts {
            name: name.into(),
            status: status.into(),
            started: None,
            stopped: None,
            duration: None,
        }
    }
}
