use std::fmt::Display;

use crate::{choice::Choice, comments::Commented, map::SortedMap, span::Spanned};

#[derive(Debug)]
pub struct Room {
    pub id: Spanned<RoomId>,
    pub message: Commented<Spanned<String>>,
    pub choices: Vec<Commented<Choice>>,
    pub items: SortedMap<String, usize>,
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
            items: Default::default(),
        }
    }
}

impl Display for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Room {
            id,
            message,
            choices,
            items,
        } = self;
        writeln!(f, "## {}", id.content.id())?;
        writeln!(f, "{}", message.as_ref().map(|message| &message.content))?;
        for choice in choices {
            writeln!(f, "{choice}")?;
        }
        for (item, amount) in items.iter() {
            writeln!(f, "{item} = {amount}")?;
        }
        Ok(())
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
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
