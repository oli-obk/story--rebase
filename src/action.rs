use crate::{room::RoomId, span::Spanned};
use color_eyre::eyre::bail;
use color_eyre::Report;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Action {
    Goto(Spanned<RoomId>),
    Modify {
        operator: Spanned<Operator>,
        amount: Spanned<usize>,
        what: Spanned<String>,
        room: Spanned<RoomId>,
    },
}

#[derive(Debug, Clone)]
pub enum Operator {
    Add,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Operator::Add => "+=",
        })
    }
}

impl FromStr for Operator {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "+=" => Operator::Add,
            _ => bail!("unknown operator `{s}`"),
        })
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Goto(target) => write!(f, "{}", target.content.id()),
            Action::Modify {
                operator,
                amount,
                what,
                room,
            } => {
                write!(
                    f,
                    "{}.{} {} {}",
                    room.content.id(),
                    what.content,
                    operator.content,
                    amount.content
                )
            }
        }
    }
}
