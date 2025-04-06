use crate::suit::*;
use crate::{hand::error::HandErr, suit::Suit};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tile {
    Man(MpsValue),
    Pin(MpsValue),
    Sou(MpsValue),
    Wind(WValue),
    Dragon(DValue),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MpsValue {
    One,
    Two,
    Three,
    Four,
    Five,
    AkaFive,
    Six,
    Seven,
    Eight,
    Nine,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum WValue {
    East,
    South,
    West,
    North,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DValue {
    Red,
    Green,
    White,
}

impl TryFrom<String> for Tile {
    type Error = HandErr;

    fn try_from(tile_string: String) -> Result<Self, Self::Error> {
        if tile_string.len() != 2 {
            return Err(HandErr::InvalidTile);
        }
        let value_char = tile_string.chars().nth(0).unwrap();
        let suit_char = tile_string.chars().nth(1).unwrap();

        let suit = Suit::suit_from_string(suit_char, value_char)?;
        Tile::new(value_char, &suit)
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Man(t) => match t {
                MpsValue::One => write!(f, "{}{}", ONE_VALUE, MAN_SUIT_CHAR),
                MpsValue::Two => write!(f, "{}{}", TWO_VALUE, MAN_SUIT_CHAR),
                MpsValue::Three => write!(f, "{}{}", THREE_VALUE, MAN_SUIT_CHAR),
                MpsValue::Four => write!(f, "{}{}", FOUR_VALUE, MAN_SUIT_CHAR),
                MpsValue::Five => write!(f, "{}{}", FIVE_VALUE, MAN_SUIT_CHAR),
                MpsValue::AkaFive => write!(f, "{}{}", AKAFIVE_VALUE, MAN_SUIT_CHAR), //MMMMMM choices choic,
                MpsValue::Six => write!(f, "{}{}", SIX_VALUE, MAN_SUIT_CHAR),
                MpsValue::Seven => write!(f, "{}{}", SEVEN_VALUE, MAN_SUIT_CHAR),
                MpsValue::Eight => write!(f, "{}{}", EIGHT_VALUE, MAN_SUIT_CHAR),
                MpsValue::Nine => write!(f, "{}{}", NINE_VALUE, MAN_SUIT_CHAR),
            },
            Tile::Pin(t) => match t {
                MpsValue::One => write!(f, "{}{}", ONE_VALUE, PIN_SUIT_CHAR),
                MpsValue::Two => write!(f, "{}{}", TWO_VALUE, PIN_SUIT_CHAR),
                MpsValue::Three => write!(f, "{}{}", THREE_VALUE, PIN_SUIT_CHAR),
                MpsValue::Four => write!(f, "{}{}", FOUR_VALUE, PIN_SUIT_CHAR),
                MpsValue::Five => write!(f, "{}{}", FIVE_VALUE, PIN_SUIT_CHAR),
                MpsValue::AkaFive => write!(f, "{}{}", AKAFIVE_VALUE, PIN_SUIT_CHAR), //MMMMMM choices choic,
                MpsValue::Six => write!(f, "{}{}", SIX_VALUE, PIN_SUIT_CHAR),
                MpsValue::Seven => write!(f, "{}{}", SEVEN_VALUE, PIN_SUIT_CHAR),
                MpsValue::Eight => write!(f, "{}{}", EIGHT_VALUE, PIN_SUIT_CHAR),
                MpsValue::Nine => write!(f, "{}{}", NINE_VALUE, PIN_SUIT_CHAR),
            },
            Tile::Sou(t) => match t {
                MpsValue::One => write!(f, "{}{}", ONE_VALUE, SOU_SUIT_CHAR),
                MpsValue::Two => write!(f, "{}{}", TWO_VALUE, SOU_SUIT_CHAR),
                MpsValue::Three => write!(f, "{}{}", THREE_VALUE, SOU_SUIT_CHAR),
                MpsValue::Four => write!(f, "{}{}", FOUR_VALUE, SOU_SUIT_CHAR),
                MpsValue::Five => write!(f, "{}{}", FIVE_VALUE, SOU_SUIT_CHAR),
                MpsValue::AkaFive => write!(f, "{}{}", AKAFIVE_VALUE, SOU_SUIT_CHAR), //MMMMMM choices choic,
                MpsValue::Six => write!(f, "{}{}", SIX_VALUE, SOU_SUIT_CHAR),
                MpsValue::Seven => write!(f, "{}{}", SEVEN_VALUE, SOU_SUIT_CHAR),
                MpsValue::Eight => write!(f, "{}{}", EIGHT_VALUE, SOU_SUIT_CHAR),
                MpsValue::Nine => write!(f, "{}{}", NINE_VALUE, SOU_SUIT_CHAR),
            },
            Tile::Wind(t) => match t {
                WValue::East => write!(f, "{}{}", EAST_VALUE, WIND_SUIT_CHAR),
                WValue::South => write!(f, "{}{}", SOUTH_VALUE, WIND_SUIT_CHAR),
                WValue::West => write!(f, "{}{}", WEST_VALUE, WIND_SUIT_CHAR),
                WValue::North => write!(f, "{}{}", NORTH_VALUE, WIND_SUIT_CHAR),
            },
            Tile::Dragon(t) => match t {
                DValue::Red => write!(f, "{}{}", RED_VALUE, DRAGON_SUIT_CHAR),
                DValue::Green => write!(f, "{}{}", GREEN_VALUE, DRAGON_SUIT_CHAR),
                DValue::White => write!(f, "{}{}", WHITE_VALUE, DRAGON_SUIT_CHAR),
            },
        }
    }
}

impl Iterator for Tile {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Tile::Man(t) => {
                let next_t = match t {
                    MpsValue::One => MpsValue::Two,
                    MpsValue::Two => MpsValue::Three,
                    MpsValue::Three => MpsValue::Four,
                    MpsValue::Four => MpsValue::Five,
                    MpsValue::Five => MpsValue::Six,
                    MpsValue::AkaFive => MpsValue::Six,
                    MpsValue::Six => MpsValue::Seven,
                    MpsValue::Seven => MpsValue::Eight,
                    MpsValue::Eight => MpsValue::Nine,
                    MpsValue::Nine => MpsValue::One,
                };
                *t = next_t.clone();
                Some(Tile::Man(next_t))
            }
            Tile::Pin(t) => {
                let next_t = match t {
                    MpsValue::One => MpsValue::Two,
                    MpsValue::Two => MpsValue::Three,
                    MpsValue::Three => MpsValue::Four,
                    MpsValue::Four => MpsValue::Five,
                    MpsValue::Five => MpsValue::Six,
                    MpsValue::AkaFive => MpsValue::Six,
                    MpsValue::Six => MpsValue::Seven,
                    MpsValue::Seven => MpsValue::Eight,
                    MpsValue::Eight => MpsValue::Nine,
                    MpsValue::Nine => MpsValue::One,
                };
                *t = next_t.clone();
                Some(Tile::Pin(next_t))
            }
            Tile::Sou(t) => {
                let next_t = match t {
                    MpsValue::One => MpsValue::Two,
                    MpsValue::Two => MpsValue::Three,
                    MpsValue::Three => MpsValue::Four,
                    MpsValue::Four => MpsValue::Five,
                    MpsValue::Five => MpsValue::Six,
                    MpsValue::AkaFive => MpsValue::Six,
                    MpsValue::Six => MpsValue::Seven,
                    MpsValue::Seven => MpsValue::Eight,
                    MpsValue::Eight => MpsValue::Nine,
                    MpsValue::Nine => MpsValue::One,
                };
                *t = next_t.clone();
                Some(Tile::Sou(next_t))
            }
            Tile::Wind(t) => {
                let next_t = match t {
                    WValue::East => WValue::South,
                    WValue::South => WValue::West,
                    WValue::West => WValue::North,
                    WValue::North => WValue::East,
                };
                *t = next_t.clone();
                Some(Tile::Wind(next_t))
            }
            Tile::Dragon(t) => {
                let next_t = match t {
                    DValue::Red => DValue::White,
                    DValue::White => DValue::Green,
                    DValue::Green => DValue::Red,
                };
                *t = next_t.clone();
                Some(Tile::Dragon(next_t))
            }
        }
    }
}

