use crate::hand::error::HandErr;
use crate::suit::Suit;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TileGroup {
    pub value: String,
    pub suit: Suit,
    pub isopen: bool,
    pub group_type: GroupType,
    pub isterminal: bool,
    pub isaka: bool,
}

impl TryFrom<String> for TileGroup {
    type Error = HandErr;
    fn try_from(group: String) -> Result<Self, Self::Error> {
        let isopen = group.chars().last().unwrap().to_string() == "o";

        // is akadora check (sussy bcuz not every tile needs an akadora attribute)
        let mut isaka = false;
        let value = if group.chars().nth(0).unwrap().to_string() == "0" {
            "5".to_string()
        } else {
            group.chars().nth(0).unwrap().to_string()
        };
        if group.contains('0') {
            isaka = true;
        }

        let suitchar = if !isopen {
            group.chars().last().unwrap().to_string()
        } else {
            group.chars().nth(group.len() - 2).unwrap().to_string()
        };
        let suit = Suit::suit_from_string(&suitchar, &value)?;
        let value = if suitchar == "z" {
            match value.as_str() {
                "1" => "E".to_string(),
                "2" => "S".to_string(),
                "3" => "W".to_string(),
                "4" => "N".to_string(),
                "5" => "w".to_string(),
                "6" => "g".to_string(),
                "7" => "r".to_string(),
                _ => value,
            }
        } else {
            value
        };

        let group_type = GroupType::group_type_from_string(group.to_string())?;

        let mut isterminal = false;
        if group_type == GroupType::Sequence {
            if value == "1" || value == "7" {
                isterminal = true;
            }
        } else if (value == "1" || value == "9") && suit != Suit::Wind && suit != Suit::Dragon {
            isterminal = true;
        }

        TileGroup::new(value, suit, isopen, group_type, isterminal, isaka)
    }
}

impl TileGroup {
    fn new(
        value: String,
        suit: Suit,
        isopen: bool,
        group_type: GroupType,
        isterminal: bool,
        isaka: bool,
    ) -> Result<Self, HandErr> {
        let tile = Self {
            value,
            suit,
            isopen,
            group_type,
            isterminal,
            isaka,
        };

        Ok(tile)
    }

    /// Check if the group is an honor.
    pub fn is_honor(&self) -> bool {
        matches!(self.suit, Suit::Wind | Suit::Dragon)
    }

    /// Parse the group value into an integer.
    pub fn parse_u8(&self) -> Result<u8, std::num::ParseIntError> {
        self.value.parse()
    }

    /// Get the next tile  
    /// Usually used for getting the dora tile from the dora indicator tile
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mahc::tile_group::TileGroup;
    /// use mahc::suit::Suit;
    /// let tile: TileGroup = "7s".to_string().try_into().unwrap();
    /// let input = tile.next_tile().unwrap();
    /// let actual = input.value;
    /// let expected = "8";
    ///
    /// assert_eq!(actual, expected);
    ///
    /// let actual = input.suit;
    /// let expected = Suit::Souzu;
    ///
    /// assert_eq!(actual, expected);
    /// ```
    pub fn next_tile(&self) -> Result<Self, HandErr> {
        let value: String = match self.suit {
            Suit::Souzu | Suit::Manzu | Suit::Pinzu => {
                let value = (self.parse_u8().unwrap() + 1).to_string();
                if value == "10" {
                    "1".to_string()
                } else {
                    value
                }
            }
            Suit::Wind => match self.value.as_str() {
                "E" => "S".to_string(),
                "S" => "W".to_string(),
                "W" => "N".to_string(),
                "N" => "E".to_string(),
                _ => return Err(HandErr::InvalidGroup),
            },
            Suit::Dragon => match self.value.as_str() {
                "w" => "g".to_string(),
                "g" => "r".to_string(),
                "r" => "w".to_string(),
                _ => return Err(HandErr::InvalidGroup),
            },
        };
        Self::new(
            value,
            self.suit.clone(),
            false,
            self.group_type.clone(),
            false,
            false,
        )
    }
}

//AHAHAHAHAHAHAHAH I DONT NEED THIS
//turns our i did need this :)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum GroupType {
    Sequence,
    Triplet,
    Kan,
    Pair,
    None,
}

