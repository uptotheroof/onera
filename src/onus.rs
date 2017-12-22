use interrupts::Interrupts;
use sequence::Sequence;
use time::{self, Timestamp};

#[derive(Debug)]
pub struct Onus {
    pub name: String,
    pub created: Timestamp,
    pub visibility: Visibility,
    pub started: Option<Timestamp>,
    pub finished: Option<Timestamp>,
    pub span: Option<Timestamp>,
    pub seq: Vec<Sequence>,
    pub rem: Vec<String>,
    pub int: Vec<Interrupts>,
}

impl Onus {
    pub fn visible(&self) -> bool {
        self.visibility.master
    }

    pub fn set_visibility(&mut self, visibility: bool) {
        self.visibility.master = visibility;
    }

    pub fn interruptions_visible(&self) -> bool {
        self.visibility.interrupt_visibility
    }

    pub fn set_interruption_visibility(&mut self, visibility: bool) {
        self.visibility.interrupt_visibility = visibility;
    }

    pub fn remarks_visible(&self) -> bool {
        self.visibility.remarks_visibility
    }

    pub fn set_remark_visibility(&mut self, visibility: bool) {
        self.visibility.remarks_visibility = visibility;
    }

    pub fn sequence_visible(&self) -> bool {
        self.visibility.sequence_visibility
    }

    pub fn set_sequence_visibility(&mut self, visibility: bool) {
        self.visibility.sequence_visibility = visibility;
    }

    pub fn stats_visible(&self) -> bool {
        self.visibility.statistics_visibility
    }

    pub fn set_stats_visibility(&mut self, visibility: bool) {
        self.visibility.statistics_visibility = visibility;
    }
}

#[derive(Debug)]
pub struct OnusBuilder {
    name: String,
    created: Timestamp,
}

impl OnusBuilder {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self {
            name: name.into(),
            created: time::timestamp(),
        }
    }

    pub fn build(self) -> Onus {
        let Self { name, created } = self;
        Onus {
            name,
            created,
            visibility: Visibility::new(),
            started: None,
            finished: None,
            span: None,
            seq: Vec::new(),
            rem: Vec::new(),
            int: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Visibility {
    master: bool,
    sequence_visibility: bool,
    remarks_visibility: bool,
    interrupt_visibility: bool,
    statistics_visibility: bool,
}

impl Visibility {
    pub fn new() -> Self {
        Self {
            master: true,
            sequence_visibility: true,
            remarks_visibility: true,
            interrupt_visibility: true,
            statistics_visibility: true,
        }
    }
}
