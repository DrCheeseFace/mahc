use crate::{hand::error::HandErr, tile::*};

#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub enum Suit {
    Manzu,
    Pinzu,
    Souzu,
    Wind,
    Dragon,
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Suit::Manzu => write!(f, "{}", MAN_SUIT_CHAR),
            Suit::Pinzu => write!(f, "{}", PIN_SUIT_CHAR),
            Suit::Souzu => write!(f, "{}", SOU_SUIT_CHAR),
            Suit::Wind => write!(f, "{}", WIND_SUIT_CHAR),
            Suit::Dragon => write!(f, "{}", DRAGON_SUIT_CHAR),
        }
    }
}

impl Suit {
    /// Parse the suit from the string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mahc::suit::Suit;
    ///
    /// let tile_string = "9m";
    /// let actual_suit = Suit::suit_from_string(tile_string.chars().nth(1).unwrap(), tile_string.chars().nth(0).unwrap());
    /// let expected = Ok(Suit::Manzu);
    ///
    /// assert_eq!(actual_suit, expected);
    ///
    /// let tile_string = "6z";
    /// let actual_suit = Suit::suit_from_string(tile_string.chars().nth(1).unwrap(), tile_string.chars().nth(0).unwrap());
    /// let expected = Ok(Suit::Dragon);
    ///
    /// assert_eq!(actual_suit, expected);
    /// ```
    pub fn suit_from_string(suit: char, value: char) -> Result<Self, HandErr> {
        if [SOU_SUIT_CHAR, PIN_SUIT_CHAR, MAN_SUIT_CHAR].contains(&suit)
            && ![
                AKAFIVE_VALUE,
                ONE_VALUE,
                TWO_VALUE,
                THREE_VALUE,
                FOUR_VALUE,
                FIVE_VALUE,
                SIX_VALUE,
                SEVEN_VALUE,
                EIGHT_VALUE,
                NINE_VALUE,
            ]
            .contains(&value)
        {
            return Err(HandErr::InvalidGroup);
        }
        match suit {
            SOU_SUIT_CHAR => Ok(Self::Souzu),
            PIN_SUIT_CHAR => Ok(Self::Pinzu),
            MAN_SUIT_CHAR => Ok(Self::Manzu),
            WIND_SUIT_CHAR => {
                if ![EAST_VALUE, SOUTH_VALUE, WEST_VALUE, NORTH_VALUE].contains(&value) {
                    Err(HandErr::InvalidGroup)
                } else {
                    Ok(Self::Wind)
                }
            }
            DRAGON_SUIT_CHAR => {
                if ![RED_VALUE, GREEN_VALUE, WHITE_VALUE].contains(&value) {
                    Err(HandErr::InvalidGroup)
                } else {
                    Ok(Self::Dragon)
                }
            }
            Z_SUIT_CHAR => {
                if [EAST_VALUE_Z, SOUTH_VALUE_Z, NORTH_VALUE_Z, WEST_VALUE_Z].contains(&value) {
                    Ok(Self::Wind)
                } else if [RED_VALUE_Z, GREEN_VALUE_Z, WHITE_VALUE_Z].contains(&value) {
                    Ok(Self::Dragon)
                } else {
                    Err(HandErr::InvalidGroup)
                }
            }
            _ => Err(HandErr::InvalidSuit),
        }
    }
}

pub const MAN_SUIT_CHAR: char = 'm';
pub const PIN_SUIT_CHAR: char = 'p';
pub const SOU_SUIT_CHAR: char = 's';

pub const DRAGON_SUIT_CHAR: char = 'd';
pub const WIND_SUIT_CHAR: char = 'w';
pub const Z_SUIT_CHAR: char = 'z';

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn souzu_suit_from_string() {
        let suit = SOU_SUIT_CHAR;
        let value = ONE_VALUE;
        let actual = Suit::suit_from_string(suit, value);
        let expected = Ok(Suit::Souzu);

        assert_eq!(actual, expected);
    }

    #[test]
    fn manzu_suit_from_string() {
        let suit = MAN_SUIT_CHAR;
        let value = ONE_VALUE;
        let actual = Suit::suit_from_string(suit, value);
        let expected = Ok(Suit::Manzu);

        assert_eq!(actual, expected);
    }
    #[test]
    fn pinzu_suit_from_string() {
        let suit = PIN_SUIT_CHAR;
        let value = ONE_VALUE;
        let actual = Suit::suit_from_string(suit, value);
        let expected = Ok(Suit::Pinzu);

        assert_eq!(actual, expected);
    }
    #[test]
    fn wind_suit_from_string() {
        let suit = Z_SUIT_CHAR;
        let value = ONE_VALUE;
        let actual = Suit::suit_from_string(suit, value);
        let expected = Ok(Suit::Wind);

        assert_eq!(actual, expected);
        let suit = Z_SUIT_CHAR;
        let value = FOUR_VALUE;
        let actual = Suit::suit_from_string(suit, value);
        let expected = Ok(Suit::Wind);

        assert_eq!(actual, expected);
        let suit = WIND_SUIT_CHAR;
        let value = WEST_VALUE;
        let actual = Suit::suit_from_string(suit, value);
        let expected = Ok(Suit::Wind);

        assert_eq!(actual, expected);
    }
    #[test]
    fn akadora_suit_from_string() {
        let value = AKAFIVE_VALUE;
        let suit = MAN_SUIT_CHAR;
        let actual = Suit::suit_from_string(suit, value);
        let expected = Ok(Suit::Manzu);
        assert_eq!(actual, expected);

        let value = AKAFIVE_VALUE;
        let suit = Z_SUIT_CHAR;
        let actual = Suit::suit_from_string(suit, value);
        let expected = Err(HandErr::InvalidGroup);
        assert_eq!(actual, expected);
    }
}
