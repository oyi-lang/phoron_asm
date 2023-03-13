/// Absolute offset from the beginning of the byte stream
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pos(pub u32);

/// Abstract representation of a source file
#[derive(Debug)]
pub struct SourceFile<'a> {
    /// the name of the source file
    pub src_file: &'a str,
    /// the actual raw source code
    pub src: String,
    /// a vector of Pos values for the beginning of each line
    /// in the source code
    pub beginnings: Vec<Pos>,
}

/// Abstract representation of a region of source code
#[derive(Debug, PartialEq, Default)]
pub struct Span {
    pub low: Pos,
    pub high: Pos,
}

impl Span {
    pub fn merg(&self, other: &Span) -> Self {
        Span {
            low: std::cmp::min(self.low, other.low),
            high: std::cmp::max(self.high, other.high),
        }
    }
}

/// convenience structure for a location in the
/// source code
#[derive(Debug)]
pub struct Location {
    pub src_file: String,
    pub line: u32,
    pub col: u32,
}
