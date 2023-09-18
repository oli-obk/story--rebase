use color_eyre::{
    eyre::{bail, ensure, eyre},
    Result,
};

use crate::{
    action::Action,
    choice::Choice,
    comments::Commented,
    room::{Room, RoomId},
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
        room.choices.push(parse_choice(line)?);
    }

    Ok(comment.with(room))
}

fn parse_choice(
    Commented {
        comment,
        value: line,
    }: Commented<Spanned<&str>>,
) -> Result<Commented<Choice>> {
    let (repetitions, line) = if let Some(line) = line.strip_prefix("{") {
        let Some((n, line)) = line.split_once("}") else {
            bail!("{}: repetition marker must end in `}}`", line.span);
        };
        (Some(n.parse()?), line)
    } else {
        (None, line)
    };
    let (action, message) = if let Some(command) = line.strip_prefix("[") {
        let Some((command, rest)) = command.split_once("]") else {
            bail!("{}: commands must be closed with `]`", command.span)
        };
        (parse_action(command)?, rest)
    } else {
        let Some((next, message)) = line.split_once(":") else {
            bail!(
                "{}: room choices must start with a room name followed by a colon",
                line.span
            )
        };
        (Action::Goto(next.map(RoomId::new)), message)
    };

    Ok(comment.with(Choice {
        message: message.trim_start().map(Into::into),
        repetitions,
        action,
    }))
}

fn parse_action(command: Spanned<&str>) -> Result<Action> {
    let Some((room, rest)) = command.split_once(".") else {
        bail!("{}: invalid room to act upon", command.span)
    };
    let room = room.trim();
    let Some((what, rest)) = rest.take_while(|c| c.is_alphanumeric()) else {
        bail!("{}: invalid room content to modify", rest.span);
    };
    panic!("{rest:?}");
    let rest = rest.trim_start();
    let Some((operator, rest)) = rest.take_while(|c| is_operator_sigil(c)) else {
        bail!("{}: need something after operator", rest.span);
    };
    let operator = operator.parse()?;
    let rest = rest.trim_start();
    let amount = rest.parse()?;
    Ok(Action::Modify {
        operator,
        amount,
        what: what.map(Into::into),
        room: room.map(RoomId::new),
    })
}

fn is_operator_sigil(c: char) -> bool {
    matches!(c, '=' | '+' | '-')
}
