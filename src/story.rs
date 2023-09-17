use color_eyre::{
    eyre::{bail, eyre},
    Result,
};

use crate::{
    comments::{Comment, Commented},
    room::{Room, RoomId},
    span::Spanned,
};
use std::{
    collections::{btree_map::Entry, BTreeMap},
    ops::Index,
};

#[derive(Debug)]
pub struct Story {
    pub main_comment: Comment,
    pub rooms: Vec<Commented<Room>>,
    pub room_by_name: BTreeMap<RoomId, usize>,
    pub default: Room,
    pub room: Spanned<RoomId>,
    pub choices: Vec<u8>,
}

impl std::fmt::Display for Story {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            main_comment,
            rooms,
            room_by_name: _,
            default: _,
            room,
            choices: _,
        } = self;

        writeln!(f, "{main_comment}{}", room.content.id())?;

        for room in rooms {
            writeln!(f)?;
            write!(f, "{}", room)?;
        }

        Ok(())
    }
}

impl Story {
    pub fn create_room(&mut self, room: Commented<Room>) -> Result<()> {
        let idx = self.rooms.len();
        let id = room.id.clone();
        self.rooms.push(room);
        let Spanned { span, content: id } = id;
        match self.room_by_name.entry(id) {
            Entry::Occupied(o) => {
                bail!(
                    "{span}: previous room with id `{:?}` found: {:?}",
                    o.key(),
                    self.rooms[*o.get()]
                )
            }
            Entry::Vacant(e) => {
                e.insert(idx);
                Ok(())
            }
        }
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
        self.room = choices
            .get(idx)
            .ok_or_else(|| {
                eyre!(
                    "chose selection {idx}, but there are only {}",
                    choices.len()
                )
            })?
            .target
            .clone();
        Ok(())
    }

    pub fn new(first_room: Commented<Spanned<impl Into<String>>>) -> Self {
        Self {
            main_comment: first_room.comment,
            rooms: Default::default(),
            room_by_name: Default::default(),
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
        self.room_by_name
            .get(index)
            .map(|&i| &self.rooms[i].value)
            .unwrap_or(&self.default)
    }
}
