use color_eyre::eyre::eyre;

use crate::*;
use std::{
    collections::{btree_map::Entry, BTreeMap},
    ops::Index,
};

#[derive(Debug)]
pub struct Story {
    pub rooms: BTreeMap<RoomId, Room>,
    pub default: Room,
    pub room: Spanned<RoomId>,
}

impl Default for Story {
    fn default() -> Self {
        Self {
            rooms: Default::default(),
            default: Default::default(),
            room: Spanned {
                span: Span::dummy("<void>".into()),
                content: RoomId::new("You forgot to set up an initial room"),
            },
        }
    }
}

impl Story {
    pub fn create_room(&mut self, id: Spanned<RoomId>, room: Room) -> Result<()> {
        let Spanned { span, content: id } = id;
        match self.rooms.entry(id) {
            Entry::Occupied(o) => {
                bail!(
                    "{span}: previous room with id `{:?}` found: {:?}",
                    o.key(),
                    o.get()
                )
            }
            Entry::Vacant(e) => {
                e.insert(room);
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
}

impl Index<&RoomId> for Story {
    type Output = Room;

    fn index(&self, index: &RoomId) -> &Self::Output {
        self.rooms.get(index).unwrap_or(&self.default)
    }
}
