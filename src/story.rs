use color_eyre::eyre::eyre;

use crate::*;
use std::{
    collections::{btree_map::Entry, BTreeMap},
    ops::Index,
};

#[derive(Debug)]
pub struct Story {
    pub rooms: Vec<Room>,
    pub room_by_name: BTreeMap<RoomId, usize>,
    pub default: Room,
    pub room: Spanned<RoomId>,
}

impl std::fmt::Display for Story {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            rooms,
            room_by_name: _,
            default: _,
            room,
        } = self;
        writeln!(f, "// Initial Room")?;
        writeln!(f, "{}", room.content.id())?;

        for room in rooms {
            let Room {
                id,
                message,
                choices,
            } = room;
            writeln!(f, "## {}", id.content.id())?;
            writeln!(f, "{}", message.content)?;
            for (text, target) in choices {
                writeln!(f, "{}: {}", target.content.id(), text.content)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Story {
    pub fn create_room(&mut self, room: Room) -> Result<()> {
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
        for (msg, _next) in &room.choices {
            println!("[{}]", msg.content);
        }
    }

    pub fn choose(&mut self, idx: usize) -> Result<()> {
        let choices = &self[&self.room.content].choices;
        self.room = choices
            .get(idx)
            .ok_or_else(|| {
                eyre!(
                    "chose selection {idx}, but there are only {}",
                    choices.len()
                )
            })?
            .1
            .clone();
        Ok(())
    }

    pub fn new(first_room: Spanned<impl Into<String>>) -> Self {
        Self {
            rooms: Default::default(),
            room_by_name: Default::default(),
            default: Default::default(),
            room: first_room.map(RoomId::new),
        }
    }
}

impl Index<&RoomId> for Story {
    type Output = Room;

    fn index(&self, index: &RoomId) -> &Self::Output {
        self.room_by_name
            .get(index)
            .map(|&i| &self.rooms[i])
            .unwrap_or(&self.default)
    }
}
