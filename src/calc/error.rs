use crate::hand::error::HandErr;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CalcErr {
    HandErr(HandErr),
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
    NoYaku,
    NoHandshapesFound,
}

impl std::fmt::Display for CalcErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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
            Self::NoHandshapesFound => write!(f, "No handshapes found"),
            Self::HandErr(t) => write!(f, "{}", t),
        }
    }
}
