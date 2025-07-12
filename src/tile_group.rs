use crate::hand::error::HandErr;
use crate::suit::{Suit, Z_SUIT_CHAR};
use crate::{VALID_SEQUENCE_VALUES, tile::*};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TileGroup {
    tiles: Vec<Tile>,
    isopen: bool,
    group_type: GroupType,
}

impl std::fmt::Display for TileGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        for tile in self.tiles.iter() {
            out.push(tile.value())
        }
        out.push_str(&self.suit().to_string());
        if self.isopen {
            out.push('o');
        }
        write!(f, "{out}")
    }
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
                    tiles.push(tile);
                }
            }
            GroupType::Triplet => {
                for i in 0..3 {
                    let value = group.chars().nth(i).unwrap();
                    let tile = Tile::new(value, &suit)?;
                    tiles.push(tile);
                }
            }
            GroupType::Kan => {
                for i in 0..4 {
                    let value = group.chars().nth(i).unwrap();
                    let tile = Tile::new(value, &suit)?;
                    tiles.push(tile);
                }
            }
            GroupType::Pair => {
                tiles.push(tile);
                let value = group.chars().nth(1).unwrap();
                let tile = Tile::new(value, &suit)?;
                tiles.push(tile);
            }
            GroupType::None => tiles.push(tile),
        }

        TileGroup::new(tiles, isopen)
    }
}

impl TileGroup {
    /// Creating a new tilegroup
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mahc::tile_group::TileGroup;
    /// use mahc::tile_group::GroupType;
    /// use mahc::tile::Tile;
    /// use mahc::hand::error::HandErr;
    /// use mahc::tile::Tile::Man;
    /// use mahc::tile::MpsValue;
    ///
    /// let tile_7 = Tile::Man(MpsValue::Seven);
    /// let tile_8 = Tile::Man(MpsValue::Eight);
    /// let tile_9 = Tile::Man(MpsValue::Nine);
    /// let group = vec![tile_7.clone(), tile_8.clone(), tile_9.clone()];
    ///
    /// let tile_group = TileGroup::new(group, true).unwrap();
    /// assert!(tile_group.is_terminal());
    /// assert!(tile_group.isopen());
    /// assert_eq!(tile_group.group_type(), GroupType::Sequence);
    ///
    /// let group = vec![tile_7.clone(), tile_8.clone(), tile_8.clone()];
    ///
    /// let tile_group = TileGroup::new(group, true);
    /// assert_eq!(tile_group.err(), Some(HandErr::InvalidGroup));
    ///
    /// ```
    pub fn new(tiles: Vec<Tile>, isopen: bool) -> Result<Self, HandErr> {
        let group_type = match tiles.len() {
            1 => Ok(GroupType::None),
            2 => Ok(GroupType::Pair),
            3 => {
                let mut values: Vec<char> =
                    tiles.iter().map(|tile| tile.value()).collect::<Vec<_>>();
                values.dedup();
                if values.len() == 1 {
                    Ok(GroupType::Triplet)
                } else {
                    Ok(GroupType::Sequence)
                }
            }
            4 => Ok(GroupType::Kan),
            _ => Err(HandErr::InvalidGroup),
        }?;

        match group_type {
            GroupType::Kan | GroupType::Pair | GroupType::None | GroupType::Triplet => {
                let mut values: Vec<char> =
                    tiles.iter().map(|tile| tile.value()).collect::<Vec<_>>();
                values.dedup();
                if values.len() != 1 {
                    return Err(HandErr::InvalidGroup);
                }
            }
            GroupType::Sequence => {
                for i in 0..2 {
                    if tiles[i].is_honor() {
                        return Err(HandErr::InvalidGroup);
                    }
                    let mut next_tile = tiles[i];
                    next_tile.next();
                    if tiles[i + 1].value() != next_tile.value() || next_tile.value() == ONE_VALUE {
                        return Err(HandErr::InvalidGroup);
                    }
                }
            }
        }

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
        self.tiles[0].suit()
    }

    /// Get tiles
    pub fn tiles(&self) -> Vec<&Tile> {
        self.tiles.iter().collect()
    }

    /// Get open status
    pub fn isopen(&self) -> bool {
        self.isopen
    }

    // Get group type
    pub fn group_type(&self) -> GroupType {
        self.group_type
    }