impl GroupType {
    /// Parse the group type from the string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mahc::tile_group::GroupType;
    ///
    /// let input = "789s".to_string();
    /// let actual = GroupType::group_type_from_string(input);
    /// let expected = Ok(GroupType::Sequence);
    ///
    /// assert_eq!(actual, expected);
    /// ```
    pub fn group_type_from_string(mut group: String) -> Result<Self, HandErr> {
        let count = if group.contains('o') {
            group.len() - 2
        } else {
            group.len() - 1
        };
        group = group.replace('0', "5");

        if let Some(sub_group) = group.get(0..count) {
            for i in sub_group.chars() {
                if !"123456789ESWNrgw".contains(i) {
                    return Err(HandErr::InvalidGroup);
                }
            }
        } else {
            return Err(HandErr::InvalidGroup);
        }

        match count {
            2 => Ok(Self::Pair),
            3 => {
                if group.chars().nth(0).unwrap() == group.chars().nth(1).unwrap()
                    && group.chars().nth(1).unwrap() == group.chars().nth(2).unwrap()
                {
                    Ok(Self::Triplet)
                } else if ["123", "234", "345", "456", "567", "678", "789"]
                    .iter()
                    .cloned()
                    .collect::<std::collections::HashSet<&str>>()
                    .contains(group.get(0..count).unwrap())
                {
                    return Ok(Self::Sequence);
                } else {
                    return Err(HandErr::InvalidGroup);
                }
            }
            4 => Ok(Self::Kan),
            1 => Ok(Self::None),
            _ => Err(HandErr::InvalidGroup),
        }
    }

