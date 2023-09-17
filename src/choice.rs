use std::fmt::Display;

use crate::{comments::Commented, room::RoomId, span::Spanned, story::Story};

use color_eyre::Result;

#[derive(Debug, Clone)]
pub struct Choice {
    pub message: Spanned<String>,
    pub target: Spanned<RoomId>,
}

impl Display for Choice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.target.content.id(), self.message.content)
    }
}

impl Commented<Choice> {
    pub fn apply(self, story: &mut Story) -> Result<()> {
        story.room = self.value.target;
        Ok(())
    }
}
