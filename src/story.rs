use color_eyre::{eyre::eyre, Result};

use crate::{
    choice::Choice,
    comments::{Comment, Commented},
    map::SortedMap,
    room::{Room, RoomId},
    span::Spanned,
};
use std::ops::Index;

#[derive(Debug)]
pub struct Story {
    pub main_comment: Comment,
    pub rooms: SortedMap<RoomId, Commented<Room>>,
    pub default: Room,
    pub room: Spanned<RoomId>,
    pub choices: Vec<u8>,
}

impl std::fmt::Display for Story {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            main_comment,
            rooms,
            default: _,
            room,
            choices: _,
        } = self;

        writeln!(f, "{main_comment}{}", room.content.id())?;

        for room in rooms.values() {
            writeln!(f)?;
            write!(f, "{}", room)?;
        }

        Ok(())
    }
}

impl Story {
    pub fn create_room(&mut self, room: Commented<Room>) -> Result<()> {
        self.rooms.insert(room.id.content.clone(), room)
    }

    pub fn print_room(&self) {
        let room = &self[&self.room.content];
        println!("{}", room.message.content);
        for choice in &room.choices {
            println!("[{}]", choice.value.message.content);
        }
    }

    pub fn choose(&mut self, idx: usize) -> Result<()> {
        self.choices.push(idx.try_into()?);
        let choices = &self[&self.room.content].choices;
        let choice: Commented<Choice> = choices
            .get(idx)
            .ok_or_else(|| {
                eyre!(
                    "chose selection {idx}, but there are only {}",
                    choices.len()
                )
            })?
            .clone();
        choice.apply(self)
    }

    pub fn new(first_room: Commented<Spanned<impl Into<String>>>) -> Self {
        Self {
            main_comment: first_room.comment,
            rooms: Default::default(),
            default: Default::default(),
            room: first_room.value.map(RoomId::new),
            choices: Default::default(),
        }
    }

    pub fn room(&self) -> &Room {
        &self[&self.room.content]
    }
}

impl Index<&RoomId> for Story {
    type Output = Room;

    fn index(&self, index: &RoomId) -> &Self::Output {
        self.rooms
            .get(index)
            .map(|r| &r.value)
            .unwrap_or(&self.default)
    }
}
