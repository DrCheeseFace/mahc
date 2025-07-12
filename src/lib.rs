use std::ffi::{CStr, c_char};

use crate::{hand::error::HandErr, tile_group::TileGroup};

pub mod calc;
pub mod fu;
pub mod hand;
pub mod limit_hand;
pub mod payment;
pub mod score;
pub mod suit;
pub mod tile;
pub mod tile_group;
pub mod yaku;

const VALID_SEQUENCE_VALUES: &[&str] = &[
    "123", "234", "345", "456", "567", "678", "789", "340", "406", "067",
];

pub const MAX_HAND_SHAPES: usize = 4;
pub const MAX_GROUPS_PER_HAND: usize = 14;

#[repr(C)]
pub struct HandShapes {
    pub hands: [[CTileGroup; MAX_GROUPS_PER_HAND]; MAX_HAND_SHAPES],
    pub hands_len: usize,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
pub struct CTileGroup {
    tiles: [tile::Tile; 4],
    tiles_len: usize,
    isopen: bool,
    group_type: tile_group::GroupType,
}

#[unsafe(no_mangle)]
pub extern "C" fn C_get_valid_hand_shapes(tiles_string: *const c_char) -> *mut HandShapes {
    fn get_shapes_internal(tiles_string: *const c_char) -> Option<Box<HandShapes>> {
        let tiles_c_str = unsafe {
            if tiles_string.is_null() {
                return None;
            }
            CStr::from_ptr(tiles_string)
        };
        let tiles_str = tiles_c_str.to_str().ok()?;

        let tiles: Vec<tile::Tile> = tiles_str
            .split_whitespace()
            .filter_map(|s| s.to_string().try_into().ok())
            .collect();

        let handshapes = calc::get_valid_hand_shapes(&tiles);
        if handshapes.is_empty() || handshapes.len() > MAX_HAND_SHAPES {
            return None;
        }

        let mut c_hands = [[CTileGroup::default(); MAX_GROUPS_PER_HAND]; MAX_HAND_SHAPES];

        for (hand_idx, hand) in handshapes.iter().enumerate() {
            if hand.len() > MAX_GROUPS_PER_HAND {
                return None;
            }
            for (group_idx, group) in hand.iter().enumerate() {
                c_hands[hand_idx][group_idx] = CTileGroup::try_from(group.clone()).ok()?;
            }
        }

        let result = HandShapes {
            hands: c_hands,
            hands_len: handshapes.len(),
        };
        Some(Box::new(result))
    }

    match get_shapes_internal(tiles_string) {
        Some(boxed_result) => Box::into_raw(boxed_result),
        None => std::ptr::null_mut(),
    }
}

impl CTileGroup {
    fn default() -> CTileGroup {
        let x: TileGroup =
            TileGroup::new(["1m".to_string().try_into().unwrap()].to_vec(), false).unwrap(); //holyguacamoly
        return x.try_into().unwrap();
    }
}

impl TryFrom<tile_group::TileGroup> for CTileGroup {
    type Error = HandErr;

    fn try_from(value: tile_group::TileGroup) -> Result<Self, Self::Error> {
        let placeholder_tile: tile::Tile = "1s".to_string().try_into()?;
        let mut tiles: [tile::Tile; 4] = [
            placeholder_tile,
            placeholder_tile,
            placeholder_tile,
            placeholder_tile,
        ];

        for tile_idx in 0..value.tiles().len() {
            tiles[tile_idx] = *value.tiles()[tile_idx];
        }

        let out: CTileGroup = CTileGroup {
            tiles,
            tiles_len: value.tiles().len(),
            isopen: value.isopen(),
            group_type: value.group_type(),
        };
        return Ok(out);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn C_free_hand_shapes(ptr: *mut HandShapes) {
    if !ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(ptr);
        }
    }
}
