use std::fmt::Display;

use color_eyre::{
    eyre::{bail, ensure, eyre},
    Result,
};

pub mod room;
pub mod span;
pub mod story;

use room::*;
use span::*;
use story::*;

pub fn parse(file_content: Spanned<String>) -> Result<Story> {
    let mut lines = file_content.lines("//");
    let mut story = Story::new(lines.next().ok_or_else(|| {
        eyre!("expected at least one line stating the starting room in the story file")
    })?);
    let (line, comment) = lines.next().ok_or_else(|| {
        eyre!(
            "{}expected an empty line after the starting room",
            file_content.span
        )
    })?;
    ensure!(
        line.content.is_empty(),
        "{}expected an empty line after the starting room",
        line.span,
    );
    assert_eq!(comment.0.content, "");

    while let Some((line, comment)) = lines.next() {
        let room = parse_room(line, comment, &mut lines)?;
        story.create_room(room)?;
    }
    Ok(story)
}

fn parse_room<'a>(
    header: Spanned<&str>,
    comment: Comment,
    lines: &mut impl Iterator<Item = (Spanned<&'a str>, Comment)>,
) -> Result<Room> {
    let Some(header) = header.strip_prefix("##") else {
        bail!("{}: room header must start with ##", header.span)
    };
    let id = header.trim_start().map(|id| RoomId::new(id));
    let Some((message, msg_comment)) = lines.next() else {
        bail!("{}: trailing room header at end of file", header.span)
    };
    let mut room = Room::new(comment, id, message, msg_comment);
    while let Some((line, comment)) = lines.next() {
        if line.is_empty() {
            break;
        }
        let Some((next, message)) = line.split_once(":") else {
            bail!(
                "{}: room choices must start with a room name followed by a colon",
                line.span
            )
        };
        room.choices.push((
            message.trim_start().map(Into::into),
            next.map(RoomId::new),
            comment,
        ))
    }

    Ok(room)
}

pub struct Comment(Spanned<String>);

impl std::fmt::Debug for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl Default for Comment {
    fn default() -> Comment {
        Self(Spanned::dummy(String::new()))
    }
}

impl Display for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.0.content.is_empty() {
            for line in self.0.content.lines() {
                writeln!(f, "//{line}")?;
            }
        }
        Ok(())
    }
}
