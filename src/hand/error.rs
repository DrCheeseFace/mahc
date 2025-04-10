#[derive(Debug, PartialEq, Clone, Copy)]
pub enum HandErr {
    InvalidGroup,
    InvalidSuit,
    InvalidShape,
    InvalidTile,
    ParseErr,
}

impl std::fmt::Display for HandErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidGroup => write!(f, "Invalid Group found"),
            Self::InvalidSuit => write!(f, "Invalid Suit found"),
            Self::InvalidShape => write!(f, "Invalid Hand Shape found"),
            Self::InvalidTile => write!(f, "Tile is not valid!"),
            Self::ParseErr => write!(f, "Failed to parse value of tile to u8! This is probably due to calling this method on a non MPS suit"),
        }
    }
}
