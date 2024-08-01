
#[derive(Debug, Clone, PartialEq)]
pub struct TraEntry {
    pub id: i64,
    pub value: String,
    pub alt_value: Option<String>,
}

impl TraEntry {
    pub fn base(id: i64, value: String) -> Self {
        TraEntry { id, value, alt_value: None }
    }
    pub fn with_alt(id: i64, value: String,alt_value: String) -> Self {
        TraEntry { id, value, alt_value: Some(alt_value) }
    }
}


pub enum TraComment {
    EndOfLine(String),
    Enclosed(String),
}

pub struct EOL();

pub enum TraFragment {
    Comment(TraComment),
    Entry(TraEntry),
}

impl TraFragment {
    pub fn as_entry(&self) -> Option<&TraEntry> {
        match self {
            TraFragment::Comment(_) => None,
            TraFragment::Entry(entry) => Some(entry),
        }
    }
}
