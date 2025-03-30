use crate::{
    hand::error::HandErr, suit::Suit, AKAFIVE_VALUE, EAST_VALUE, EAST_VALUE_Z, EIGHT_VALUE,
    FIVE_VALUE, FOUR_VALUE, GREEN_VALUE, GREEN_VALUE_Z, NINE_VALUE, NORTH_VALUE, NORTH_VALUE_Z,
    ONE_VALUE, RED_VALUE, RED_VALUE_Z, SEVEN_VALUE, SIX_VALUE, SOUTH_VALUE, SOUTH_VALUE_Z,
    THREE_VALUE, TWO_VALUE, WEST_VALUE, WEST_VALUE_Z, WHITE_VALUE, WHITE_VALUE_Z,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tile {
    Man(Man),
    Pin(Pin),
    Sou(Sou),
    Wind(Wind),
    Dragon(Dragon),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Man {
    OneMan,
    TwoMan,
    ThreeMan,
    FourMan,
    FiveMan,
    AkaFiveMan,
    SixMan,
    SevenMan,
    EightMan,
    NineMan,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Pin {
    OnePin,
    TwoPin,
    ThreePin,
    FourPin,
    FivePin,
    AkaFivePin,
    SixPin,
    SevenPin,
    EightPin,
    NinePin,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Sou {
    OneSou,
    TwoSou,
    ThreeSou,
    FourSou,
    FiveSou,
    AkaFiveSou,
    SixSou,
    SevenSou,
    EightSou,
    NineSou,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Wind {
    East,
    South,
    West,
    North,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Dragon {
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

impl Iterator for Tile {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Tile::Man(t) => {
                let next_t = match t {
                    Man::OneMan => Man::TwoMan,
                    Man::TwoMan => Man::ThreeMan,
                    Man::ThreeMan => Man::FourMan,
                    Man::FourMan => Man::FiveMan,
                    Man::FiveMan => Man::SixMan,
                    Man::AkaFiveMan => Man::SixMan,
                    Man::SixMan => Man::SevenMan,
                    Man::SevenMan => Man::EightMan,
                    Man::EightMan => Man::NineMan,
                    Man::NineMan => Man::OneMan,
                };
                *t = next_t.clone();
                Some(Tile::Man(next_t))
            }
            Tile::Pin(t) => {
                let next_t = match t {
                    Pin::OnePin => Pin::TwoPin,
                    Pin::TwoPin => Pin::ThreePin,
                    Pin::ThreePin => Pin::FourPin,
                    Pin::FourPin => Pin::FivePin,
                    Pin::FivePin => Pin::SixPin,
                    Pin::AkaFivePin => Pin::SixPin,
                    Pin::SixPin => Pin::SevenPin,
                    Pin::SevenPin => Pin::EightPin,
                    Pin::EightPin => Pin::NinePin,
                    Pin::NinePin => Pin::OnePin,
                };
                *t = next_t.clone();
                Some(Tile::Pin(next_t))
            }
            Tile::Sou(t) => {
                let next_t = match t {
                    Sou::OneSou => Sou::TwoSou,
                    Sou::TwoSou => Sou::ThreeSou,
                    Sou::ThreeSou => Sou::FourSou,
                    Sou::FourSou => Sou::FiveSou,
                    Sou::FiveSou => Sou::SixSou,
                    Sou::AkaFiveSou => Sou::SixSou,
                    Sou::SixSou => Sou::SevenSou,
                    Sou::SevenSou => Sou::EightSou,
                    Sou::EightSou => Sou::NineSou,
                    Sou::NineSou => Sou::OneSou,
                };
                *t = next_t.clone();
                Some(Tile::Sou(next_t))
            }
            Tile::Wind(t) => {
                let next_t = match t {
                    Wind::East => Wind::South,
                    Wind::South => Wind::West,
                    Wind::West => Wind::North,
                    Wind::North => Wind::East,
                };
                *t = next_t.clone();
                Some(Tile::Wind(next_t))
            }
            Tile::Dragon(t) => {
                let next_t = match t {
                    Dragon::Red => Dragon::White,
                    Dragon::White => Dragon::Green,
                    Dragon::Green => Dragon::Red,
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
                    Ok(Tile::Man(Man::OneMan))
                } else if value == TWO_VALUE {
                    return Ok(Tile::Man(Man::TwoMan));
                } else if value == THREE_VALUE {
                    return Ok(Tile::Man(Man::ThreeMan));
                } else if value == FOUR_VALUE {
                    return Ok(Tile::Man(Man::FourMan));
                } else if value == FIVE_VALUE {
                    if isaka {
                        return Ok(Tile::Man(Man::AkaFiveMan));
                    }
                    return Ok(Tile::Man(Man::FiveMan));
                } else if value == SIX_VALUE {
                    return Ok(Tile::Man(Man::SixMan));
                } else if value == SEVEN_VALUE {
                    return Ok(Tile::Man(Man::SevenMan));
                } else if value == EIGHT_VALUE {
                    return Ok(Tile::Man(Man::EightMan));
                } else if value == NINE_VALUE {
                    return Ok(Tile::Man(Man::NineMan));
                } else {
                    return Err(HandErr::InvalidTile);
                }
            }
            Suit::Pinzu => {
                if value == ONE_VALUE {
                    Ok(Tile::Pin(Pin::OnePin))
                } else if value == TWO_VALUE {
                    return Ok(Tile::Pin(Pin::TwoPin));
                } else if value == THREE_VALUE {
                    return Ok(Tile::Pin(Pin::ThreePin));
                } else if value == FOUR_VALUE {
                    return Ok(Tile::Pin(Pin::FourPin));
                } else if value == FIVE_VALUE {
                    if isaka {
                        return Ok(Tile::Pin(Pin::AkaFivePin));
                    }
                    return Ok(Tile::Pin(Pin::FivePin));
                } else if value == SIX_VALUE {
                    return Ok(Tile::Pin(Pin::SixPin));
                } else if value == SEVEN_VALUE {
                    return Ok(Tile::Pin(Pin::SevenPin));
                } else if value == EIGHT_VALUE {
                    return Ok(Tile::Pin(Pin::EightPin));
                } else if value == NINE_VALUE {
                    return Ok(Tile::Pin(Pin::NinePin));
                } else {
                    return Err(HandErr::InvalidTile);
                }
            }
            Suit::Souzu => {
                if value == ONE_VALUE {
                    Ok(Tile::Sou(Sou::OneSou))
                } else if value == TWO_VALUE {
                    return Ok(Tile::Sou(Sou::TwoSou));
                } else if value == THREE_VALUE {
                    return Ok(Tile::Sou(Sou::ThreeSou));
                } else if value == FOUR_VALUE {
                    return Ok(Tile::Sou(Sou::FourSou));
                } else if value == FIVE_VALUE {
                    if isaka {
                        return Ok(Tile::Sou(Sou::AkaFiveSou));
                    }
                    return Ok(Tile::Sou(Sou::FiveSou));
                } else if value == SIX_VALUE {
                    return Ok(Tile::Sou(Sou::SixSou));
                } else if value == SEVEN_VALUE {
                    return Ok(Tile::Sou(Sou::SevenSou));
                } else if value == EIGHT_VALUE {
                    return Ok(Tile::Sou(Sou::EightSou));
                } else if value == NINE_VALUE {
                    return Ok(Tile::Sou(Sou::NineSou));
                } else {
                    return Err(HandErr::InvalidTile);
                }
            }
            Suit::Wind => {
                if value == WEST_VALUE || value == WEST_VALUE_Z {
                    Ok(Tile::Wind(Wind::West))
                } else if value == EAST_VALUE || value == EAST_VALUE_Z {
                    return Ok(Tile::Wind(Wind::East));
                } else if value == SOUTH_VALUE || value == SOUTH_VALUE_Z {
                    return Ok(Tile::Wind(Wind::South));
                } else if value == NORTH_VALUE || value == NORTH_VALUE_Z {
                    return Ok(Tile::Wind(Wind::North));
                } else {
                    return Err(HandErr::InvalidTile);
                }
            }
            Suit::Dragon => {
                if value == RED_VALUE || value == RED_VALUE_Z {
                    Ok(Tile::Dragon(Dragon::Red))
                } else if value == GREEN_VALUE || value == GREEN_VALUE_Z {
                    return Ok(Tile::Dragon(Dragon::Green));
                } else if value == WHITE_VALUE || value == WHITE_VALUE_Z {
                    return Ok(Tile::Dragon(Dragon::White));
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
        if *self == Tile::Man(Man::AkaFiveMan)
            || *self == Tile::Pin(Pin::AkaFivePin)
            || *self == Tile::Sou(Sou::AkaFiveSou)
        {
            return true;
        }
        false
    }

    pub fn is_terminal(&self) -> bool {
        if *self == Tile::Man(Man::OneMan)
            || *self == Tile::Man(Man::NineMan)
            || *self == Tile::Pin(Pin::OnePin)
            || *self == Tile::Pin(Pin::NinePin)
            || *self == Tile::Sou(Sou::OneSou)
            || *self == Tile::Sou(Sou::NineSou)
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
            Tile::Man(Man::OneMan) => ONE_VALUE,
            Tile::Man(Man::TwoMan) => TWO_VALUE,
            Tile::Man(Man::ThreeMan) => THREE_VALUE,
            Tile::Man(Man::FourMan) => FOUR_VALUE,
            Tile::Man(Man::FiveMan) => FIVE_VALUE,
            Tile::Man(Man::AkaFiveMan) => FIVE_VALUE,
            Tile::Man(Man::SixMan) => SIX_VALUE,
            Tile::Man(Man::SevenMan) => SEVEN_VALUE,
            Tile::Man(Man::EightMan) => EIGHT_VALUE,
            Tile::Man(Man::NineMan) => NINE_VALUE,

            Tile::Sou(Sou::OneSou) => ONE_VALUE,
            Tile::Sou(Sou::TwoSou) => TWO_VALUE,
            Tile::Sou(Sou::ThreeSou) => THREE_VALUE,
            Tile::Sou(Sou::FourSou) => FOUR_VALUE,
            Tile::Sou(Sou::FiveSou) => FIVE_VALUE,
            Tile::Sou(Sou::AkaFiveSou) => FIVE_VALUE,
            Tile::Sou(Sou::SixSou) => SIX_VALUE,
            Tile::Sou(Sou::SevenSou) => SEVEN_VALUE,
            Tile::Sou(Sou::EightSou) => EIGHT_VALUE,
            Tile::Sou(Sou::NineSou) => NINE_VALUE,

            Tile::Pin(Pin::OnePin) => ONE_VALUE,
            Tile::Pin(Pin::TwoPin) => TWO_VALUE,
            Tile::Pin(Pin::ThreePin) => THREE_VALUE,
            Tile::Pin(Pin::FourPin) => FOUR_VALUE,
            Tile::Pin(Pin::FivePin) => FIVE_VALUE,
            Tile::Pin(Pin::AkaFivePin) => FIVE_VALUE,
            Tile::Pin(Pin::SixPin) => SIX_VALUE,
            Tile::Pin(Pin::SevenPin) => SEVEN_VALUE,
            Tile::Pin(Pin::EightPin) => EIGHT_VALUE,
            Tile::Pin(Pin::NinePin) => NINE_VALUE,

            Tile::Wind(Wind::East) => EAST_VALUE,
            Tile::Wind(Wind::South) => SOUTH_VALUE,
            Tile::Wind(Wind::West) => WEST_VALUE,
            Tile::Wind(Wind::North) => NORTH_VALUE,

            Tile::Dragon(Dragon::Red) => RED_VALUE,
            Tile::Dragon(Dragon::Green) => GREEN_VALUE,
            Tile::Dragon(Dragon::White) => WHITE_VALUE,
        }
    }

    /// Parse the group value into an integer.
    pub fn parse_u8(&self) -> Result<u8, std::num::ParseIntError> {
        self.value().to_string().parse()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        AKAFIVE_VALUE, EAST_VALUE, EIGHT_VALUE, FIVE_VALUE, FOUR_VALUE, GREEN_VALUE, NINE_VALUE,
        NORTH_VALUE, ONE_VALUE, RED_VALUE, SEVEN_VALUE, SIX_VALUE, SOUTH_VALUE, THREE_VALUE,
        TWO_VALUE, WEST_VALUE, WHITE_VALUE,
    };

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
        assert_eq!(tile, Tile::Dragon(Dragon::Red));
        let tile = Tile::new(SEVEN_VALUE, &Suit::Dragon).unwrap();
        assert_eq!(tile, Tile::Dragon(Dragon::Red));
        let tile: Tile = "rd".to_string().try_into().unwrap();
        assert_eq!(tile, Tile::Dragon(Dragon::Red));
        let tile: Tile = "7z".to_string().try_into().unwrap();
        assert_eq!(tile, Tile::Dragon(Dragon::Red));

        let tile = Tile::new(GREEN_VALUE, &Suit::Dragon).unwrap();
        assert_eq!(tile, Tile::Dragon(Dragon::Green));
        let tile = Tile::new(SIX_VALUE, &Suit::Dragon).unwrap();
        assert_eq!(tile, Tile::Dragon(Dragon::Green));
        let tile: Tile = "gd".to_string().try_into().unwrap();
        assert_eq!(tile, Tile::Dragon(Dragon::Green));
        let tile: Tile = "6z".to_string().try_into().unwrap();
        assert_eq!(tile, Tile::Dragon(Dragon::Green));

        let tile = Tile::new(WHITE_VALUE, &Suit::Dragon).unwrap();
        assert_eq!(tile, Tile::Dragon(Dragon::White));
        let tile = Tile::new(FIVE_VALUE, &Suit::Dragon).unwrap();
        assert_eq!(tile, Tile::Dragon(Dragon::White));
        let tile: Tile = "wd".to_string().try_into().unwrap();
        assert_eq!(tile, Tile::Dragon(Dragon::White));
        let tile: Tile = "5z".to_string().try_into().unwrap();
        assert_eq!(tile, Tile::Dragon(Dragon::White));
    }

    #[test]
    fn get_wind_tile() {
        let tile = Tile::new(EAST_VALUE, &Suit::Wind).unwrap();
        assert_eq!(tile, Tile::Wind(Wind::East));
        let tile = Tile::new(ONE_VALUE, &Suit::Wind).unwrap();
        assert_eq!(tile, Tile::Wind(Wind::East));

        let tile = Tile::new(WEST_VALUE, &Suit::Wind).unwrap();
        assert_eq!(tile, Tile::Wind(Wind::West));
        let tile = Tile::new(THREE_VALUE, &Suit::Wind).unwrap();
        assert_eq!(tile, Tile::Wind(Wind::West));

        let tile = Tile::new(SOUTH_VALUE, &Suit::Wind).unwrap();
        assert_eq!(tile, Tile::Wind(Wind::South));
        let tile = Tile::new(TWO_VALUE, &Suit::Wind).unwrap();
        assert_eq!(tile, Tile::Wind(Wind::South));

        let tile = Tile::new(NORTH_VALUE, &Suit::Wind).unwrap();
        assert_eq!(tile, Tile::Wind(Wind::North));
        let tile = Tile::new(FOUR_VALUE, &Suit::Wind).unwrap();
        assert_eq!(tile, Tile::Wind(Wind::North));
    }

    #[test]
    fn get_man_tile() {
        let tile = Tile::new(ONE_VALUE, &Suit::Manzu).unwrap();
        assert_eq!(tile, Tile::Man(Man::OneMan));
        let tile = Tile::new(TWO_VALUE, &Suit::Manzu).unwrap();
        assert_eq!(tile, Tile::Man(Man::TwoMan));
        let tile = Tile::new(THREE_VALUE, &Suit::Manzu).unwrap();
        assert_eq!(tile, Tile::Man(Man::ThreeMan));
        let tile = Tile::new(FOUR_VALUE, &Suit::Manzu).unwrap();
        assert_eq!(tile, Tile::Man(Man::FourMan));
        let tile = Tile::new(FIVE_VALUE, &Suit::Manzu).unwrap();
        assert_eq!(tile, Tile::Man(Man::FiveMan));
        let tile = Tile::new(SIX_VALUE, &Suit::Manzu).unwrap();
        assert_eq!(tile, Tile::Man(Man::SixMan));
        let tile = Tile::new(SEVEN_VALUE, &Suit::Manzu).unwrap();
        assert_eq!(tile, Tile::Man(Man::SevenMan));
        let tile = Tile::new(EIGHT_VALUE, &Suit::Manzu).unwrap();
        assert_eq!(tile, Tile::Man(Man::EightMan));
        let tile = Tile::new(NINE_VALUE, &Suit::Manzu).unwrap();
        assert_eq!(tile, Tile::Man(Man::NineMan));
        let tile = Tile::new(AKAFIVE_VALUE, &Suit::Manzu).unwrap();
        assert_eq!(tile, Tile::Man(Man::AkaFiveMan));
    }

    #[test]
    fn get_pin_tile() {
        let tile = Tile::new(ONE_VALUE, &Suit::Pinzu).unwrap();
        assert_eq!(tile, Tile::Pin(Pin::OnePin));
        let tile = Tile::new(TWO_VALUE, &Suit::Pinzu).unwrap();
        assert_eq!(tile, Tile::Pin(Pin::TwoPin));
        let tile = Tile::new(THREE_VALUE, &Suit::Pinzu).unwrap();
        assert_eq!(tile, Tile::Pin(Pin::ThreePin));
        let tile = Tile::new(FOUR_VALUE, &Suit::Pinzu).unwrap();
        assert_eq!(tile, Tile::Pin(Pin::FourPin));
        let tile = Tile::new(FIVE_VALUE, &Suit::Pinzu).unwrap();
        assert_eq!(tile, Tile::Pin(Pin::FivePin));
        let tile = Tile::new(SIX_VALUE, &Suit::Pinzu).unwrap();
        assert_eq!(tile, Tile::Pin(Pin::SixPin));
        let tile = Tile::new(SEVEN_VALUE, &Suit::Pinzu).unwrap();
        assert_eq!(tile, Tile::Pin(Pin::SevenPin));
        let tile = Tile::new(EIGHT_VALUE, &Suit::Pinzu).unwrap();
        assert_eq!(tile, Tile::Pin(Pin::EightPin));
        let tile = Tile::new(NINE_VALUE, &Suit::Pinzu).unwrap();
        assert_eq!(tile, Tile::Pin(Pin::NinePin));
        let tile = Tile::new(AKAFIVE_VALUE, &Suit::Pinzu).unwrap();
        assert_eq!(tile, Tile::Pin(Pin::AkaFivePin));
    }

    #[test]
    fn get_sou_tile() {
        let tile = Tile::new(ONE_VALUE, &Suit::Souzu).unwrap();
        assert_eq!(tile, Tile::Sou(Sou::OneSou));
        let tile = Tile::new(TWO_VALUE, &Suit::Souzu).unwrap();
        assert_eq!(tile, Tile::Sou(Sou::TwoSou));
        let tile = Tile::new(THREE_VALUE, &Suit::Souzu).unwrap();
        assert_eq!(tile, Tile::Sou(Sou::ThreeSou));
        let tile = Tile::new(FOUR_VALUE, &Suit::Souzu).unwrap();
        assert_eq!(tile, Tile::Sou(Sou::FourSou));
        let tile = Tile::new(FIVE_VALUE, &Suit::Souzu).unwrap();
        assert_eq!(tile, Tile::Sou(Sou::FiveSou));
        let tile = Tile::new(SIX_VALUE, &Suit::Souzu).unwrap();
        assert_eq!(tile, Tile::Sou(Sou::SixSou));
        let tile = Tile::new(SEVEN_VALUE, &Suit::Souzu).unwrap();
        assert_eq!(tile, Tile::Sou(Sou::SevenSou));
        let tile = Tile::new(EIGHT_VALUE, &Suit::Souzu).unwrap();
        assert_eq!(tile, Tile::Sou(Sou::EightSou));
        let tile = Tile::new(NINE_VALUE, &Suit::Souzu).unwrap();
        assert_eq!(tile, Tile::Sou(Sou::NineSou));
        let tile = Tile::new(AKAFIVE_VALUE, &Suit::Souzu).unwrap();
        assert_eq!(tile, Tile::Sou(Sou::AkaFiveSou));
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
