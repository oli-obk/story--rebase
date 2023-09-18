use std::{fmt::Display, num::NonZeroU8};

use crate::{
    action::{Action, Operator},
    comments::Commented,
    span::Spanned,
    story::Story,
};

use color_eyre::Result;

#[derive(Debug, Clone)]
pub struct Choice {
    pub message: Spanned<String>,
    /// How many times can this action be taken?
    /// None means infinite.
    pub repetitions: Option<Spanned<NonZeroU8>>,
    pub action: Action,
}

impl Display for Choice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(repetitions) = &self.repetitions {
            write!(f, "{{{}}}", repetitions.content)?;
        }
        write!(f, "{}: {}", self.action, self.message.content)
    }
}

impl Commented<Choice> {
    pub fn apply(self, story: &mut Story) -> Result<()> {
        match self.value.action {
            Action::Goto(target) => story.room = target,
            Action::Modify {
                operator,
                amount,
                what,
                room,
            } => {
                let value = story
                    .rooms
                    .get_or_insert_default(room.content)
                    .value
                    .items
                    .get_or_insert_default(what.content);
                match operator.content {
                    Operator::Add => *value += amount.content,
                }
            }
        }
        Ok(())
    }
}
