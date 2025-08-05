use std::ffi::{CStr, CString, c_char};

use crate::{
    calc::{error::CalcErr, get_hand_score},
    fu::Fu,
    hand::{Hand, error::HandErr},
    payment::Points,
    score::{FuValue, HanValue},
    tile_group::TileGroup,
    yaku::Yaku,
};

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
pub const MAX_DORA_TILE_COUNT: usize = 13;

#[repr(C)]
pub struct HandShapes {
    pub hands: [HandShape; MAX_HAND_SHAPES],
    pub hands_len: usize,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
pub struct HandShape {
    pub groups: [CTileGroup; MAX_GROUPS_PER_HAND],
    pub group_count: usize,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
pub struct CTileGroup {
    tiles: [tile::Tile; 4],
    tiles_len: usize,
    isopen: bool,
    group_type: tile_group::GroupType,
}

#[repr(C)]
#[derive(Debug)]
pub struct Conditions {
    handshape: HandShape,
    win_tile: tile::Tile,
    seat_wind: tile::Tile,
    prev_wind: tile::Tile,
    dora_tiles: *const tile::Tile,
    dora_tiles_len: usize,
    winning_group_idx: usize,
    tsumo: bool,
    riichi: bool,
    double_riichi: bool,
    ippatsu: bool,
    haitei: bool,
    chankan: bool,
    rinshan: bool,
    tenhou: bool,
    honba: u8,
}

#[repr(C)]
#[derive(Debug)]
pub enum FfiResult {
    Ok,
    Err(CalcErr),
}

#[repr(C)]
#[derive(Debug)]
pub struct ScoreResult {
    error: FfiResult,
    score_info: ScoreInfo,
}

#[repr(C)]
#[derive(Debug)]
pub struct ScoreInfo {
    yaku: *const Yaku,
    yaku_len: usize,
    fu: *const Fu,
    fu_len: usize,
    han_score: HanValue,
    fu_score: FuValue,
    dealer_ron: Points,
    dealer_tsumo: Points,
    non_dealer_ron: Points,
    non_dealer_tsumo_dealer: Points,
    non_dealer_tsumo_non_dealer: Points,
}

#[repr(C)]
#[derive(Debug)]
pub struct ScoreYaku {
    yaku: *const Yaku,
    yaku_count: usize,
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

        let default_hand_shape = HandShape {
            groups: [CTileGroup::default(); MAX_GROUPS_PER_HAND],
            group_count: 0,
        };

        let mut hands = [default_hand_shape; MAX_HAND_SHAPES];

        for (hand_idx, hand) in handshapes.iter().enumerate() {
            if hand.len() > MAX_GROUPS_PER_HAND {
                return None;
            }
            hands[hand_idx].group_count = hand.len();
            for (group_idx, group) in hand.iter().enumerate() {
                hands[hand_idx].groups[group_idx] = CTileGroup::try_from(group.clone()).ok()?;
            }
        }

        let result = HandShapes {
            hands,
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
        x.try_into().unwrap()
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
        Ok(out)
    }
}

impl From<CTileGroup> for tile_group::TileGroup {
    fn from(val: CTileGroup) -> Self {
        TileGroup::new(val.tiles.to_vec()[0..val.tiles_len].to_vec(), val.isopen).unwrap()
    }
}

#[unsafe(no_mangle)]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn C_free_hand_shapes(ptr: *mut HandShapes) {
    unsafe {
        if !ptr.is_null() {
            let _ = Box::from_raw(ptr);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn C_get_hand_score(conditions: Conditions) -> *mut ScoreResult {
    fn get_score_internal(conditions: Conditions) -> Result<Box<score::Score>, CalcErr> {
        let dora_slice =
            unsafe { std::slice::from_raw_parts(conditions.dora_tiles, conditions.dora_tiles_len) };

        let mut tile_groups: Vec<TileGroup> = vec![];
        for (idx, group) in conditions.handshape.groups[0..conditions.handshape.group_count]
            .iter()
            .enumerate()
        {
            if idx != conditions.winning_group_idx {
                tile_groups.push((*group).into());
            }
        }
        tile_groups.push(conditions.handshape.groups[conditions.winning_group_idx].into());

        let hand = Hand::new(
            tile_groups,
            conditions.win_tile,
            conditions.seat_wind,
            conditions.prev_wind,
        )
        .map_err(CalcErr::HandErr)?;

        get_hand_score(
            &hand,
            &Some(dora_slice.to_vec()),
            conditions.tsumo,
            conditions.riichi,
            conditions.double_riichi,
            conditions.ippatsu,
            conditions.haitei,
            conditions.rinshan,
            conditions.chankan,
            conditions.tenhou,
            conditions.honba.into(),
        )
        .map(Box::new)
    }

    let result = match get_score_internal(conditions) {
        Ok(s) => ScoreResult {
            error: FfiResult::Ok,
            score_info: ScoreInfo {
                yaku: s.yaku().as_ptr(),
                yaku_len: s.yaku().len(),
                fu: s.fu().as_ptr(),
                fu_len: s.fu().len(),
                han_score: s.han(),
                fu_score: s.fu_score(),
                dealer_ron: s.payment().dealer_ron(s.honba()),
                dealer_tsumo: s.payment().dealer_tsumo(s.honba()),
                non_dealer_ron: s.payment().non_dealer_ron(s.honba()),
                non_dealer_tsumo_dealer: s.payment().non_dealer_tsumo_to_dealer(s.honba()),
                non_dealer_tsumo_non_dealer: s.payment().non_dealer_tsumo_to_non_dealer(s.honba()),
            },
        },
        Err(e) => ScoreResult {
            error: FfiResult::Err(e),
            score_info: ScoreInfo {
                yaku: [].as_ptr(),
                yaku_len: 0,
                fu: [].as_ptr(),
                fu_len: 0,
                han_score: 0,
                fu_score: 0,
                dealer_ron: 0,
                dealer_tsumo: 0,
                non_dealer_ron: 0,
                non_dealer_tsumo_dealer: 0,
                non_dealer_tsumo_non_dealer: 0,
            },
        },
    };

    Box::into_raw(Box::new(result))
}

#[unsafe(no_mangle)]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn C_free_score_result(result: *mut ScoreResult) {
    unsafe {
        if result.is_null() {
            return;
        }
        let _ = Box::from_raw(result);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn C_get_err_message_from_result(ffi_result: FfiResult) -> *mut c_char {
    match ffi_result {
        FfiResult::Ok => CString::new("OK").unwrap().into_raw(),
        FfiResult::Err(calc_err) => CString::new(calc_err.to_string()).unwrap().into_raw(),
    }
}

#[unsafe(no_mangle)]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn C_free_c_string(result: *mut c_char) {
    unsafe {
        if result.is_null() {
            return;
        }
        let _ = Box::from_raw(result);
    }
}
