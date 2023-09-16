use crate::{
    span::{Span, Spanned},
    Comment,
};

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
            Comment::empty("<the abyss>".into()),
            Spanned {
                span: Span::dummy("<the abyss>".into()),
                content: RoomId::new("the abyss"),
            },
            Spanned {
                content: "You fell off the end of the world",
                span: Span::dummy("<the abyss>".into()),
            },
            Comment::empty("<the abyss>".into()),
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
