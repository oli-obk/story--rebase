use color_eyre::{eyre::Context, Result};
use std::{fmt::Display, path::PathBuf};

#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub span: Span,
    pub content: T,
}

#[derive(Clone)]
pub struct Span {
    file: PathBuf,
    line_start: usize,
    line_end: usize,
    col_start: usize,
    col_end: usize,
}

impl std::fmt::Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
impl Span {
    pub fn dummy(file: PathBuf) -> Self {
        Self {
            file,
            line_start: 0,
            line_end: 0,
            col_start: 0,
            col_end: 0,
        }
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            file,
            line_start,
            line_end,
            col_start,
            col_end,
        } = self;
        let file = file.display();
        write!(f, "{file}:{line_start}:{col_start} {line_end}:{col_end}")
    }
}
impl Spanned<&str> {
    pub fn split_once(&self, delimiter: &str) -> Option<(Self, Self)> {
        let (a, b) = self.content.split_once(delimiter)?;
        let n = a.chars().count();
        let mut span = self.span.clone();
        span.col_end = n;
        let a = Spanned { span, content: a };
        let mut span = self.span.clone();
        span.col_start = n + 1;
        let b = Spanned { span, content: b };
        Some((a, b))
    }
    pub fn trim_end(&self) -> Self {
        let content = self.content.trim_end();
        let n = self.content[content.len()..].chars().count();
        let mut span = self.span.clone();
        span.col_end -= n;
        Self { content, span }
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    pub fn strip_prefix(&self, prefix: &str) -> Option<Self> {
        let content = self.content.strip_prefix(prefix)?;
        let n = self.content[..(self.content.len() - content.len())]
            .chars()
            .count();
        let mut span = self.span.clone();
        span.col_start += n;
        Some(Self { content, span })
    }

    pub fn trim_start(&self) -> Self {
        let content = self.content.trim_start();
        let n = self.content[..(self.content.len() - content.len())]
            .chars()
            .count();
        let mut span = self.span.clone();
        span.col_start += n;
        Self { content, span }
    }

    pub fn starts_with(&self, pat: &str) -> bool {
        self.content.starts_with(pat)
    }
}

impl<T> Spanned<T> {
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Spanned<U> {
        let Spanned { content, span } = self;
        let content = f(content);
        Spanned { content, span }
    }
}

impl Spanned<String> {
    pub fn read_from_file(path: impl Into<PathBuf>) -> Result<Self> {
        let path = path.into();
        let path_str = path.display().to_string();
        let story = std::fs::read_to_string(&path).with_context(|| path_str)?;
        let span = Span {
            file: path,
            line_start: 1,
            line_end: story.lines().count() + 1,
            col_start: 0,
            col_end: 0,
        };
        Ok(Self {
            span,
            content: story,
        })
    }
    pub fn lines<'a>(&'a self) -> impl Iterator<Item = Spanned<&'a str>> {
        assert_eq!(self.span.col_start, 0);
        self.content.lines().enumerate().map(|(i, content)| {
            let mut span = self.span.clone();
            span.line_start += i;
            span.line_end = span.line_start;
            span.col_end = content.chars().count();
            Spanned { content, span }
        })
    }
}
