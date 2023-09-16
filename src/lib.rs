use std::{
    fmt::{Debug, Display},
    ops::Deref,
};

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
    assert_eq!(line.comment.0.content, "");

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

#[derive(Clone)]
pub struct Comment(Spanned<String>);

impl Comment {
    fn with<T>(self, value: T) -> Commented<T> {
        Commented {
            comment: self,
            value,
        }
    }
}

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

#[derive(Debug)]
pub struct Commented<T> {
    pub comment: Comment,
    pub value: T,
}

impl<T> Deref for Commented<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> Commented<T> {
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Commented<U> {
        let Commented { value, comment } = self;
        let value = f(value);
        Commented { value, comment }
    }

    pub fn as_ref(&self) -> Commented<&T> {
        Commented {
            comment: self.comment.clone(),
            value: &self.value,
        }
    }

    fn dummy(value: T) -> Commented<T> {
        Self {
            comment: Comment::default(),
            value,
        }
    }
}

impl<T: Display> Display for Commented<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { comment, value } = self;
        write!(f, "{comment}{value}")
    }
}
