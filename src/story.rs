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
}

impl Index<&RoomId> for Story {
    type Output = Room;

    fn index(&self, index: &RoomId) -> &Self::Output {
        self.rooms.get(index).unwrap_or(&self.default)
    }
}
