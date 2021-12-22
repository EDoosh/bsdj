use std::fmt;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Colors {
    Background,
    Highlight,
    Details,
    Cursor,
    TextCursor,
}

impl fmt::Display for Colors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Colors::Background => "background",
                Colors::Highlight => "highlight",
                Colors::Details => "details",
                Colors::Cursor => "cursor",
                Colors::TextCursor => "textcursor",
            }
        )
    }
}
