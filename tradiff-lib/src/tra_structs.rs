
#[derive(Debug, Clone, PartialEq)]
pub struct TraEntry {
    pub id: i64,
    pub content: TraEntryContent,

}

#[derive(Debug, Clone, PartialEq)]
pub enum TraEntryContent {
    Explicit(ExplicitTraEntry),
    At(i64),
    Tlk(u32),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExplicitTraEntry {
    pub value: WeiduString,
    pub sound: Option<String>,
    pub alt_value: Option<WeiduString>,
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

#[derive(Debug, Clone, PartialEq)]
pub enum WeiduString {
    Literal(String),
    At(i64),
    Ref(u32),
    Concat(Box<WeiduString>, Box<String>),
}
