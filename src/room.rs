use std::fmt::Display;

use crate::{span::Spanned, Comment};

#[derive(Debug)]
pub struct Room {
    pub comment: Comment,
    pub id: Spanned<RoomId>,
    pub message_comment: Comment,
    pub message: Spanned<String>,
    pub choices: Vec<(Spanned<String>, Spanned<RoomId>, Comment)>,
}

impl Default for Room {
    fn default() -> Self {
        Self::new(
            Comment::default(),
            Spanned::dummy(RoomId::new("the abyss")),
            Spanned::dummy("You fell off the end of the world"),
            Comment::default(),
        )
    }
}

impl Room {
    pub fn new(
        comment: Comment,
        id: Spanned<RoomId>,
        message: Spanned<impl Into<String>>,
        message_comment: Comment,
    ) -> Self {
        Self {
            id,
            comment,
            message: message.map(Into::into),
            choices: Default::default(),
            message_comment,
        }
    }
}

impl Display for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Room {
            comment,
            id,
            message,
            choices,
            message_comment,
        } = self;
        writeln!(f, "{comment}## {}", id.content.id())?;
        writeln!(f, "{message_comment}{}", message.content)?;
        for (text, target, comment) in choices {
            writeln!(f, "{comment}{}: {}", target.content.id(), text.content)?;
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
