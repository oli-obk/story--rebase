use std::fmt::Display;

use crate::{comments::Commented, span::Spanned};

#[derive(Debug)]
pub struct Room {
    pub id: Spanned<RoomId>,
    pub message: Commented<Spanned<String>>,
    pub choices: Vec<Commented<Choice>>,
}

#[derive(Debug)]
pub struct Choice {
    pub message: Spanned<String>,
    pub target: Spanned<RoomId>,
}

impl Display for Choice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.target.content.id(), self.message.content)
    }
}

impl Default for Room {
    fn default() -> Self {
        Self::new(
            Spanned::dummy(RoomId::new("the abyss")),
            Commented::dummy(Spanned::dummy("You fell off the end of the world")),
        )
    }
}

impl Room {
    pub fn new(id: Spanned<RoomId>, message: Commented<Spanned<impl Into<String>>>) -> Self {
        Self {
            id,
            message: message.map(|message| message.map(Into::into)),
            choices: Default::default(),
        }
    }
}

impl Display for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Room {
            id,
            message,
            choices,
        } = self;
        writeln!(f, "## {}", id.content.id())?;
        writeln!(f, "{}", message.as_ref().map(|message| &message.content))?;
        for choice in choices {
            writeln!(f, "{choice}")?;
        }
        Ok(())
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
    pub fn id(&self) -> &str {
        &self.0
    }
}