impl Tile {
    pub fn new(value: char, suit: &Suit) -> Result<Tile, HandErr> {
        let mut isaka = false;
        let value = if value == AKAFIVE_VALUE {
            isaka = true;
            FIVE_VALUE
        } else {
            value
        };

        match suit {
            Suit::Manzu => {
                if value == ONE_VALUE {
                    Ok(Tile::Man(MpsValue::One))
                } else if value == TWO_VALUE {
                    return Ok(Tile::Man(MpsValue::Two));
                } else if value == THREE_VALUE {
                    return Ok(Tile::Man(MpsValue::Three));
                } else if value == FOUR_VALUE {
                    return Ok(Tile::Man(MpsValue::Four));
                } else if value == FIVE_VALUE {
                    if isaka {
                        return Ok(Tile::Man(MpsValue::AkaFive));
                    }
                    return Ok(Tile::Man(MpsValue::Five));
                } else if value == SIX_VALUE {
                    return Ok(Tile::Man(MpsValue::Six));
                } else if value == SEVEN_VALUE {
                    return Ok(Tile::Man(MpsValue::Seven));
                } else if value == EIGHT_VALUE {
                    return Ok(Tile::Man(MpsValue::Eight));
                } else if value == NINE_VALUE {
                    return Ok(Tile::Man(MpsValue::Nine));
                } else {
                    return Err(HandErr::InvalidTile);
                }
            }
            Suit::Pinzu => {
                if value == ONE_VALUE {
                    Ok(Tile::Pin(MpsValue::One))
                } else if value == TWO_VALUE {
                    return Ok(Tile::Pin(MpsValue::Two));
                } else if value == THREE_VALUE {
                    return Ok(Tile::Pin(MpsValue::Three));
                } else if value == FOUR_VALUE {
                    return Ok(Tile::Pin(MpsValue::Four));
                } else if value == FIVE_VALUE {
                    if isaka {
                        return Ok(Tile::Pin(MpsValue::AkaFive));
                    }
                    return Ok(Tile::Pin(MpsValue::Five));
                } else if value == SIX_VALUE {
                    return Ok(Tile::Pin(MpsValue::Six));
                } else if value == SEVEN_VALUE {
                    return Ok(Tile::Pin(MpsValue::Seven));
                } else if value == EIGHT_VALUE {
                    return Ok(Tile::Pin(MpsValue::Eight));
                } else if value == NINE_VALUE {
                    return Ok(Tile::Pin(MpsValue::Nine));
                } else {
                    return Err(HandErr::InvalidTile);
                }
            }
            Suit::Souzu => {
                if value == ONE_VALUE {
                    Ok(Tile::Sou(MpsValue::One))
                } else if value == TWO_VALUE {
                    return Ok(Tile::Sou(MpsValue::Two));
                } else if value == THREE_VALUE {
                    return Ok(Tile::Sou(MpsValue::Three));
                } else if value == FOUR_VALUE {
                    return Ok(Tile::Sou(MpsValue::Four));
                } else if value == FIVE_VALUE {
                    if isaka {
                        return Ok(Tile::Sou(MpsValue::AkaFive));
                    }
                    return Ok(Tile::Sou(MpsValue::Five));
                } else if value == SIX_VALUE {
                    return Ok(Tile::Sou(MpsValue::Six));
                } else if value == SEVEN_VALUE {
                    return Ok(Tile::Sou(MpsValue::Seven));
                } else if value == EIGHT_VALUE {
                    return Ok(Tile::Sou(MpsValue::Eight));
                } else if value == NINE_VALUE {
                    return Ok(Tile::Sou(MpsValue::Nine));
                } else {
                    return Err(HandErr::InvalidTile);
                }
            }
            Suit::Wind => {
                if value == WEST_VALUE || value == WEST_VALUE_Z {
                    Ok(Tile::Wind(WValue::West))
                } else if value == EAST_VALUE || value == EAST_VALUE_Z {
                    return Ok(Tile::Wind(WValue::East));
                } else if value == SOUTH_VALUE || value == SOUTH_VALUE_Z {
                    return Ok(Tile::Wind(WValue::South));
                } else if value == NORTH_VALUE || value == NORTH_VALUE_Z {
                    return Ok(Tile::Wind(WValue::North));
                } else {
                    return Err(HandErr::InvalidTile);
                }
            }
            Suit::Dragon => {
                if value == RED_VALUE || value == RED_VALUE_Z {
                    Ok(Tile::Dragon(DValue::Red))
                } else if value == GREEN_VALUE || value == GREEN_VALUE_Z {
                    return Ok(Tile::Dragon(DValue::Green));
                } else if value == WHITE_VALUE || value == WHITE_VALUE_Z {
                    return Ok(Tile::Dragon(DValue::White));
                } else {
                    return Err(HandErr::InvalidTile);
                }
            }
        }
    }