    pub fn tile_count(&self) -> u8 {
        match self {
            Self::Pair => 2,
            Self::Triplet => 3,
            Self::Sequence => 3,
            Self::Kan => 4,
            Self::None => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_honor_tilegroup_from_string() {
        let tile = TileGroup::try_from("1m".to_string()).unwrap();
        assert_eq!(tile.suit, Suit::Manzu);
        assert_eq!(tile.value, "1");
        assert!(!tile.isopen);
        assert_eq!(tile.group_type, GroupType::None);
        assert!(tile.isterminal);

        let tile = TileGroup::try_from("111mo".to_string()).unwrap();
        assert!(tile.isopen);
        assert_eq!(tile.group_type, GroupType::Triplet);
        assert_eq!(tile.suit, Suit::Manzu);

        let tile = TileGroup::try_from("123m".to_string()).unwrap();
        assert_eq!(tile.group_type, GroupType::Sequence);
        assert_eq!(tile.suit, Suit::Manzu);

        let tile = TileGroup::try_from("234m".to_string()).unwrap();
        assert_eq!(tile.group_type, GroupType::Sequence);
        assert_eq!(tile.suit, Suit::Manzu);
        assert!(!tile.isterminal);
    }

    #[test]
    fn wind_tilegroup_from_string() {
        let tile = TileGroup::try_from("1z".to_string()).unwrap();
        assert_eq!(tile.suit, Suit::Wind);
        assert_eq!(tile.value, "E");
        assert!(!tile.isopen);
        assert_eq!(tile.group_type, GroupType::None);
        assert_eq!(tile.isterminal, false);

        let tile = TileGroup::try_from("222zo".to_string()).unwrap();
        assert!(tile.isopen);
        assert_eq!(tile.group_type, GroupType::Triplet);
        assert_eq!(tile.suit, Suit::Wind);
        assert_eq!(tile.value, "S");

        let tile = TileGroup::try_from("EEEEw".to_string()).unwrap();
        assert!(!tile.isopen);
        assert_eq!(tile.group_type, GroupType::Kan);
        assert_eq!(tile.suit, Suit::Wind);
        assert_eq!(tile.value, "E");
    }

    #[test]
    fn dragon_tilegroup_from_string() {
        let tile = TileGroup::try_from("5z".to_string()).unwrap();
        assert_eq!(tile.suit, Suit::Dragon);
        assert_eq!(tile.value, "w");
        assert!(!tile.isopen);
        assert_eq!(tile.group_type, GroupType::None);

        let tile = TileGroup::try_from("666zo".to_string()).unwrap();
        assert_eq!(tile.suit, Suit::Dragon);
        assert_eq!(tile.value, "g");
        assert!(tile.isopen);
        assert_eq!(tile.group_type, GroupType::Triplet);

        let tile = TileGroup::try_from("7777z".to_string()).unwrap();
        assert!(!tile.isopen);
        assert_eq!(tile.group_type, GroupType::Kan);
        assert_eq!(tile.suit, Suit::Dragon);
        assert_eq!(tile.value, "r");
    }

    #[test]
    fn no_suit_error_from_string() {
        let tile = TileGroup::try_from("1".to_string());
        assert_eq!(tile, Err(HandErr::InvalidSuit));
    }

    #[test]
    fn no_value_error_from_string() {
        let tile = TileGroup::try_from("m".to_string());
        assert_eq!(tile, Err(HandErr::InvalidGroup));
    }

    #[test]
    fn too_large_error_from_string() {
        let tile = TileGroup::try_from("11111s".to_string());
        assert_eq!(tile, Err(HandErr::InvalidGroup));
    }

    #[test]
    fn invalid_suit_error_from_string() {
        let tile = TileGroup::try_from("999z".to_string());
        assert_eq!(tile, Err(HandErr::InvalidGroup));
    }

    #[test]
    fn is_akadora_from_string() {
        let tile = TileGroup::try_from("0m".to_string()).unwrap();
        assert_eq!(tile.value, "5");
        assert_eq!(tile.isaka, true);
        assert_eq!(tile.group_type, GroupType::None);

        let tile = TileGroup::try_from("055m".to_string()).unwrap();
        assert_eq!(tile.value, "5");
        assert_eq!(tile.isaka, true);
        assert_eq!(tile.group_type, GroupType::Triplet);

        let tile = TileGroup::try_from("406m".to_string()).unwrap();
        assert_eq!(tile.value, "4");
        assert_eq!(tile.isaka, true);
        assert_eq!(tile.group_type, GroupType::Sequence);
    }

    #[test]
    fn is_not_akadora_from_string() {
        let tile = TileGroup::try_from("1m".to_string()).unwrap();
        assert_eq!(tile.value, "1");
        assert_eq!(tile.isaka, false);
        assert_eq!(tile.group_type, GroupType::None);
    }

    #[test]
    fn next_dragon() {
        let tile = TileGroup::try_from("wd".to_string()).unwrap();
        let next_tile = tile.next_tile().unwrap();
        assert_eq!(next_tile.value, "g");
        assert_eq!(next_tile.suit, Suit::Dragon);

        let tile = TileGroup::try_from("gd".to_string()).unwrap();
        let next_tile = tile.next_tile().unwrap();
        assert_eq!(next_tile.value, "r");

        let tile = TileGroup::try_from("rd".to_string()).unwrap();
        let next_tile = tile.next_tile().unwrap();
        assert_eq!(next_tile.value, "w");
    }
    #[test]
    fn next_wind() {
        let tile = TileGroup::try_from("Ew".to_string()).unwrap();
        let next_tile = tile.next_tile().unwrap();
        assert_eq!(next_tile.value, "S");
        assert_eq!(next_tile.suit, Suit::Wind);

        let tile = TileGroup::try_from("Sw".to_string()).unwrap();
        let next_tile = tile.next_tile().unwrap();
        assert_eq!(next_tile.value, "W");

        let tile = TileGroup::try_from("Ww".to_string()).unwrap();
        let next_tile = tile.next_tile().unwrap();
        assert_eq!(next_tile.value, "N");

        let tile = TileGroup::try_from("Nw".to_string()).unwrap();
        let next_tile = tile.next_tile().unwrap();
        assert_eq!(next_tile.value, "E");
    }

    #[test]
    fn next_manpinsou() {
        let tile = TileGroup::try_from("1m".to_string()).unwrap();
        let next_tile = tile.next_tile().unwrap();
        assert_eq!(next_tile.value, "2");
        assert_eq!(next_tile.suit, Suit::Manzu);

        let tile = TileGroup::try_from("9m".to_string()).unwrap();
        let next_tile = tile.next_tile().unwrap();
        assert_eq!(next_tile.value, "1");

        let tile = TileGroup::try_from("0m".to_string()).unwrap();
        let next_tile = tile.next_tile().unwrap();
        assert_eq!(next_tile.value, "6");
    }
}
