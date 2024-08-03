
#[derive(Debug, Clone, PartialEq)]
pub struct TraEntry {
    pub id: i64,
    pub value: String,
    pub sound: Option<String>,
    pub alt_value: Option<String>,
    pub alt_sound: Option<String>,
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
