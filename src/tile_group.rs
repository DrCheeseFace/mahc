use crate::hand::error::HandErr;
use crate::suit::Suit;
use crate::tile::Tile;
use crate::{
    AKAFIVE_VALUE, EAST_VALUE, EAST_VALUE_Z, EIGHT_VALUE, FIVE_VALUE, FOUR_VALUE, GREEN_VALUE,
    GREEN_VALUE_Z, NINE_VALUE, NORTH_VALUE, NORTH_VALUE_Z, ONE_VALUE, OPEN_CHAR, RED_VALUE,
    RED_VALUE_Z, SEVEN_VALUE, SIX_VALUE, SOUTH_VALUE, SOUTH_VALUE_Z, THREE_VALUE, TWO_VALUE,
    VALID_SEQUENCE_VALUES, WEST_VALUE, WEST_VALUE_Z, WHITE_VALUE, WHITE_VALUE_Z, Z_SUIT_CHAR,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TileGroup {
    pub tiles: Vec<Tile>,
    pub isopen: bool,
    pub group_type: GroupType,
}

impl TryFrom<String> for TileGroup {
    type Error = HandErr;
    fn try_from(group: String) -> Result<Self, Self::Error> {
        let isopen = group.chars().last().unwrap() == OPEN_CHAR;

        let value = group.chars().nth(0).unwrap();

        let suitchar = if !isopen {
            group.chars().last().unwrap()
        } else {
            group.chars().nth(group.len() - 2).unwrap()
        };

        let suit = Suit::suit_from_string(suitchar, value)?;
        let value = if suitchar == Z_SUIT_CHAR {
            match value {
                EAST_VALUE_Z => EAST_VALUE,
                SOUTH_VALUE_Z => SOUTH_VALUE,
                WEST_VALUE_Z => WEST_VALUE,
                NORTH_VALUE_Z => NORTH_VALUE,
                WHITE_VALUE_Z => WHITE_VALUE,
                GREEN_VALUE_Z => GREEN_VALUE,
                RED_VALUE_Z => RED_VALUE,
                _ => value,
            }
        } else {
            value
        };

        let group_type = GroupType::group_type_from_string(group.to_string())?;
        let mut tiles: Vec<Tile> = Vec::new();
        let tile = Tile::new(value, &suit)?;

        match group_type {
            GroupType::Sequence => {
                for i in 0..3 {
                    let value = group.chars().nth(i).unwrap();
                    let tile = Tile::new(value, &suit)?;
                    tiles.push(tile.clone());
                }
            }
            GroupType::Triplet => {
                for i in 0..3 {
                    let value = group.chars().nth(i).unwrap();
                    let tile = Tile::new(value, &suit)?;
                    tiles.push(tile.clone());
                }
            }
            GroupType::Kan => {
                for i in 0..4 {
                    let value = group.chars().nth(i).unwrap();
                    let tile = Tile::new(value, &suit)?;
                    tiles.push(tile.clone());
                }
            }
            GroupType::Pair => {
                tiles.push(tile.clone());
                let value = group.chars().nth(1).unwrap();
                let tile = Tile::new(value, &suit)?;
                tiles.push(tile.clone());
            }
            GroupType::None => tiles.push(tile.clone()),
        }

        TileGroup::new(tiles, isopen, group_type)
    }
}

impl TileGroup {
    fn new(tiles: Vec<Tile>, isopen: bool, group_type: GroupType) -> Result<Self, HandErr> {
        let tile = Self {
            tiles,
            isopen,
            group_type,
        };

        Ok(tile)
    }

    /// Check if the group is an honor.
    pub fn is_honor(&self) -> bool {
        for tile in self.tiles.iter() {
            if tile.is_honor() {
                return true;
            }
        }
        false
    }

    /// Parse the group value into an integer.
    pub fn parse_u8(&self) -> Result<u8, std::num::ParseIntError> {
        self.value().to_string().parse()
    }

    /// Get value of tilegroup
    pub fn value(&self) -> char {
        self.tiles[0].value()
    }

    /// Check if tilegroup contains a terminal
    pub fn is_terminal(&self) -> bool {
        for tile in self.tiles.iter() {
            if tile.is_terminal() {
                return true;
            }
        }
        false
    }

    /// Get Suit
    pub fn suit(&self) -> Suit {
        self.tiles[0].suit().clone()
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
        let count = if group.contains(OPEN_CHAR) {
            group.len() - 2
        } else {
            group.len() - 1
        };
        group = group.replace(AKAFIVE_VALUE, &FIVE_VALUE.to_string());

        if let Some(sub_group) = group.get(0..count) {
            for i in sub_group.chars() {
                if ![
                    ONE_VALUE,
                    TWO_VALUE,
                    THREE_VALUE,
                    FOUR_VALUE,
                    FIVE_VALUE,
                    SIX_VALUE,
                    SEVEN_VALUE,
                    EIGHT_VALUE,
                    NINE_VALUE,
                    AKAFIVE_VALUE,
                    EAST_VALUE,
                    EAST_VALUE_Z,
                    SOUTH_VALUE,
                    SOUTH_VALUE_Z,
                    WEST_VALUE,
                    WEST_VALUE_Z,
                    NORTH_VALUE,
                    NORTH_VALUE_Z,
                    RED_VALUE,
                    RED_VALUE_Z,
                    WHITE_VALUE,
                    WHITE_VALUE_Z,
                    GREEN_VALUE,
                    GREEN_VALUE_Z,
                ]
                .contains(&i)
                {
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
                } else if VALID_SEQUENCE_VALUES
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
    use crate::{FOUR_VALUE, ONE_VALUE};

    use super::*;

    #[test]
    fn non_honor_tilegroup_from_string() {
        let tile = TileGroup::try_from("1m".to_string()).unwrap();
        assert_eq!(tile.suit(), Suit::Manzu);
        assert_eq!(tile.value(), ONE_VALUE);
        assert!(!tile.isopen);
        assert_eq!(tile.group_type, GroupType::None);
        assert!(tile.is_terminal());

        let tile = TileGroup::try_from("111mo".to_string()).unwrap();
        assert!(tile.isopen);
        assert_eq!(tile.group_type, GroupType::Triplet);
        assert_eq!(tile.suit(), Suit::Manzu);

        let tile = TileGroup::try_from("123m".to_string()).unwrap();
        assert_eq!(tile.group_type, GroupType::Sequence);
        assert_eq!(tile.suit(), Suit::Manzu);

        let tile = TileGroup::try_from("234m".to_string()).unwrap();
        assert_eq!(tile.group_type, GroupType::Sequence);
        assert_eq!(tile.suit(), Suit::Manzu);
        assert!(!tile.is_terminal());
    }

    #[test]
    fn wind_tilegroup_from_string() {
        let tile = TileGroup::try_from("1z".to_string()).unwrap();
        assert_eq!(tile.suit(), Suit::Wind);
        assert_eq!(tile.value(), EAST_VALUE);
        assert!(!tile.isopen);
        assert_eq!(tile.group_type, GroupType::None);
        assert_eq!(tile.is_terminal(), false);

        let tile = TileGroup::try_from("222zo".to_string()).unwrap();
        assert!(tile.isopen);
        assert_eq!(tile.group_type, GroupType::Triplet);
        assert_eq!(tile.suit(), Suit::Wind);
        assert_eq!(tile.value(), SOUTH_VALUE);

        let tile = TileGroup::try_from("EEEEw".to_string()).unwrap();
        assert!(!tile.isopen);
        assert_eq!(tile.group_type, GroupType::Kan);
        assert_eq!(tile.suit(), Suit::Wind);
        assert_eq!(tile.value(), EAST_VALUE);
    }

    #[test]
    fn dragon_tilegroup_from_string() {
        let tile = TileGroup::try_from("5z".to_string()).unwrap();
        assert_eq!(tile.suit(), Suit::Dragon);
        assert_eq!(tile.value(), WHITE_VALUE);
        assert!(!tile.isopen);
        assert_eq!(tile.group_type, GroupType::None);

        let tile = TileGroup::try_from("666zo".to_string()).unwrap();
        assert_eq!(tile.suit(), Suit::Dragon);
        assert_eq!(tile.value(), GREEN_VALUE);
        assert!(tile.isopen);
        assert_eq!(tile.group_type, GroupType::Triplet);

        let tile = TileGroup::try_from("7777z".to_string()).unwrap();
        assert!(!tile.isopen);
        assert_eq!(tile.group_type, GroupType::Kan);
        assert_eq!(tile.suit(), Suit::Dragon);
        assert_eq!(tile.value(), RED_VALUE);
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
        assert_eq!(tile.value(), FIVE_VALUE);
        assert_eq!(tile.tiles[0].is_aka(), true);
        assert_eq!(tile.group_type, GroupType::None);

        let tile = TileGroup::try_from("055m".to_string()).unwrap();
        assert_eq!(tile.value(), FIVE_VALUE);
        assert_eq!(tile.tiles[0].is_aka(), true);
        assert_eq!(tile.tiles[1].is_aka(), false);
        assert_eq!(tile.tiles[2].is_aka(), false);
        assert_eq!(tile.group_type, GroupType::Triplet);

        let tile = TileGroup::try_from("406m".to_string()).unwrap();
        assert_eq!(tile.value(), FOUR_VALUE);
        assert_eq!(tile.tiles[0].is_aka(), false);
        assert_eq!(tile.tiles[1].is_aka(), true);
        assert_eq!(tile.tiles[2].is_aka(), false);
        assert_eq!(tile.group_type, GroupType::Sequence);
    }

    #[test]
    fn is_not_akadora_from_string() {
        let tile = TileGroup::try_from("1m".to_string()).unwrap();
        assert_eq!(tile.value(), ONE_VALUE);
        assert_eq!(tile.tiles[0].is_aka(), false);
        assert_eq!(tile.group_type, GroupType::None);
    }
}