    pub fn suit(&self) -> Suit {
        match self {
            Tile::Man(_) => Suit::Manzu,
            Tile::Pin(_) => Suit::Pinzu,
            Tile::Sou(_) => Suit::Souzu,
            Tile::Wind(_) => Suit::Wind,
            Tile::Dragon(_) => Suit::Dragon,
        }
    }

    pub fn is_aka(&self) -> bool {
        if *self == Tile::Man(MpsValue::AkaFive)
            || *self == Tile::Pin(MpsValue::AkaFive)
            || *self == Tile::Sou(MpsValue::AkaFive)
        {
            return true;
        }
        false
    }

    pub fn is_terminal(&self) -> bool {
        if *self == Tile::Man(MpsValue::One)
            || *self == Tile::Man(MpsValue::Nine)
            || *self == Tile::Pin(MpsValue::One)
            || *self == Tile::Pin(MpsValue::Nine)
            || *self == Tile::Sou(MpsValue::One)
            || *self == Tile::Sou(MpsValue::Nine)
        {
            return true;
        }
        false
    }

    pub fn is_honor(&self) -> bool {
        matches!(self, Tile::Dragon(_) | Tile::Wind(_))
    }

    pub fn value(&self) -> char {
        match self {
            Tile::Man(MpsValue::One) => ONE_VALUE,
            Tile::Man(MpsValue::Two) => TWO_VALUE,
            Tile::Man(MpsValue::Three) => THREE_VALUE,
            Tile::Man(MpsValue::Four) => FOUR_VALUE,
            Tile::Man(MpsValue::Five) => FIVE_VALUE,
            Tile::Man(MpsValue::AkaFive) => FIVE_VALUE,
            Tile::Man(MpsValue::Six) => SIX_VALUE,
            Tile::Man(MpsValue::Seven) => SEVEN_VALUE,
            Tile::Man(MpsValue::Eight) => EIGHT_VALUE,
            Tile::Man(MpsValue::Nine) => NINE_VALUE,

            Tile::Sou(MpsValue::One) => ONE_VALUE,
            Tile::Sou(MpsValue::Two) => TWO_VALUE,
            Tile::Sou(MpsValue::Three) => THREE_VALUE,
            Tile::Sou(MpsValue::Four) => FOUR_VALUE,
            Tile::Sou(MpsValue::Five) => FIVE_VALUE,
            Tile::Sou(MpsValue::AkaFive) => FIVE_VALUE,
            Tile::Sou(MpsValue::Six) => SIX_VALUE,
            Tile::Sou(MpsValue::Seven) => SEVEN_VALUE,
            Tile::Sou(MpsValue::Eight) => EIGHT_VALUE,
            Tile::Sou(MpsValue::Nine) => NINE_VALUE,

            Tile::Pin(MpsValue::One) => ONE_VALUE,
            Tile::Pin(MpsValue::Two) => TWO_VALUE,
            Tile::Pin(MpsValue::Three) => THREE_VALUE,
            Tile::Pin(MpsValue::Four) => FOUR_VALUE,
            Tile::Pin(MpsValue::Five) => FIVE_VALUE,
            Tile::Pin(MpsValue::AkaFive) => FIVE_VALUE,
            Tile::Pin(MpsValue::Six) => SIX_VALUE,
            Tile::Pin(MpsValue::Seven) => SEVEN_VALUE,
            Tile::Pin(MpsValue::Eight) => EIGHT_VALUE,
            Tile::Pin(MpsValue::Nine) => NINE_VALUE,

            Tile::Wind(WValue::East) => EAST_VALUE,
            Tile::Wind(WValue::South) => SOUTH_VALUE,
            Tile::Wind(WValue::West) => WEST_VALUE,
            Tile::Wind(WValue::North) => NORTH_VALUE,

            Tile::Dragon(DValue::Red) => RED_VALUE,
            Tile::Dragon(DValue::Green) => GREEN_VALUE,
            Tile::Dragon(DValue::White) => WHITE_VALUE,
        }
    }

