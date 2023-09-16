use color_eyre::{
    eyre::{bail, eyre},
    Result,
};

pub mod room;
pub mod span;
pub mod story;

use room::*;
use span::*;
use story::*;

pub fn parse(story: Spanned<String>) -> Result<Story> {
    let mut lines = story
        .lines()
        .map(|line| line.trim_end())
        // Comments are ignored entirely
        // FIXME: add to `Span` of the next item for debugging purposes?
        .filter(|line| !line.starts_with("//"));
    let mut story = Story::new(
        lines
            .next()
            .ok_or_else(|| eyre!("expected at least one non-comment line in the story file"))?,
    );

    while let Some(line) = lines.next() {
        let (id, room) = parse_room(line, &mut lines)?;
        story.create_room(id, room)?;
    }
    Ok(story)
}

fn parse_room<'a>(
    header: Spanned<&str>,
    lines: &mut impl Iterator<Item = Spanned<&'a str>>,
) -> Result<(Spanned<RoomId>, Room)> {
    let Some(header) = header.strip_prefix("##") else {
        bail!("{}: room header must start with ##", header.span)
    };
    let id = header.trim_start().map(|id| RoomId::new(id));
    let Some(message) = lines.next() else {
        bail!("{}: trailing room header at end of file", header.span)
    };
    let mut room = Room::new(message);
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let Some((next, message)) = line.split_once(":") else {
            bail!(
                "{}: room choices must start with a room name followed by a colon",
                line.span
            )
        };
        room.choices
            .push((message.trim_start().map(Into::into), next.map(RoomId::new)))
    }

    Ok((id, room))
}
