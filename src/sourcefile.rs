//! Abstract representation of a physical source file in the file system. The `SourceFile` is
//! used by the lexer for tokenisation, as well as to provide `Span`s over the actual source code
//! for error reporting during parsing and type-checking.

use std::{convert::From, fmt::Debug, fs, io, ops::Sub, path::Path};

/// Absolute offset from the beginning of the byte stream
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pos(usize);

impl Pos {
    pub fn new(p: usize) -> Self {
        Pos(p)
    }
}

impl Sub<Pos> for Pos {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Pos(self.0 - other.0)
    }
}

impl From<Pos> for usize {
    fn from(pos: Pos) -> Self {
        pos.0
    }
}

impl From<usize> for Pos {
    fn from(val: usize) -> Self {
        Pos(val)
    }
}

/// Abstract representation of a source file
#[derive(Debug)]
pub struct SourceFile {
    /// the name of the source file
    pub src_file: String,
    /// the actual raw source code
    pub src: String,
    /// a vector of Pos values for the beginning of each line
    /// in the source code
    pub beginnings: Vec<Pos>,
}

impl SourceFile {
    pub fn new<P>(src_file: P) -> Result<Self, io::Error>
    where
        P: AsRef<Path> + Debug,
    {
        let src = fs::read_to_string(src_file.as_ref())?;

        let mut beginnings = vec![Pos::new(0)];
        beginnings.extend_from_slice(
            &src.match_indices("\n")
                .map(|(idx, _)| Pos::new(idx))
                .collect::<Vec<_>>(),
        );

        Ok(SourceFile {
            src_file: src_file
                .as_ref()
                .to_str()
                .ok_or(io::Error::new(
                    io::ErrorKind::Other,
                    "could not read source file contents",
                ))?
                .to_owned(),
            src,
            beginnings,
        })
    }
}

/// Abstract representation of a region of source code
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Span {
    pub low: Pos,
    pub high: Pos,
}

impl Span {
    pub fn merge(&self, other: &Span) -> Self {
        Span {
            low: std::cmp::min(self.low, other.low),
            high: std::cmp::max(self.high, other.high),
        }
    }

    /// Return the `Location` information corresponding to this span.
    ///     - src_file is obtained from the `Sourcefile` argument,
    ///     - line number is retrieved by performing binary search with span.low
    ///       on `beginnings`.
    ///     - column number is retrieved as span.low - beginnings[line number].
    pub fn location<'s>(&self, source_file: &'s SourceFile) -> Location<'s> {
        let src_file = &source_file.src_file;

        let (line, col) = match source_file.beginnings.binary_search(&self.low) {
            Ok(line) => {
                let line = line + 1;
                let col: usize = (self.low - source_file.beginnings[line - 1]).into();

                (line, col)
            }

            Err(line) => {
                let line = line;
                let col: usize = (self.low - source_file.beginnings[line - 1]).into();

                (line, col)
            }
        };

        Location {
            src_file,
            line,
            col,
        }
    }

    /// Retrieve the snippet of source code corresponding to this span
    ///     - extract the source code from span.low to span.high
    pub fn source<'s>(&self, source_file: &'s SourceFile) -> &'s str {
        &source_file.src[self.low.into()..self.high.into()]
    }

    /// Retrieve the entire line of source code corresponding to this span
    ///     - extract the whole line from the source file.
    pub fn source_line<'s>(&self, source_file: &'s SourceFile) -> &'s str {
        let Location { line, .. } = self.location(&source_file);

        let start_pos = source_file.beginnings[line - 1];
        let end_pos = if line == source_file.beginnings.len() {
            source_file.beginnings[line - 1]
        } else {
            source_file.beginnings[line]
        };

        &source_file.src[start_pos.into()..end_pos.into()]
    }
}

/// convenience structure for a location in the
/// source code
#[derive(Debug)]
pub struct Location<'l> {
    pub src_file: &'l str,
    pub line: usize,
    pub col: usize,
}
