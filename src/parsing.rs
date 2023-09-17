use color_eyre::{
    eyre::{bail, ensure, eyre},
    Result,
};

use crate::{
    comments::Commented,
    room::{Choice, Room, RoomId},
    span::Spanned,
    story::Story,
};

pub fn parse(file_content: Spanned<&str>) -> Result<Story> {
    let mut lines = file_content.lines("//");
    let mut story = Story::new(lines.next().ok_or_else(|| {
        eyre!("expected at least one line stating the starting room in the story file")
    })?);
    let line = lines.next().ok_or_else(|| {
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
    assert_eq!(line.comment.text(), "");

    while let Some(line) = lines.next() {
        let room = parse_room(line, &mut lines)?;
        story.create_room(room)?;
    }
    Ok(story)
}

fn parse_room<'a>(
    Commented {
        comment,
        value: header,
    }: Commented<Spanned<&str>>,
    lines: &mut impl Iterator<Item = Commented<Spanned<&'a str>>>,
) -> Result<Commented<Room>> {
    let Some(header) = header.strip_prefix("##") else {
        bail!("{}: room header must start with ##", header.span)
    };
    let id = header.trim_start().map(|id| RoomId::new(id));
    let Some(message) = lines.next() else {
        bail!("{}: trailing room header at end of file", header.span)
    };
    let mut room = Room::new(id, message);
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
        room.choices.push(line.comment.with(Choice {
            message: message.trim_start().map(Into::into),
            target: next.map(RoomId::new),
        }));
    }

    Ok(comment.with(room))
}
