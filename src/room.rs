use crate::span::{Span, Spanned};

#[derive(Debug)]
pub struct Room {
    pub message: Spanned<String>,
    pub choices: Vec<(Spanned<String>, Spanned<RoomId>)>,
}

impl Default for Room {
    fn default() -> Self {
        Self::new(Spanned {
            content: "You fell off the end of the world",
            span: Span::dummy("<the abyss>".into()),
        })
    }
}

impl Room {
    pub fn new(message: Spanned<impl Into<String>>) -> Self {
        Self {
            message: message.map(Into::into),
            choices: Default::default(),
        }
    }
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone)]
pub struct RoomId(String);

impl std::fmt::Debug for RoomId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.0)
    }
}

impl RoomId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}
