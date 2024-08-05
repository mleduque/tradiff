
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

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ExplicitTraEntry {
    pub value: WeiduString,
    pub sound: Option<String>,
    pub alt_value: Option<WeiduString>,
    pub alt_sound: Option<String>,
}

// Only used to build test expected values
#[cfg(test)]
impl ExplicitTraEntry {
    pub fn simplest(value: &str) -> Self {
        ExplicitTraEntry { value: WeiduString::Literal(value.to_string()), ..Default::default() }
    }

    pub fn with_female(value: &str, alt_value: &str) -> Self {
        ExplicitTraEntry {
            value: WeiduString::Literal(value.to_string()),
            alt_value: Some(WeiduString::Literal(alt_value.to_string())),
            ..Default::default()
        }
    }

    pub fn with_sound(value: &str, sound: &str) -> Self {
        ExplicitTraEntry {
            value: WeiduString::Literal(value.to_string()),
            sound: Some(sound.to_string()),
            ..Default::default()
        }
    }

    pub fn new(value: &str, sound: Option<&str>, alt_value: Option<&str>, alt_sound: Option<&str>) -> Self {
        ExplicitTraEntry {
            value: WeiduString::Literal(value.to_string()),
            sound: sound.map(|s| s.to_string()),
            alt_value: alt_value.map(|s| WeiduString::Literal(s.to_string())),
            alt_sound: alt_sound.map(|s|s.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TraComment {
    EndOfLine(String),
    Enclosed(String),
}


#[derive(Debug, Clone, PartialEq)]
pub enum WeiduString {
    Literal(String),
    At(i64),
    Ref(u32),
    Concat(Box<WeiduString>, Box<String>),
}

impl Default for WeiduString {
    fn default() -> Self {
        Self::Literal("".to_string())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TraFragment {
    Comment(TraComment),
    Entry(TraEntry),
    Error,
}

impl TraFragment {
    pub fn as_entry(&self) -> Option<&TraEntry> {
        match self {
            TraFragment::Entry(entry) => Some(entry),
            _ => None,
        }
    }
}