    /// Parse the group value into an integer.
    pub fn parse_u8(&self) -> Result<u8, HandErr> {
        // self.value().to_string().parse();

        match self {
            Tile::Man(t) | Tile::Pin(t) | Tile::Sou(t) => match t {
                MpsValue::One => Ok(1),
                MpsValue::Two => Ok(2),
                MpsValue::Three => Ok(3),
                MpsValue::Four => Ok(4),
                MpsValue::Five => Ok(5),
                MpsValue::AkaFive => Ok(5),
                MpsValue::Six => Ok(6),
                MpsValue::Seven => Ok(7),
                MpsValue::Eight => Ok(8),
                MpsValue::Nine => Ok(9),
            },
            _ => Err(HandErr::Err),
        }
    }

    pub fn get_emoji(&self) -> &str {
        match self {
            Tile::Man(t) => match t {
                MpsValue::One => "🀇",
                MpsValue::Two => "🀈",
                MpsValue::Three => "🀉",
                MpsValue::Four => "🀊",
                MpsValue::Five => "🀋",
                MpsValue::AkaFive => "🀋", //MMMMMM choices choices
                MpsValue::Six => "🀌",
                MpsValue::Seven => "🀍",
                MpsValue::Eight => "🀎",
                MpsValue::Nine => "🀏",
            },
            Tile::Pin(t) => match t {
                MpsValue::One => "🀙",
                MpsValue::Two => "🀚",
                MpsValue::Three => "🀛",
                MpsValue::Four => "🀜",
                MpsValue::Five => "🀝",
                MpsValue::AkaFive => "🀝",
                MpsValue::Six => "🀞",
                MpsValue::Seven => "🀟",
                MpsValue::Eight => "🀠",
                MpsValue::Nine => "🀡",
            },
            Tile::Sou(t) => match t {
                MpsValue::One => "🀐",
                MpsValue::Two => "🀑",
                MpsValue::Three => "🀒",
                MpsValue::Four => "🀓",
                MpsValue::Five => "🀔",
                MpsValue::AkaFive => "🀔",
                MpsValue::Six => "🀕",
                MpsValue::Seven => "🀖",
                MpsValue::Eight => "🀗",
                MpsValue::Nine => "🀘",
            },
            Tile::Wind(t) => match t {
                WValue::East => "🀀",
                WValue::South => "🀁",
                WValue::West => "🀂",
                WValue::North => "🀃",
            },
            Tile::Dragon(t) => match t {
                DValue::Red => "🀄",
                DValue::Green => "🀅",
                DValue::White => "🀆",
            },
        }
    }
}

