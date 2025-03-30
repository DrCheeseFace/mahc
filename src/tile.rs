use crate::{hand::error::HandErr, suit::Suit};

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
        let value_string = tile_string.chars().nth(0).unwrap().to_string();
        let suit_string = tile_string.chars().nth(1).unwrap().to_string();

        let suit = Suit::suit_from_string(&suit_string, &value_string)?;
        Tile::new(&value_string, &suit)
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
    pub fn new(value: &str, suit: &Suit) -> Result<Tile, HandErr> {
        let mut isaka = false;
        let value = if value == "0" {
            isaka = true;
            "5".to_string()
        } else {
            value.to_string()
        };

        match suit {
            Suit::Manzu => {
                if value == "1" {
                    Ok(Tile::Man(Man::OneMan))
                } else if value == "2" {
                    return Ok(Tile::Man(Man::TwoMan));
                } else if value == "3" {
                    return Ok(Tile::Man(Man::ThreeMan));
                } else if value == "4" {
                    return Ok(Tile::Man(Man::FourMan));
                } else if value == "5" {
                    if isaka {
                        return Ok(Tile::Man(Man::AkaFiveMan));
                    }
                    return Ok(Tile::Man(Man::FiveMan));
                } else if value == "6" {
                    return Ok(Tile::Man(Man::SixMan));
                } else if value == "7" {
                    return Ok(Tile::Man(Man::SevenMan));
                } else if value == "8" {
                    return Ok(Tile::Man(Man::EightMan));
                } else if value == "9" {
                    return Ok(Tile::Man(Man::NineMan));
                } else {
                    return Err(HandErr::InvalidTile);
                }
            }
            Suit::Pinzu => {
                if value == "1" {
                    Ok(Tile::Pin(Pin::OnePin))
                } else if value == "2" {
                    return Ok(Tile::Pin(Pin::TwoPin));
                } else if value == "3" {
                    return Ok(Tile::Pin(Pin::ThreePin));
                } else if value == "4" {
                    return Ok(Tile::Pin(Pin::FourPin));
                } else if value == "5" {
                    if isaka {
                        return Ok(Tile::Pin(Pin::AkaFivePin));
                    }
                    return Ok(Tile::Pin(Pin::FivePin));
                } else if value == "6" {
                    return Ok(Tile::Pin(Pin::SixPin));
                } else if value == "7" {
                    return Ok(Tile::Pin(Pin::SevenPin));
                } else if value == "8" {
                    return Ok(Tile::Pin(Pin::EightPin));
                } else if value == "9" {
                    return Ok(Tile::Pin(Pin::NinePin));
                } else {
                    return Err(HandErr::InvalidTile);
                }
            }
            Suit::Souzu => {
                if value == "1" {
                    Ok(Tile::Sou(Sou::OneSou))
                } else if value == "2" {
                    return Ok(Tile::Sou(Sou::TwoSou));
                } else if value == "3" {
                    return Ok(Tile::Sou(Sou::ThreeSou));
                } else if value == "4" {
                    return Ok(Tile::Sou(Sou::FourSou));
                } else if value == "5" {
                    if isaka {
                        return Ok(Tile::Sou(Sou::AkaFiveSou));
                    }
                    return Ok(Tile::Sou(Sou::FiveSou));
                } else if value == "6" {
                    return Ok(Tile::Sou(Sou::SixSou));
                } else if value == "7" {
                    return Ok(Tile::Sou(Sou::SevenSou));
                } else if value == "8" {
                    return Ok(Tile::Sou(Sou::EightSou));
                } else if value == "9" {
                    return Ok(Tile::Sou(Sou::NineSou));
                } else {
                    return Err(HandErr::InvalidTile);
                }
            }
            Suit::Wind => {
                if value == "W" || value == "3" {
                    Ok(Tile::Wind(Wind::West))
                } else if value == "E" || value == "1" {
                    return Ok(Tile::Wind(Wind::East));
                } else if value == "S" || value == "2" {
                    return Ok(Tile::Wind(Wind::South));
                } else if value == "N" || value == "4" {
                    return Ok(Tile::Wind(Wind::North));
                } else {
                    return Err(HandErr::InvalidTile);
                }
            }
            Suit::Dragon => {
                if value == "r" || value == "7" {
                    Ok(Tile::Dragon(Dragon::Red))
                } else if value == "g" || value == "6" {
                    return Ok(Tile::Dragon(Dragon::Green));
                } else if value == "w" || value == "5" {
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

    pub fn value(&self) -> &str {
        match self {
            Tile::Man(Man::OneMan) => "1",
            Tile::Man(Man::TwoMan) => "2",
            Tile::Man(Man::ThreeMan) => "3",
            Tile::Man(Man::FourMan) => "4",
            Tile::Man(Man::FiveMan) => "5",
            Tile::Man(Man::AkaFiveMan) => "5",
            Tile::Man(Man::SixMan) => "6",
            Tile::Man(Man::SevenMan) => "7",
            Tile::Man(Man::EightMan) => "8",
            Tile::Man(Man::NineMan) => "9",

            Tile::Sou(Sou::OneSou) => "1",
            Tile::Sou(Sou::TwoSou) => "2",
            Tile::Sou(Sou::ThreeSou) => "3",
            Tile::Sou(Sou::FourSou) => "4",
            Tile::Sou(Sou::FiveSou) => "5",
            Tile::Sou(Sou::AkaFiveSou) => "5",
            Tile::Sou(Sou::SixSou) => "6",
            Tile::Sou(Sou::SevenSou) => "7",
            Tile::Sou(Sou::EightSou) => "8",
            Tile::Sou(Sou::NineSou) => "9",

            Tile::Pin(Pin::OnePin) => "1",
            Tile::Pin(Pin::TwoPin) => "2",
            Tile::Pin(Pin::ThreePin) => "3",
            Tile::Pin(Pin::FourPin) => "4",
            Tile::Pin(Pin::FivePin) => "5",
            Tile::Pin(Pin::AkaFivePin) => "5",
            Tile::Pin(Pin::SixPin) => "6",
            Tile::Pin(Pin::SevenPin) => "7",
            Tile::Pin(Pin::EightPin) => "8",
            Tile::Pin(Pin::NinePin) => "9",

            Tile::Wind(Wind::East) => "E",
            Tile::Wind(Wind::South) => "S",
            Tile::Wind(Wind::West) => "W",
            Tile::Wind(Wind::North) => "N",

            Tile::Dragon(Dragon::Red) => "r",
            Tile::Dragon(Dragon::Green) => "g",
            Tile::Dragon(Dragon::White) => "w",
        }
    }

    /// Parse the group value into an integer.
    pub fn parse_u8(&self) -> Result<u8, std::num::ParseIntError> {
        self.value().parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_aka() {
        let tile = Tile::new("0", &Suit::Souzu).unwrap();
        assert!(tile.is_aka());

        let tile = Tile::new("0", &Suit::Pinzu).unwrap();
        assert!(tile.is_aka());

        let tile = Tile::new("0", &Suit::Manzu).unwrap();
        assert!(tile.is_aka());

        let tile = Tile::new("5", &Suit::Manzu).unwrap();
        assert!(!tile.is_aka());
    }

    #[test]
    fn get_dragon_tile() {
        let tile = Tile::new("r", &Suit::Dragon).unwrap();
        assert_eq!(tile, Tile::Dragon(Dragon::Red));
        let tile = Tile::new("7", &Suit::Dragon).unwrap();
        assert_eq!(tile, Tile::Dragon(Dragon::Red));
        let tile: Tile = "rd".to_string().try_into().unwrap();
        assert_eq!(tile, Tile::Dragon(Dragon::Red));
        let tile: Tile = "7z".to_string().try_into().unwrap();
        assert_eq!(tile, Tile::Dragon(Dragon::Red));

        let tile = Tile::new("g", &Suit::Dragon).unwrap();
        assert_eq!(tile, Tile::Dragon(Dragon::Green));
        let tile = Tile::new("6", &Suit::Dragon).unwrap();
        assert_eq!(tile, Tile::Dragon(Dragon::Green));
        let tile: Tile = "gd".to_string().try_into().unwrap();
        assert_eq!(tile, Tile::Dragon(Dragon::Green));
        let tile: Tile = "6z".to_string().try_into().unwrap();
        assert_eq!(tile, Tile::Dragon(Dragon::Green));

        let tile = Tile::new("w", &Suit::Dragon).unwrap();
        assert_eq!(tile, Tile::Dragon(Dragon::White));
        let tile = Tile::new("5", &Suit::Dragon).unwrap();
        assert_eq!(tile, Tile::Dragon(Dragon::White));
        let tile: Tile = "wd".to_string().try_into().unwrap();
        assert_eq!(tile, Tile::Dragon(Dragon::White));
        let tile: Tile = "5z".to_string().try_into().unwrap();
        assert_eq!(tile, Tile::Dragon(Dragon::White));
    }

    #[test]
    fn get_wind_tile() {
        let tile = Tile::new("E", &Suit::Wind).unwrap();
        assert_eq!(tile, Tile::Wind(Wind::East));
        let tile = Tile::new("1", &Suit::Wind).unwrap();
        assert_eq!(tile, Tile::Wind(Wind::East));

        let tile = Tile::new("W", &Suit::Wind).unwrap();
        assert_eq!(tile, Tile::Wind(Wind::West));
        let tile = Tile::new("3", &Suit::Wind).unwrap();
        assert_eq!(tile, Tile::Wind(Wind::West));

        let tile = Tile::new("S", &Suit::Wind).unwrap();
        assert_eq!(tile, Tile::Wind(Wind::South));
        let tile = Tile::new("2", &Suit::Wind).unwrap();
        assert_eq!(tile, Tile::Wind(Wind::South));

        let tile = Tile::new("N", &Suit::Wind).unwrap();
        assert_eq!(tile, Tile::Wind(Wind::North));
        let tile = Tile::new("4", &Suit::Wind).unwrap();
        assert_eq!(tile, Tile::Wind(Wind::North));
    }

    #[test]
    fn get_man_tile() {
        let tile = Tile::new("1", &Suit::Manzu).unwrap();
        assert_eq!(tile, Tile::Man(Man::OneMan));
        let tile = Tile::new("2", &Suit::Manzu).unwrap();
        assert_eq!(tile, Tile::Man(Man::TwoMan));
        let tile = Tile::new("3", &Suit::Manzu).unwrap();
        assert_eq!(tile, Tile::Man(Man::ThreeMan));
        let tile = Tile::new("4", &Suit::Manzu).unwrap();
        assert_eq!(tile, Tile::Man(Man::FourMan));
        let tile = Tile::new("5", &Suit::Manzu).unwrap();
        assert_eq!(tile, Tile::Man(Man::FiveMan));
        let tile = Tile::new("6", &Suit::Manzu).unwrap();
        assert_eq!(tile, Tile::Man(Man::SixMan));
        let tile = Tile::new("7", &Suit::Manzu).unwrap();
        assert_eq!(tile, Tile::Man(Man::SevenMan));
        let tile = Tile::new("8", &Suit::Manzu).unwrap();
        assert_eq!(tile, Tile::Man(Man::EightMan));
        let tile = Tile::new("9", &Suit::Manzu).unwrap();
        assert_eq!(tile, Tile::Man(Man::NineMan));
        let tile = Tile::new("0", &Suit::Manzu).unwrap();
        assert_eq!(tile, Tile::Man(Man::AkaFiveMan));
    }

    #[test]
    fn get_pin_tile() {
        let tile = Tile::new("1", &Suit::Pinzu).unwrap();
        assert_eq!(tile, Tile::Pin(Pin::OnePin));
        let tile = Tile::new("2", &Suit::Pinzu).unwrap();
        assert_eq!(tile, Tile::Pin(Pin::TwoPin));
        let tile = Tile::new("3", &Suit::Pinzu).unwrap();
        assert_eq!(tile, Tile::Pin(Pin::ThreePin));
        let tile = Tile::new("4", &Suit::Pinzu).unwrap();
        assert_eq!(tile, Tile::Pin(Pin::FourPin));
        let tile = Tile::new("5", &Suit::Pinzu).unwrap();
        assert_eq!(tile, Tile::Pin(Pin::FivePin));
        let tile = Tile::new("6", &Suit::Pinzu).unwrap();
        assert_eq!(tile, Tile::Pin(Pin::SixPin));
        let tile = Tile::new("7", &Suit::Pinzu).unwrap();
        assert_eq!(tile, Tile::Pin(Pin::SevenPin));
        let tile = Tile::new("8", &Suit::Pinzu).unwrap();
        assert_eq!(tile, Tile::Pin(Pin::EightPin));
        let tile = Tile::new("9", &Suit::Pinzu).unwrap();
        assert_eq!(tile, Tile::Pin(Pin::NinePin));
        let tile = Tile::new("0", &Suit::Pinzu).unwrap();
        assert_eq!(tile, Tile::Pin(Pin::AkaFivePin));
    }

    #[test]
    fn get_sou_tile() {
        let tile = Tile::new("1", &Suit::Souzu).unwrap();
        assert_eq!(tile, Tile::Sou(Sou::OneSou));
        let tile = Tile::new("2", &Suit::Souzu).unwrap();
        assert_eq!(tile, Tile::Sou(Sou::TwoSou));
        let tile = Tile::new("3", &Suit::Souzu).unwrap();
        assert_eq!(tile, Tile::Sou(Sou::ThreeSou));
        let tile = Tile::new("4", &Suit::Souzu).unwrap();
        assert_eq!(tile, Tile::Sou(Sou::FourSou));
        let tile = Tile::new("5", &Suit::Souzu).unwrap();
        assert_eq!(tile, Tile::Sou(Sou::FiveSou));
        let tile = Tile::new("6", &Suit::Souzu).unwrap();
        assert_eq!(tile, Tile::Sou(Sou::SixSou));
        let tile = Tile::new("7", &Suit::Souzu).unwrap();
        assert_eq!(tile, Tile::Sou(Sou::SevenSou));
        let tile = Tile::new("8", &Suit::Souzu).unwrap();
        assert_eq!(tile, Tile::Sou(Sou::EightSou));
        let tile = Tile::new("9", &Suit::Souzu).unwrap();
        assert_eq!(tile, Tile::Sou(Sou::NineSou));
        let tile = Tile::new("0", &Suit::Souzu).unwrap();
        assert_eq!(tile, Tile::Sou(Sou::AkaFiveSou));
    }

    #[test]
    fn next_dragon() {
        let mut tile = Tile::try_from("wd".to_string()).unwrap();
        tile.next().unwrap();
        assert_eq!(tile.value(), "g");
        assert_eq!(tile.suit(), Suit::Dragon);

        let mut tile = Tile::try_from("gd".to_string()).unwrap();
        tile.next().unwrap();
        assert_eq!(tile.value(), "r");

        let mut tile = Tile::try_from("rd".to_string()).unwrap();
        tile.next().unwrap();
        assert_eq!(tile.value(), "w");
    }
    #[test]
    fn next_wind() {
        let mut tile = Tile::try_from("Ew".to_string()).unwrap();
        tile.next().unwrap();
        assert_eq!(tile.value(), "S");
        assert_eq!(tile.suit(), Suit::Wind);

        let mut tile = Tile::try_from("Sw".to_string()).unwrap();
        tile.next().unwrap();
        assert_eq!(tile.value(), "W");

        let mut tile = Tile::try_from("Ww".to_string()).unwrap();
        tile.next().unwrap();
        assert_eq!(tile.value(), "N");

        let mut tile = Tile::try_from("Nw".to_string()).unwrap();
        tile.next().unwrap();
        assert_eq!(tile.value(), "E");
    }

    #[test]
    fn next_manpinsou() {
        let mut tile = Tile::try_from("1m".to_string()).unwrap();
        tile.next().unwrap();
        assert_eq!(tile.value(), "2");
        assert_eq!(tile.suit(), Suit::Manzu);

        let mut tile = Tile::try_from("9m".to_string()).unwrap();
        tile.next().unwrap();
        assert_eq!(tile.value(), "1");

        let mut tile = Tile::try_from("0m".to_string()).unwrap();
        tile.next().unwrap();
        assert_eq!(tile.value(), "6");
    }
}