    /// get emoji format for tile group
    pub fn get_emoji(&self) -> String {
        let mut out: String = String::new();
        let mut has_aka: bool = false;
        for tile in self.tiles.iter() {
            out.push_str(tile.get_emoji());
            // for some reason, the red dragon emoji is different and doesnt need a space added
            if tile.is_aka() {
                has_aka = true;
            }
        }
        if self.isopen {
            out.push('o');
        }
        if has_aka {
            out.push('a');
        }
        out
    }

    /// Parse the group value into an integer.
    pub fn parse_u8(&self) -> Result<u8, HandErr> {
        self.tiles[0].parse_u8()
    }
}

//AHAHAHAHAHAHAHAH I DONT NEED THIS
//turns our i did need this :)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
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
    pub fn group_type_from_string(group: String) -> Result<Self, HandErr> {
        let count = if group.contains(OPEN_CHAR) {
            group.len() - 2
        } else {
            group.len() - 1
        };
        //easier to do this instead of mess with akadora values :|
        let group = group.replace(AKAFIVE_VALUE, &FIVE_VALUE.to_string());

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
            2 => {
                if group.chars().nth(0).unwrap() == group.chars().nth(1).unwrap() {
                    Ok(Self::Pair)
                } else {
                    Err(HandErr::InvalidGroup)
                }
            }
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
            4 => {
                if group.chars().nth(0).unwrap() == group.chars().nth(1).unwrap()
                    && group.chars().nth(1).unwrap() == group.chars().nth(2).unwrap()
                    && group.chars().nth(2).unwrap() == group.chars().nth(3).unwrap()
                {
                    Ok(Self::Kan)
                } else {
                    Err(HandErr::InvalidGroup)
                }
            }
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

pub const OPEN_CHAR: char = 'o';

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_honor_tilegroup_from_string() {
        let tiles = TileGroup::try_from("1m".to_string()).unwrap();
        assert_eq!(tiles.suit(), Suit::Manzu);
        assert_eq!(tiles.value(), ONE_VALUE);
        assert!(!tiles.isopen);
        assert_eq!(tiles.group_type, GroupType::None);
        assert!(tiles.is_terminal());

        let tiles = TileGroup::try_from("111mo".to_string()).unwrap();
        assert!(tiles.isopen);
        assert_eq!(tiles.group_type, GroupType::Triplet);
        assert_eq!(tiles.suit(), Suit::Manzu);

        let tiles = TileGroup::try_from("123m".to_string()).unwrap();
        assert_eq!(tiles.group_type, GroupType::Sequence);
        assert_eq!(tiles.suit(), Suit::Manzu);

        let tiles = TileGroup::try_from("234m".to_string()).unwrap();
        assert_eq!(tiles.group_type, GroupType::Sequence);
        assert_eq!(tiles.suit(), Suit::Manzu);
        assert!(!tiles.is_terminal());
    }

    #[test]
    fn wind_tilesgroup_from_string() {
        let tiles = TileGroup::try_from("1z".to_string()).unwrap();
        assert_eq!(tiles.suit(), Suit::Wind);
        assert_eq!(tiles.value(), EAST_VALUE);
        assert!(!tiles.isopen);
        assert_eq!(tiles.group_type, GroupType::None);
        assert_eq!(tiles.is_terminal(), false);

        let tiles = TileGroup::try_from("222zo".to_string()).unwrap();
        assert!(tiles.isopen);
        assert_eq!(tiles.group_type, GroupType::Triplet);
        assert_eq!(tiles.suit(), Suit::Wind);
        assert_eq!(tiles.value(), SOUTH_VALUE);

        let tiles = TileGroup::try_from("EEEEw".to_string()).unwrap();
        assert!(!tiles.isopen);
        assert_eq!(tiles.group_type, GroupType::Kan);
        assert_eq!(tiles.suit(), Suit::Wind);
        assert_eq!(tiles.value(), EAST_VALUE);
    }

    #[test]
    fn dragon_tilesgroup_from_string() {
        let tiles = TileGroup::try_from("5z".to_string()).unwrap();
        assert_eq!(tiles.suit(), Suit::Dragon);
        assert_eq!(tiles.value(), WHITE_VALUE);
        assert!(!tiles.isopen);
        assert_eq!(tiles.group_type, GroupType::None);

        let tiles = TileGroup::try_from("666zo".to_string()).unwrap();
        assert_eq!(tiles.suit(), Suit::Dragon);
        assert_eq!(tiles.value(), GREEN_VALUE);
        assert!(tiles.isopen);
        assert_eq!(tiles.group_type, GroupType::Triplet);

        let tiles = TileGroup::try_from("7777z".to_string()).unwrap();
        assert!(!tiles.isopen);
        assert_eq!(tiles.group_type, GroupType::Kan);
        assert_eq!(tiles.suit(), Suit::Dragon);
        assert_eq!(tiles.value(), RED_VALUE);
    }

    #[test]
    fn no_suit_error_from_string() {
        let tiles = TileGroup::try_from("1".to_string());
        assert_eq!(tiles, Err(HandErr::InvalidSuit));
    }

    #[test]
    fn no_value_error_from_string() {
        let tiles = TileGroup::try_from("m".to_string());
        assert_eq!(tiles, Err(HandErr::InvalidGroup));
    }

    #[test]
    fn too_large_error_from_string() {
        let tiles = TileGroup::try_from("11111s".to_string());
        assert_eq!(tiles, Err(HandErr::InvalidGroup));
    }

    #[test]
    fn invalid_suit_error_from_string() {
        let tiles = TileGroup::try_from("999z".to_string());
        assert_eq!(tiles, Err(HandErr::InvalidGroup));
    }

    #[test]
    fn is_akadora_from_string() {
        let tiles = TileGroup::try_from("0m".to_string()).unwrap();
        assert_eq!(tiles.value(), FIVE_VALUE);
        assert_eq!(tiles.tiles[0].is_aka(), true);
        assert_eq!(tiles.group_type, GroupType::None);

        let tiles = TileGroup::try_from("055m".to_string()).unwrap();
        assert_eq!(tiles.value(), FIVE_VALUE);
        assert_eq!(tiles.tiles[0].is_aka(), true);
        assert_eq!(tiles.tiles[1].is_aka(), false);
        assert_eq!(tiles.tiles[2].is_aka(), false);
        assert_eq!(tiles.group_type, GroupType::Triplet);

        let tiles = TileGroup::try_from("406m".to_string()).unwrap();
        assert_eq!(tiles.value(), FOUR_VALUE);
        assert_eq!(tiles.tiles[0].is_aka(), false);
        assert_eq!(tiles.tiles[1].is_aka(), true);
        assert_eq!(tiles.tiles[2].is_aka(), false);
        assert_eq!(tiles.group_type, GroupType::Sequence);
    }

    #[test]
    fn is_not_akadora_from_string() {
        let tiles = TileGroup::try_from("1m".to_string()).unwrap();
        assert_eq!(tiles.value(), ONE_VALUE);
        assert_eq!(tiles.tiles[0].is_aka(), false);
        assert_eq!(tiles.group_type, GroupType::None);
    }

    #[test]
    fn validate_pair_from_string() {
        let tiles = TileGroup::try_from("12m".to_string());
        assert_eq!(tiles.err(), Some(HandErr::InvalidGroup));

        let tiles = TileGroup::try_from("11m".to_string()).unwrap();
        assert_eq!(tiles.group_type, GroupType::Pair);

        let tiles = TileGroup::try_from("05m".to_string()).unwrap();
        assert_eq!(tiles.group_type, GroupType::Pair);
    }

    #[test]
    fn validate_triplet_from_string() {
        let tiles = TileGroup::try_from("112m".to_string());
        assert_eq!(tiles.err(), Some(HandErr::InvalidGroup));

        let tiles = TileGroup::try_from("122m".to_string());
        assert_eq!(tiles.err(), Some(HandErr::InvalidGroup));

        let tiles = TileGroup::try_from("111m".to_string()).unwrap();
        assert_eq!(tiles.group_type, GroupType::Triplet);

        let tiles = TileGroup::try_from("050m".to_string()).unwrap();
        assert_eq!(tiles.group_type, GroupType::Triplet);
    }

    #[test]
    fn validate_sequence_from_string() {
        let tiles = TileGroup::try_from("124m".to_string());
        assert_eq!(tiles.err(), Some(HandErr::InvalidGroup));

        let tiles = TileGroup::try_from("1234m".to_string());
        assert_eq!(tiles.err(), Some(HandErr::InvalidGroup));

        let tiles = TileGroup::try_from("567m".to_string()).unwrap();
        assert_eq!(tiles.group_type, GroupType::Sequence);

        let tiles = TileGroup::try_from("406m".to_string()).unwrap();
        assert_eq!(tiles.group_type, GroupType::Sequence);
    }

    #[test]
    fn validate_kan_from_string() {
        let tiles = TileGroup::try_from("1112m".to_string());
        assert_eq!(tiles.err(), Some(HandErr::InvalidGroup));

        let tiles = TileGroup::try_from("1222m".to_string());
        assert_eq!(tiles.err(), Some(HandErr::InvalidGroup));

        let tiles = TileGroup::try_from("1111m".to_string()).unwrap();
        assert_eq!(tiles.group_type, GroupType::Kan);

        let tiles = TileGroup::try_from("0505m".to_string()).unwrap();
        assert_eq!(tiles.group_type, GroupType::Kan);
    }

    #[test]
    fn non_honor_tilegroup() {
        let group = vec!["1m".to_string().try_into().unwrap()];
        let tiles = TileGroup::new(group, false).unwrap();
        assert_eq!(tiles.suit(), Suit::Manzu);
        assert_eq!(tiles.value(), ONE_VALUE);
        assert!(!tiles.isopen);
        assert_eq!(tiles.group_type, GroupType::None);
        assert!(tiles.is_terminal());

        let group = vec![
            "1m".to_string().try_into().unwrap(),
            "1m".to_string().try_into().unwrap(),
            "1m".to_string().try_into().unwrap(),
        ];
        let tiles = TileGroup::new(group, true).unwrap();
        assert!(tiles.isopen);
        assert_eq!(tiles.group_type, GroupType::Triplet);
        assert_eq!(tiles.suit(), Suit::Manzu);

        let group = vec![
            "1m".to_string().try_into().unwrap(),
            "2m".to_string().try_into().unwrap(),
            "3m".to_string().try_into().unwrap(),
        ];
        let tiles = TileGroup::new(group, false).unwrap();
        assert_eq!(tiles.group_type, GroupType::Sequence);
        assert_eq!(tiles.suit(), Suit::Manzu);
        assert!(tiles.is_terminal());

        let group = vec![
            "2m".to_string().try_into().unwrap(),
            "3m".to_string().try_into().unwrap(),
            "4m".to_string().try_into().unwrap(),
        ];
        let tiles = TileGroup::new(group, false).unwrap();
        assert_eq!(tiles.group_type, GroupType::Sequence);
        assert_eq!(tiles.suit(), Suit::Manzu);
        assert!(!tiles.is_terminal());
    }

    #[test]
    fn wind_tilegroup() {
        let group = vec!["Ew".to_string().try_into().unwrap()];
        let tiles = TileGroup::new(group, false).unwrap();
        assert_eq!(tiles.suit(), Suit::Wind);
        assert_eq!(tiles.value(), EAST_VALUE);
        assert!(!tiles.isopen);
        assert_eq!(tiles.group_type, GroupType::None);
        assert_eq!(tiles.is_terminal(), false);

        let group = vec![
            "2z".to_string().try_into().unwrap(),
            "2z".to_string().try_into().unwrap(),
            "2z".to_string().try_into().unwrap(),
        ];
        let tiles = TileGroup::new(group, true).unwrap();
        assert!(tiles.isopen);
        assert_eq!(tiles.group_type, GroupType::Triplet);
        assert_eq!(tiles.suit(), Suit::Wind);
        assert_eq!(tiles.value(), SOUTH_VALUE);

        let group = vec![
            "Ew".to_string().try_into().unwrap(),
            "Ew".to_string().try_into().unwrap(),
            "Ew".to_string().try_into().unwrap(),
            "Ew".to_string().try_into().unwrap(),
        ];
        let tiles = TileGroup::new(group, false).unwrap();
        assert!(!tiles.isopen);
        assert_eq!(tiles.group_type, GroupType::Kan);
        assert_eq!(tiles.suit(), Suit::Wind);
        assert_eq!(tiles.value(), EAST_VALUE);
    }

    #[test]
    fn dragon_tilesgroup() {
        let group = vec!["5z".to_string().try_into().unwrap()];
        let tiles = TileGroup::new(group, false).unwrap();
        assert_eq!(tiles.suit(), Suit::Dragon);
        assert_eq!(tiles.value(), WHITE_VALUE);
        assert!(!tiles.isopen);
        assert_eq!(tiles.group_type, GroupType::None);

        let group = vec![
            "6z".to_string().try_into().unwrap(),
            "6z".to_string().try_into().unwrap(),
            "6z".to_string().try_into().unwrap(),
        ];
        let tiles = TileGroup::new(group, true).unwrap();
        assert_eq!(tiles.suit(), Suit::Dragon);
        assert_eq!(tiles.value(), GREEN_VALUE);
        assert!(tiles.isopen);
        assert_eq!(tiles.group_type, GroupType::Triplet);

        let group = vec![
            "7z".to_string().try_into().unwrap(),
            "7z".to_string().try_into().unwrap(),
            "7z".to_string().try_into().unwrap(),
            "7z".to_string().try_into().unwrap(),
        ];
        let tiles = TileGroup::new(group, false).unwrap();
        assert!(!tiles.isopen);
        assert_eq!(tiles.group_type, GroupType::Kan);
        assert_eq!(tiles.suit(), Suit::Dragon);
        assert_eq!(tiles.value(), RED_VALUE);
    }

    #[test]
    fn too_large_error() {
        let group = vec![
            "1s".to_string().try_into().unwrap(),
            "1s".to_string().try_into().unwrap(),
            "1s".to_string().try_into().unwrap(),
            "1s".to_string().try_into().unwrap(),
            "1s".to_string().try_into().unwrap(),
        ];
        let tiles = TileGroup::new(group, false);
        assert_eq!(tiles, Err(HandErr::InvalidGroup));
    }

    #[test]
    fn is_akadora() {
        let group = vec!["0m".to_string().try_into().unwrap()];
        let tiles = TileGroup::new(group, false).unwrap();
        assert_eq!(tiles.value(), FIVE_VALUE);
        assert_eq!(tiles.tiles[0].is_aka(), true);
        assert_eq!(tiles.group_type, GroupType::None);

        let group = vec![
            "0s".to_string().try_into().unwrap(),
            "5s".to_string().try_into().unwrap(),
            "5s".to_string().try_into().unwrap(),
        ];
        let tiles = TileGroup::new(group, false).unwrap();
        assert_eq!(tiles.value(), FIVE_VALUE);
        assert_eq!(tiles.tiles[0].is_aka(), true);
        assert_eq!(tiles.tiles[1].is_aka(), false);
        assert_eq!(tiles.tiles[2].is_aka(), false);
        assert_eq!(tiles.group_type, GroupType::Triplet);

        let group = vec![
            "4s".to_string().try_into().unwrap(),
            "0s".to_string().try_into().unwrap(),
            "6s".to_string().try_into().unwrap(),
        ];
        let tiles = TileGroup::new(group, false).unwrap();
        assert_eq!(tiles.value(), FOUR_VALUE);
        assert_eq!(tiles.tiles[0].is_aka(), false);
        assert_eq!(tiles.tiles[1].is_aka(), true);
        assert_eq!(tiles.tiles[2].is_aka(), false);
        assert_eq!(tiles.group_type, GroupType::Sequence);
    }

    #[test]
    fn is_not_akadora() {
        let group = vec!["1m".to_string().try_into().unwrap()];
        let tiles = TileGroup::new(group, false).unwrap();
        assert_eq!(tiles.value(), ONE_VALUE);
        assert_eq!(tiles.tiles[0].is_aka(), false);
        assert_eq!(tiles.group_type, GroupType::None);
    }

    #[test]
    fn validate_pair() {
        let group = vec![
            "1m".to_string().try_into().unwrap(),
            "2m".to_string().try_into().unwrap(),
        ];
        let tiles = TileGroup::new(group, false);
        assert_eq!(tiles.err(), Some(HandErr::InvalidGroup));

        let group = vec![
            "1m".to_string().try_into().unwrap(),
            "1m".to_string().try_into().unwrap(),
        ];
        let tiles = TileGroup::new(group, false).unwrap();
        assert_eq!(tiles.group_type, GroupType::Pair);

        let group = vec![
            "0m".to_string().try_into().unwrap(),
            "5m".to_string().try_into().unwrap(),
        ];
        let tiles = TileGroup::new(group, false).unwrap();
        assert_eq!(tiles.group_type, GroupType::Pair);
    }

    #[test]
    fn validate_triplet() {
        let group = vec![
            "1m".to_string().try_into().unwrap(),
            "1m".to_string().try_into().unwrap(),
            "2m".to_string().try_into().unwrap(),
        ];
        let tiles = TileGroup::new(group, false);
        assert_eq!(tiles.err(), Some(HandErr::InvalidGroup));

        let group = vec![
            "1m".to_string().try_into().unwrap(),
            "2m".to_string().try_into().unwrap(),
            "2m".to_string().try_into().unwrap(),
        ];
        let tiles = TileGroup::new(group, false);
        assert_eq!(tiles.err(), Some(HandErr::InvalidGroup));

        let group = vec![
            "1m".to_string().try_into().unwrap(),
            "1m".to_string().try_into().unwrap(),
            "1m".to_string().try_into().unwrap(),
        ];
        let tiles = TileGroup::new(group, false).unwrap();
        assert_eq!(tiles.group_type, GroupType::Triplet);

        let group = vec![
            "0m".to_string().try_into().unwrap(),
            "5m".to_string().try_into().unwrap(),
            "0m".to_string().try_into().unwrap(),
        ];
        let tiles = TileGroup::new(group, false).unwrap();
        assert_eq!(tiles.group_type, GroupType::Triplet);
    }

    #[test]
    fn validate_sequence() {
        let group = vec![
            "1m".to_string().try_into().unwrap(),
            "2m".to_string().try_into().unwrap(),
            "4m".to_string().try_into().unwrap(),
        ];
        let tiles = TileGroup::new(group, false);
        assert_eq!(tiles.err(), Some(HandErr::InvalidGroup));

        let group = vec![
            "1m".to_string().try_into().unwrap(),
            "2m".to_string().try_into().unwrap(),
            "3m".to_string().try_into().unwrap(),
            "4m".to_string().try_into().unwrap(),
        ];
        let tiles = TileGroup::new(group, false);
        assert_eq!(tiles.err(), Some(HandErr::InvalidGroup));

        let group = vec![
            "5m".to_string().try_into().unwrap(),
            "6m".to_string().try_into().unwrap(),
            "7m".to_string().try_into().unwrap(),
        ];
        let tiles = TileGroup::new(group, false).unwrap();
        assert_eq!(tiles.group_type, GroupType::Sequence);

        let group = vec![
            "4m".to_string().try_into().unwrap(),
            "0m".to_string().try_into().unwrap(),
            "6m".to_string().try_into().unwrap(),
        ];
        let tiles = TileGroup::new(group, false).unwrap();
        assert_eq!(tiles.group_type, GroupType::Sequence);

        let group = vec![
            "Ew".to_string().try_into().unwrap(),
            "Sw".to_string().try_into().unwrap(),
            "Ww".to_string().try_into().unwrap(),
        ];
        let tiles = TileGroup::new(group, false).unwrap_err();
        assert_eq!(tiles, HandErr::InvalidGroup);

        let group = vec![
            "8m".to_string().try_into().unwrap(),
            "9m".to_string().try_into().unwrap(),
            "1m".to_string().try_into().unwrap(),
        ];
        let tiles = TileGroup::new(group, false).unwrap_err();
        assert_eq!(tiles, HandErr::InvalidGroup);
    }

    #[test]
    fn validate_kan() {
        let group = vec![
            "1m".to_string().try_into().unwrap(),
            "1m".to_string().try_into().unwrap(),
            "1m".to_string().try_into().unwrap(),
            "2m".to_string().try_into().unwrap(),
        ];
        let tiles = TileGroup::new(group, false);
        assert_eq!(tiles.err(), Some(HandErr::InvalidGroup));

        let group = vec![
            "1m".to_string().try_into().unwrap(),
            "2m".to_string().try_into().unwrap(),
            "2m".to_string().try_into().unwrap(),
            "2m".to_string().try_into().unwrap(),
        ];
        let tiles = TileGroup::new(group, false);
        assert_eq!(tiles.err(), Some(HandErr::InvalidGroup));

        let group = vec![
            "2m".to_string().try_into().unwrap(),
            "2m".to_string().try_into().unwrap(),
            "2m".to_string().try_into().unwrap(),
            "2m".to_string().try_into().unwrap(),
        ];
        let tiles = TileGroup::new(group, false).unwrap();
        assert_eq!(tiles.group_type, GroupType::Kan);

        let group = vec![
            "0m".to_string().try_into().unwrap(),
            "5m".to_string().try_into().unwrap(),
            "0m".to_string().try_into().unwrap(),
            "5m".to_string().try_into().unwrap(),
        ];
        let tiles = TileGroup::new(group, false).unwrap();
        assert_eq!(tiles.group_type, GroupType::Kan);
    }
}