pub const EAST_VALUE: char = 'E';
pub const SOUTH_VALUE: char = 'S';
pub const WEST_VALUE: char = 'W';
pub const NORTH_VALUE: char = 'N';

pub const EAST_VALUE_Z: char = '1';
pub const SOUTH_VALUE_Z: char = '2';
pub const WEST_VALUE_Z: char = '3';
pub const NORTH_VALUE_Z: char = '4';

pub const RED_VALUE: char = 'r';
pub const GREEN_VALUE: char = 'g';
pub const WHITE_VALUE: char = 'w';

pub const RED_VALUE_Z: char = '7';
pub const GREEN_VALUE_Z: char = '6';
pub const WHITE_VALUE_Z: char = '5';

pub const ONE_VALUE: char = '1';
pub const TWO_VALUE: char = '2';
pub const THREE_VALUE: char = '3';
pub const FOUR_VALUE: char = '4';
pub const FIVE_VALUE: char = '5';
pub const AKAFIVE_VALUE: char = '0';
pub const SIX_VALUE: char = '6';
pub const SEVEN_VALUE: char = '7';
pub const EIGHT_VALUE: char = '8';
pub const NINE_VALUE: char = '9';

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_aka() {
        let tile = Tile::new(AKAFIVE_VALUE, &Suit::Souzu).unwrap();
        assert!(tile.is_aka());

        let tile = Tile::new(AKAFIVE_VALUE, &Suit::Pinzu).unwrap();
        assert!(tile.is_aka());

        let tile = Tile::new(AKAFIVE_VALUE, &Suit::Manzu).unwrap();
        assert!(tile.is_aka());

        let tile = Tile::new(FIVE_VALUE, &Suit::Manzu).unwrap();
        assert!(!tile.is_aka());
    }

    #[test]
    fn get_dragon_tile() {
        let tile = Tile::new(RED_VALUE, &Suit::Dragon).unwrap();
        assert_eq!(tile, Tile::Dragon(DValue::Red));
        let tile = Tile::new(SEVEN_VALUE, &Suit::Dragon).unwrap();
        assert_eq!(tile, Tile::Dragon(DValue::Red));
        let tile: Tile = "rd".to_string().try_into().unwrap();
        assert_eq!(tile, Tile::Dragon(DValue::Red));
        let tile: Tile = "7z".to_string().try_into().unwrap();
        assert_eq!(tile, Tile::Dragon(DValue::Red));

        let tile = Tile::new(GREEN_VALUE, &Suit::Dragon).unwrap();
        assert_eq!(tile, Tile::Dragon(DValue::Green));
        let tile = Tile::new(SIX_VALUE, &Suit::Dragon).unwrap();
        assert_eq!(tile, Tile::Dragon(DValue::Green));
        let tile: Tile = "gd".to_string().try_into().unwrap();
        assert_eq!(tile, Tile::Dragon(DValue::Green));
        let tile: Tile = "6z".to_string().try_into().unwrap();
        assert_eq!(tile, Tile::Dragon(DValue::Green));

        let tile = Tile::new(WHITE_VALUE, &Suit::Dragon).unwrap();
        assert_eq!(tile, Tile::Dragon(DValue::White));
        let tile = Tile::new(FIVE_VALUE, &Suit::Dragon).unwrap();
        assert_eq!(tile, Tile::Dragon(DValue::White));
        let tile: Tile = "wd".to_string().try_into().unwrap();
        assert_eq!(tile, Tile::Dragon(DValue::White));
        let tile: Tile = "5z".to_string().try_into().unwrap();
        assert_eq!(tile, Tile::Dragon(DValue::White));
    }

    #[test]
    fn get_wind_tile() {
        let tile = Tile::new(EAST_VALUE, &Suit::Wind).unwrap();
        assert_eq!(tile, Tile::Wind(WValue::East));
        let tile = Tile::new(ONE_VALUE, &Suit::Wind).unwrap();
        assert_eq!(tile, Tile::Wind(WValue::East));

        let tile = Tile::new(WEST_VALUE, &Suit::Wind).unwrap();
        assert_eq!(tile, Tile::Wind(WValue::West));
        let tile = Tile::new(THREE_VALUE, &Suit::Wind).unwrap();
        assert_eq!(tile, Tile::Wind(WValue::West));

        let tile = Tile::new(SOUTH_VALUE, &Suit::Wind).unwrap();
        assert_eq!(tile, Tile::Wind(WValue::South));
        let tile = Tile::new(TWO_VALUE, &Suit::Wind).unwrap();
        assert_eq!(tile, Tile::Wind(WValue::South));

        let tile = Tile::new(NORTH_VALUE, &Suit::Wind).unwrap();
        assert_eq!(tile, Tile::Wind(WValue::North));
        let tile = Tile::new(FOUR_VALUE, &Suit::Wind).unwrap();
        assert_eq!(tile, Tile::Wind(WValue::North));
    }

    #[test]
    fn get_man_tile() {
        let tile = Tile::new(ONE_VALUE, &Suit::Manzu).unwrap();
        assert_eq!(tile, Tile::Man(MpsValue::One));
        let tile = Tile::new(TWO_VALUE, &Suit::Manzu).unwrap();
        assert_eq!(tile, Tile::Man(MpsValue::Two));
        let tile = Tile::new(THREE_VALUE, &Suit::Manzu).unwrap();
        assert_eq!(tile, Tile::Man(MpsValue::Three));
        let tile = Tile::new(FOUR_VALUE, &Suit::Manzu).unwrap();
        assert_eq!(tile, Tile::Man(MpsValue::Four));
        let tile = Tile::new(FIVE_VALUE, &Suit::Manzu).unwrap();
        assert_eq!(tile, Tile::Man(MpsValue::Five));
        let tile = Tile::new(SIX_VALUE, &Suit::Manzu).unwrap();
        assert_eq!(tile, Tile::Man(MpsValue::Six));
        let tile = Tile::new(SEVEN_VALUE, &Suit::Manzu).unwrap();
        assert_eq!(tile, Tile::Man(MpsValue::Seven));
        let tile = Tile::new(EIGHT_VALUE, &Suit::Manzu).unwrap();
        assert_eq!(tile, Tile::Man(MpsValue::Eight));
        let tile = Tile::new(NINE_VALUE, &Suit::Manzu).unwrap();
        assert_eq!(tile, Tile::Man(MpsValue::Nine));
        let tile = Tile::new(AKAFIVE_VALUE, &Suit::Manzu).unwrap();
        assert_eq!(tile, Tile::Man(MpsValue::AkaFive));
    }

    #[test]
    fn get_pin_tile() {
        let tile = Tile::new(ONE_VALUE, &Suit::Pinzu).unwrap();
        assert_eq!(tile, Tile::Pin(MpsValue::One));
        let tile = Tile::new(TWO_VALUE, &Suit::Pinzu).unwrap();
        assert_eq!(tile, Tile::Pin(MpsValue::Two));
        let tile = Tile::new(THREE_VALUE, &Suit::Pinzu).unwrap();
        assert_eq!(tile, Tile::Pin(MpsValue::Three));
        let tile = Tile::new(FOUR_VALUE, &Suit::Pinzu).unwrap();
        assert_eq!(tile, Tile::Pin(MpsValue::Four));
        let tile = Tile::new(FIVE_VALUE, &Suit::Pinzu).unwrap();
        assert_eq!(tile, Tile::Pin(MpsValue::Five));
        let tile = Tile::new(SIX_VALUE, &Suit::Pinzu).unwrap();
        assert_eq!(tile, Tile::Pin(MpsValue::Six));
        let tile = Tile::new(SEVEN_VALUE, &Suit::Pinzu).unwrap();
        assert_eq!(tile, Tile::Pin(MpsValue::Seven));
        let tile = Tile::new(EIGHT_VALUE, &Suit::Pinzu).unwrap();
        assert_eq!(tile, Tile::Pin(MpsValue::Eight));
        let tile = Tile::new(NINE_VALUE, &Suit::Pinzu).unwrap();
        assert_eq!(tile, Tile::Pin(MpsValue::Nine));
        let tile = Tile::new(AKAFIVE_VALUE, &Suit::Pinzu).unwrap();
        assert_eq!(tile, Tile::Pin(MpsValue::AkaFive));
    }

    #[test]
    fn get_sou_tile() {
        let tile = Tile::new(ONE_VALUE, &Suit::Souzu).unwrap();
        assert_eq!(tile, Tile::Sou(MpsValue::One));
        let tile = Tile::new(TWO_VALUE, &Suit::Souzu).unwrap();
        assert_eq!(tile, Tile::Sou(MpsValue::Two));
        let tile = Tile::new(THREE_VALUE, &Suit::Souzu).unwrap();
        assert_eq!(tile, Tile::Sou(MpsValue::Three));
        let tile = Tile::new(FOUR_VALUE, &Suit::Souzu).unwrap();
        assert_eq!(tile, Tile::Sou(MpsValue::Four));
        let tile = Tile::new(FIVE_VALUE, &Suit::Souzu).unwrap();
        assert_eq!(tile, Tile::Sou(MpsValue::Five));
        let tile = Tile::new(SIX_VALUE, &Suit::Souzu).unwrap();
        assert_eq!(tile, Tile::Sou(MpsValue::Six));
        let tile = Tile::new(SEVEN_VALUE, &Suit::Souzu).unwrap();
        assert_eq!(tile, Tile::Sou(MpsValue::Seven));
        let tile = Tile::new(EIGHT_VALUE, &Suit::Souzu).unwrap();
        assert_eq!(tile, Tile::Sou(MpsValue::Eight));
        let tile = Tile::new(NINE_VALUE, &Suit::Souzu).unwrap();
        assert_eq!(tile, Tile::Sou(MpsValue::Nine));
        let tile = Tile::new(AKAFIVE_VALUE, &Suit::Souzu).unwrap();
        assert_eq!(tile, Tile::Sou(MpsValue::AkaFive));
    }

    #[test]
    fn next_dragon() {
        let mut tile = Tile::try_from("wd".to_string()).unwrap();
        tile.next().unwrap();
        assert_eq!(tile.value(), GREEN_VALUE);
        assert_eq!(tile.suit(), Suit::Dragon);

        let mut tile = Tile::try_from("gd".to_string()).unwrap();
        tile.next().unwrap();
        assert_eq!(tile.value(), RED_VALUE);

        let mut tile = Tile::try_from("rd".to_string()).unwrap();
        tile.next().unwrap();
        assert_eq!(tile.value(), WHITE_VALUE);
    }
    #[test]
    fn next_wind() {
        let mut tile = Tile::try_from("Ew".to_string()).unwrap();
        tile.next().unwrap();
        assert_eq!(tile.value(), SOUTH_VALUE);
        assert_eq!(tile.suit(), Suit::Wind);

        let mut tile = Tile::try_from("Sw".to_string()).unwrap();
        tile.next().unwrap();
        assert_eq!(tile.value(), WEST_VALUE);

        let mut tile = Tile::try_from("Ww".to_string()).unwrap();
        tile.next().unwrap();
        assert_eq!(tile.value(), NORTH_VALUE);

        let mut tile = Tile::try_from("Nw".to_string()).unwrap();
        tile.next().unwrap();
        assert_eq!(tile.value(), EAST_VALUE);
    }

    #[test]
    fn next_manpinsou() {
        let mut tile = Tile::try_from("1m".to_string()).unwrap();
        tile.next().unwrap();
        assert_eq!(tile.value(), TWO_VALUE);
        assert_eq!(tile.suit(), Suit::Manzu);

        let mut tile = Tile::try_from("9m".to_string()).unwrap();
        tile.next().unwrap();
        assert_eq!(tile.value(), ONE_VALUE);

        let mut tile = Tile::try_from("0m".to_string()).unwrap();
        tile.next().unwrap();
        assert_eq!(tile.value(), SIX_VALUE);
    }
}
