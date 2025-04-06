#[derive(Debug, PartialEq)]
pub enum HandErr {
    InvalidGroup,
    InvalidSuit,
    InvalidShape,
    NoYaku,
    NoHandTiles,
    NoWinTile,
    DuplicateRiichi,
    IppatsuWithoutRiichi,
    DoubleRiichiHaiteiIppatsu,
    DoubleRiichiHaiteiChankan,
    ChankanTsumo,
    RinshanKanWithoutKan,
    RinshanWithoutTsumo,
    RinshanIppatsu,
    NoHan,
    NoFu,
    InvalidTile,
    Err,
}

impl std::fmt::Display for HandErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidGroup => write!(f, "Invalid Group found"),
            Self::InvalidSuit => write!(f, "Invalid Suit found"),
            Self::InvalidShape => write!(f, "Invalid Hand Shape found"),
            Self::NoYaku => write!(f, "No Yaku"),
            Self::NoHandTiles => write!(f, "No Hand Tiles given"),
            Self::NoWinTile => write!(f, "No Win Tile given"),
            Self::DuplicateRiichi => write!(f, "Cant Riichi and Double Riichi Simultaneously"),
            Self::IppatsuWithoutRiichi => write!(f, "Cant Ippatsu without Riichi"),
            Self::ChankanTsumo => write!(f, "Cant Tsumo and Chankan"),
            Self::RinshanKanWithoutKan => write!(f, "Cant Rinshan without Kan"),
            Self::RinshanWithoutTsumo => write!(f, "Cant Rinshan without Tsumo"),
            Self::RinshanIppatsu => write!(f, "Cant Rinshan and Ippatsu"),
            Self::DoubleRiichiHaiteiIppatsu => {
                write!(f, "Cant Double Riichi, Ippatsu and Haitei")
            }
            Self::DoubleRiichiHaiteiChankan => {
                write!(f, "Cant Double Riichi, Haitei and Chankan")
            }
            Self::NoHan => write!(f, "No Han provided!"),
            Self::NoFu => write!(f, "No Fu provided!"),
            Self::InvalidTile => write!(f, "Tile is not valid!"),
            Self::Err => write!(f, "Something went wrong!"),
        }
    }
}
