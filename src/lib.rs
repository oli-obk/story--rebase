use std::{
    fmt::{Debug, Display},
    ops::Deref,
};

use color_eyre::{eyre::bail, Result};

pub mod parsing;
pub mod room;
pub mod span;
pub mod story;

use room::*;
use span::*;

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
