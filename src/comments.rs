use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Deref;

use crate::span::Spanned;

#[derive(Clone)]
pub struct Comment(Spanned<String>);

impl Comment {
    pub fn with<T>(self, value: T) -> Commented<T> {
        Commented {
            comment: self,
            value,
        }
    }

    pub fn text(&self) -> &str {
        &self.0.content
    }

    pub fn new(text: Spanned<String>) -> Self {
        Self(text)
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

#[derive(Debug, Clone)]
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

    pub(crate) fn dummy(value: T) -> Commented<T> {
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
