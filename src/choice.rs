use std::fmt::Display;

use crate::{room::RoomId, span::Spanned};

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
