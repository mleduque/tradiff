
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
    pub fn simplest(value: WeiduStringLit) -> Self {
        ExplicitTraEntry { value: WeiduString::Literal(value), ..Default::default() }
    }

    pub fn with_female(value: WeiduStringLit, alt_value: WeiduStringLit) -> Self {
        ExplicitTraEntry {
            value: WeiduString::Literal(value),
            alt_value: Some(WeiduString::Literal(alt_value)),
            ..Default::default()
        }
    }

    pub fn with_sound(value: WeiduStringLit, sound: &str) -> Self {
        ExplicitTraEntry {
            value: WeiduString::Literal(value),
            sound: Some(sound.to_string()),
            ..Default::default()
        }
    }

    pub fn new(value: WeiduStringLit, sound: Option<&str>, 
            alt_value: Option<WeiduStringLit>, alt_sound: Option<&str>) -> Self {
        ExplicitTraEntry {
            value: WeiduString::Literal(value),
            sound: sound.map(|s| s.to_string()),
            alt_value: alt_value.map(|s| WeiduString::Literal(s)),
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
pub enum WeiduStringLit {
    Tilde(String),
    DoubleQuote(String),
    Percent(String),
    FiveTildes(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum WeiduString {
    Literal(WeiduStringLit),
    At(i64),
    Ref(u32),
    Concat(Box<WeiduString>, Box<WeiduStringLit>),
}

impl Default for WeiduString {
    fn default() -> Self {
        Self::Literal(WeiduStringLit::Tilde("".to_string()))
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

#[macro_export]
macro_rules! tilde {
    ($expression:expr) => {
        WeiduStringLit::Tilde($expression.into())
    };
}

#[macro_export]
macro_rules! dquote {
    ($expression:expr) => {
        WeiduStringLit::DoubleQuote($expression.into())
    };
}

#[macro_export]
macro_rules! percent {
    ($expression:expr) => {
        WeiduStringLit::Percent($expression.into())
    };
}

#[macro_export]
macro_rules! ftildes {
    ($expression:expr) => {
        WeiduStringLit::FiveTildes($expression.into())
    };
}
