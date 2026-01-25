pub mod error;
mod utils;

use crate::fu::{Fu, calculate_total_fu_value};
use crate::hand::Hand;
use crate::limit_hand::LimitHands;
use crate::payment::Payment;
use crate::score::{FuValue, HanValue, HonbaCounter, Score};
use crate::tile::{DValue, MpsValue, Tile, WValue};
use crate::tile_group::TileGroup;
use crate::yaku::Yaku;
use error::CalcErr;
use std::collections::HashMap;

/// Get the score breakdown of the hand.
#[allow(clippy::too_many_arguments)]
pub fn get_hand_score(
    hand: &Hand,
    dora: &Option<Vec<Tile>>,
    tsumo: bool,
    riichi: bool,
    doubleriichi: bool,
    ippatsu: bool,
    haitei: bool,
    rinshan: bool,
    chankan: bool,
    tenhou: bool,
    honba: HonbaCounter,
) -> Result<Score, CalcErr> {
    if let Some(t) = validate_scoring_conditions(
        hand,
        tsumo,
        riichi,
        doubleriichi,
        ippatsu,
        haitei,
        rinshan,
        chankan,
    ) {
        return Err(t);
    }

    let yaku_and_yakuman = get_yaku_and_yakuman(
        hand,
        tsumo,
        riichi,
        doubleriichi,
        ippatsu,
        haitei,
        rinshan,
        chankan,
        tenhou,
    );
    if yaku_and_yakuman.is_empty() {
        return Err(CalcErr::NoYaku);
    }

    let scoring_yaku: Vec<Yaku> = if yaku_and_yakuman.iter().any(|yaku| yaku.is_yakuman()) {
        yaku_and_yakuman
            .iter()
            .filter(|yaku| yaku.is_yakuman())
            .copied()
            .collect()
    } else {
        yaku_and_yakuman
            .iter()
            .filter(|yaku| !yaku.is_yakuman())
            .copied()
            .collect()
    };

    let fu_types = hand.calculate_fu(tsumo);
    let fu_value = calculate_total_fu_value(&fu_types);

    // get han from dora tiles
    let mut scoring_han: HanValue = scoring_yaku
        .iter()
        .map(|yaku| yaku.get_han(hand.is_open()))
        .sum::<HanValue>();
    let dora_count = hand.get_dora_count(dora);
    if !scoring_yaku[0].is_yakuman() {
        scoring_han += dora_count
    }

    let payment = calculate_yaku_payment(&yaku_and_yakuman, &fu_types, dora, hand)?;

    let score = Score::new(
        payment,
        scoring_yaku,
        fu_types,
        scoring_han,
        fu_value,
        honba,
        hand.is_open(),
        dora_count,
    );
    Ok(score)
}

// Get list of yaku and yakuman from hand
#[allow(clippy::too_many_arguments)]
pub fn get_yaku_and_yakuman(
    hand: &Hand,
    tsumo: bool,
    riichi: bool,
    doubleriichi: bool,
    ippatsu: bool,
    haitei: bool,
    rinshan: bool,
    chankan: bool,
    tenhou: bool,
) -> Vec<Yaku> {
    let mut conditions: Vec<Yaku> = vec![];
    for yakuman in get_yakuman(hand, tsumo, tenhou) {
        conditions.push(yakuman);
    }
    for yaku in get_yaku(
        hand,
        tsumo,
        riichi,
        doubleriichi,
        ippatsu,
        haitei,
        rinshan,
        chankan,
    ) {
        conditions.push(yaku);
    }
    conditions
}

// Get list of yakuman from hand
pub fn get_yakuman(hand: &Hand, tsumo: bool, tenhou: bool) -> Vec<Yaku> {
    let mut yakuman: Vec<Yaku> = vec![];
    let yakumanconditions = [
        (hand.is_daisangen(), Yaku::Daisangen),
        (hand.is_suuankou(tsumo), Yaku::Suuankou),
        (hand.is_suuankoutankiwait(), Yaku::SuuankouTankiWait),
        (hand.is_chinroutou(), Yaku::Chinroutou),
        (hand.is_ryuuiisou(), Yaku::Ryuuiisou),
        (hand.is_chuurenpoutou(), Yaku::ChuurenPoutou),
        (hand.is_chuurenpoutou9sided(), Yaku::ChuurenPoutou9SidedWait),
        (hand.is_tsuuiisou(), Yaku::Tsuuiisou),
        (hand.is_daichiishin(), Yaku::Daichiishin),
        (hand.is_suukantsu(), Yaku::Suukantsu),
        (hand.is_shousuushii(), Yaku::Shousuushii),
        (hand.is_daisuushii(), Yaku::Daisuushii),
        (hand.is_kokushi(), Yaku::KokushiMusou),
        (hand.is_kokushi13sided(), Yaku::KokushiMusou13SidedWait),
        (hand.is_tenhou(tenhou), Yaku::Tenhou),
        (hand.is_chiihou(tenhou), Yaku::Chiihou),
    ];

    for (condition, yaku_type) in yakumanconditions {
        if condition {
            yakuman.push(yaku_type);
        }
    }
    yakuman
}

// Get list of yaku from hand
#[allow(clippy::too_many_arguments)]
pub fn get_yaku(
    hand: &Hand,
    tsumo: bool,
    riichi: bool,
    doubleriichi: bool,
    ippatsu: bool,
    haitei: bool,
    rinshan: bool,
    chankan: bool,
) -> Vec<Yaku> {
    let mut yaku: Vec<Yaku> = vec![];
    let conditions = [
        (riichi, Yaku::Riichi),
        (doubleriichi, Yaku::DoubleRiichi),
        (ippatsu, Yaku::Ippatsu),
        (haitei, Yaku::Haitei),
        (rinshan, Yaku::RinshanKaihou),
        (chankan, Yaku::Chankan),
        (hand.is_tanyao(), Yaku::Tanyao),
        (hand.is_iipeikou(), Yaku::Iipeikou),
        (hand.is_ryanpeikou(), Yaku::Ryanpeikou),
        (hand.is_toitoi(), Yaku::Toitoi),
        (hand.is_sanshokudoujun(), Yaku::SanshokuDoujun),
        (hand.is_sanankou(tsumo), Yaku::Sanankou),
        (hand.is_honitsu(), Yaku::Honitsu),
        (hand.is_shousangen(), Yaku::Shousangen),
        (hand.is_junchantaiyao(), Yaku::JunchanTaiyao),
        (hand.is_honroutou(), Yaku::Honroutou),
        (hand.is_sankantsu(), Yaku::Sankantsu),
        (hand.is_ittsuu(), Yaku::Ittsuu),
        (hand.is_chantaiyao(), Yaku::Chantaiyao),
        (hand.is_chiitoitsu(), Yaku::Chiitoitsu),
        (hand.is_menzentsumo(tsumo), Yaku::MenzenTsumo),
        (hand.is_pinfu(), Yaku::Pinfu),
        (hand.is_sanshokudoukou(), Yaku::SanshokuDoukou),
        (hand.is_chinitsu(), Yaku::Chinitsu),
    ];
    for (condition, yaku_type) in conditions {
        if condition {
            yaku.push(yaku_type);
        }
    }

    for _i in 0..hand.is_yakuhai() {
        yaku.push(Yaku::Yakuhai);
    }

    yaku
}

/// Calculate the payment amounts from the list of yaku.
pub fn calculate_yaku_payment(
    yaku: &Vec<Yaku>,
    fu: &[Fu],
    dora: &Option<Vec<Tile>>,
    hand: &Hand,
) -> Result<Payment, CalcErr> {
    let mut yakuman_count = 0;
    let mut han = 0;
    for y in yaku {
        if y.is_yakuman() {
            yakuman_count += y.get_han(hand.is_open());
        } else {
            han += y.get_han(hand.is_open());
        }
    }

    if yakuman_count > 0 {
        let basepoints: u64 = (8_000 * yakuman_count).into();
        let payment = Payment::new(basepoints);
        return Ok(payment);
    }

    if han == 0 {
        return Err(CalcErr::NoYaku);
    }

    han += hand.get_dora_count(dora);
    let fu_value: FuValue = calculate_total_fu_value(fu);
    calculate(&han, &fu_value)
}

/// Calculate the payment amounts from the han, fu, and number of honba (repeat counters).
pub fn calculate(han: &HanValue, fu: &FuValue) -> Result<Payment, CalcErr> {
    if *han == 0 {
        return Err(CalcErr::NoHan);
    }

    if *fu == 0 {
        return Err(CalcErr::NoFu);
    }

    let k = LimitHands::get_limit_hand(*han, *fu);
    if let Some(limithand) = k {
        let payment = limithand.get_score();

        return Ok(payment);
    }

    let payment = Payment::from_han_and_fu(*han, *fu);

    Ok(payment)
}

/// Checks for invalid scoring conditions
#[allow(clippy::too_many_arguments)]
pub fn validate_scoring_conditions(
    hand: &Hand,
    tsumo: bool,
    riichi: bool,
    doubleriichi: bool,
    ippatsu: bool,
    haitei: bool,
    rinshan: bool,
    chankan: bool,
) -> Option<CalcErr> {
    if tsumo && chankan {
        return Some(CalcErr::ChankanTsumo);
    }
    if rinshan && (!tsumo) {
        return Some(CalcErr::RinshanWithoutTsumo);
    }
    if rinshan && ippatsu {
        return Some(CalcErr::RinshanIppatsu);
    }
    if riichi && doubleriichi {
        return Some(CalcErr::DuplicateRiichi);
    }
    if ippatsu && !(riichi || doubleriichi) {
        return Some(CalcErr::IppatsuWithoutRiichi);
    }
    if doubleriichi && ippatsu && haitei {
        return Some(CalcErr::DoubleRiichiHaiteiIppatsu);
    }
    if doubleriichi && haitei && chankan {
        return Some(CalcErr::DoubleRiichiHaiteiChankan);
    }
    if hand.kans().is_empty() && rinshan {
        return Some(CalcErr::RinshanKanWithoutKan);
    }
    None
}

pub fn get_valid_hand_shapes(tiles: &[Tile]) -> Vec<Vec<TileGroup>> {
    if tiles.len() > 18 || tiles.len() < 14 {
        return vec![];
    }

    let mut tile_counts = get_tile_counts(tiles);
    let mut all_shapes: Vec<Vec<TileGroup>> = Vec::new();

    if tiles.len() == 14
        && let Some(hand) = check_seven_pairs(&tile_counts)
    {
        all_shapes.push(hand);
    }

    if tiles.len() == 14
        && let Some(hand) = check_thirteen_orphans(&tile_counts)
    {
        all_shapes.push(hand);
    }

    find_standard_shapes(&mut tile_counts, &mut all_shapes);

    all_shapes.retain(|shape| shape.len() == 5 || shape.len() == 13 || shape.len() == 7);

    for shape in &mut all_shapes {
        shape.sort();
    }
    all_shapes.sort();
    all_shapes.dedup();

    all_shapes
        .into_iter()
        .map(|shape| materialize_shape(&shape, tiles))
        .collect()
}

fn get_tile_counts(tiles: &[Tile]) -> HashMap<Tile, u8> {
    let mut counts = HashMap::new();
    for tile in tiles {
        *counts.entry(tile.normalized()).or_insert(0) += 1;
    }
    counts
}

fn check_seven_pairs(counts: &HashMap<Tile, u8>) -> Option<Vec<TileGroup>> {
    if counts.len() == 7 && counts.values().all(|&c| c == 2) {
        let hand = counts
            .keys()
            .map(|&tile| TileGroup::new(vec![tile, tile], false).unwrap())
            .collect();
        Some(hand)
    } else {
        None
    }
}

fn check_thirteen_orphans(counts: &HashMap<Tile, u8>) -> Option<Vec<TileGroup>> {
    let required_tiles = [
        Tile::Pin(MpsValue::One),
        Tile::Pin(MpsValue::Nine),
        Tile::Sou(MpsValue::One),
        Tile::Sou(MpsValue::Nine),
        Tile::Man(MpsValue::One),
        Tile::Man(MpsValue::Nine),
        Tile::Dragon(DValue::Red),
        Tile::Dragon(DValue::Green),
        Tile::Dragon(DValue::White),
        Tile::Wind(WValue::East),
        Tile::Wind(WValue::South),
        Tile::Wind(WValue::West),
        Tile::Wind(WValue::North),
    ];
    let mut has_pair = false;
    let mut is_kokushi = true;

    for &tile in &required_tiles {
        match counts.get(&tile) {
            Some(1) => {}
            Some(2) => {
                if has_pair {
                    is_kokushi = false;
                    break;
                }
                has_pair = true;
            }
            _ => {
                is_kokushi = false;
                break;
            }
        }
    }

    if is_kokushi && has_pair {
        let mut hand = Vec::new();
        for &tile in &required_tiles {
            if *counts.get(&tile).unwrap_or(&0) == 2 {
                hand.push(TileGroup::new(vec![tile, tile], false).unwrap());
            } else {
                hand.push(TileGroup::new(vec![tile], false).unwrap());
            }
        }
        return Some(hand);
    }

    None
}

fn find_standard_shapes(tile_counts: &mut HashMap<Tile, u8>, all_shapes: &mut Vec<Vec<TileGroup>>) {
    let mut sorted_tiles: Vec<_> = tile_counts.keys().copied().collect();
    sorted_tiles.sort();

    for &tile in &sorted_tiles {
        if *tile_counts.get(&tile).unwrap_or(&0) >= 2 {
            let pair_group = TileGroup::new(vec![tile, tile], false).unwrap();

            tile_counts.entry(tile).and_modify(|c| *c -= 2);

            let mut current_hand = vec![pair_group];
            find_melds_recursive(tile_counts, &mut current_hand, all_shapes);

            tile_counts.entry(tile).and_modify(|c| *c += 2);
        }
    }
}

fn find_melds_recursive(
    tile_counts: &mut HashMap<Tile, u8>,
    current_hand: &mut Vec<TileGroup>,
    all_shapes: &mut Vec<Vec<TileGroup>>,
) {
    if tile_counts.values().all(|&c| c == 0) {
        all_shapes.push(current_hand.clone());
        return;
    }

    let mut sorted_tiles: Vec<_> = tile_counts.keys().copied().collect();
    sorted_tiles.sort();
    let first_tile = match sorted_tiles
        .iter()
        .find(|t| *tile_counts.get(t).unwrap_or(&0) > 0)
    {
        Some(&t) => t,
        None => return,
    };

    if *tile_counts.get(&first_tile).unwrap_or(&0) >= 4 {
        let triplet = TileGroup::new(vec![first_tile; 4], false).unwrap();
        current_hand.push(triplet);
        tile_counts.entry(first_tile).and_modify(|c| *c -= 4);

        find_melds_recursive(tile_counts, current_hand, all_shapes);

        tile_counts.entry(first_tile).and_modify(|c| *c += 4);
        current_hand.pop();
    }
    if *tile_counts.get(&first_tile).unwrap_or(&0) >= 3 {
        let triplet = TileGroup::new(vec![first_tile; 3], false).unwrap();
        current_hand.push(triplet);
        tile_counts.entry(first_tile).and_modify(|c| *c -= 3);

        find_melds_recursive(tile_counts, current_hand, all_shapes);

        tile_counts.entry(first_tile).and_modify(|c| *c += 3);
        current_hand.pop();
    }

    if !first_tile.is_honor() {
        let (t2, t3) = (first_tile.get_next(), first_tile.get_next().get_next());
        if *tile_counts.get(&t2).unwrap_or(&0) >= 1 && *tile_counts.get(&t3).unwrap_or(&0) >= 1 {
            let sequence = TileGroup::new(vec![first_tile, t2, t3], false).unwrap();
            current_hand.push(sequence);
            tile_counts.entry(first_tile).and_modify(|c| *c -= 1);
            tile_counts.entry(t2).and_modify(|c| *c -= 1);
            tile_counts.entry(t3).and_modify(|c| *c -= 1);

            find_melds_recursive(tile_counts, current_hand, all_shapes);

            tile_counts.entry(first_tile).and_modify(|c| *c += 1);
            tile_counts.entry(t2).and_modify(|c| *c += 1);
            tile_counts.entry(t3).and_modify(|c| *c += 1);
            current_hand.pop();
        }
    }
}

// this is for handling aka mostly
fn materialize_shape(shape: &[TileGroup], original_tiles: &[Tile]) -> Vec<TileGroup> {
    let mut available: HashMap<Tile, u8> = HashMap::new();
    for t in original_tiles {
        *available.entry(*t).or_insert(0) += 1;
    }

    shape
        .iter()
        .map(|group| {
            let mut tiles = Vec::new();

            for t in group.tiles() {
                // aka first
                let candidates = match t {
                    Tile::Man(MpsValue::Five) => {
                        [Tile::Man(MpsValue::AkaFive), Tile::Man(MpsValue::Five)]
                    }
                    Tile::Pin(MpsValue::Five) => {
                        [Tile::Pin(MpsValue::AkaFive), Tile::Pin(MpsValue::Five)]
                    }
                    Tile::Sou(MpsValue::Five) => {
                        [Tile::Sou(MpsValue::AkaFive), Tile::Sou(MpsValue::Five)]
                    }
                    _ => [*t, *t],
                };

                let chosen = candidates
                    .into_iter()
                    .find(|c| available.get(c).copied().unwrap_or(0) > 0)
                    .unwrap();

                *available.get_mut(&chosen).unwrap() -= 1;
                tiles.push(chosen);
            }

            TileGroup::new(tiles, group.isopen()).unwrap()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{
        calc::{
            error::CalcErr,
            get_hand_score, get_valid_hand_shapes,
            utils::{get_kans, get_pairs, get_sequences, get_singles, get_triplets},
        },
        hand::Hand,
        tile::{MpsValue, Tile},
    };

    use super::validate_scoring_conditions;

    #[test]
    fn yakuman_scoring() {
        let out = Hand::new_from_strings(
            vec![
                "EEEEw".to_string(),
                "SSSw".to_string(),
                "WWWw".to_string(),
                "NNNw".to_string(),
                "99s".to_string(),
            ],
            "9s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_daisuushii());
        let dora: Tile = "Ew".to_string().try_into().unwrap();
        let score = get_hand_score(
            &out,
            &Some(vec![dora]),
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            3,
        )
        .unwrap();
        assert_eq!(score.honba(), 3);
        assert_eq!(score.dora_count(), 3);
        assert_eq!(score.yaku().len(), 3);
        assert_eq!(score.han(), 4);
        assert_eq!(score.payment().dealer_ron(score.honba()), 192_900)
    }

    #[test]
    fn yaku_limit_scoring() {
        let out = Hand::new_from_strings(
            vec![
                "234p".to_string(),
                "678s".to_string(),
                "345m".to_string(),
                "44s".to_string(),
                "345s".to_string(),
            ],
            "5s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        let dora: Tile = "3s".to_string().try_into().unwrap();
        let score = get_hand_score(
            &out,
            &Some(vec![dora]),
            true,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            1,
        )
        .unwrap();
        assert_eq!(score.honba(), 1);
        assert_eq!(score.dora_count(), 3);
        assert_eq!(score.yaku().len(), 3);
        assert_eq!(score.han(), 6);
        assert_eq!(score.payment().dealer_ron(score.honba()), 18_300)
    }

    #[test]
    fn yaku_nonlimit_scoring() {
        let out = Hand::new_from_strings(
            vec![
                "234p".to_string(),
                "678s".to_string(),
                "345m".to_string(),
                "44s".to_string(),
                "345s".to_string(),
            ],
            "5s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        let score = get_hand_score(
            &out, &None, true, false, false, false, false, false, false, false, 1,
        )
        .unwrap();
        assert_eq!(score.honba(), 1);
        assert_eq!(score.dora_count(), 0);
        assert_eq!(score.yaku().len(), 3);
        assert_eq!(score.han(), 3);
        assert_eq!(score.payment().dealer_tsumo(score.honba()), 1400)
    }

    #[test]
    fn validate_scoring_conditions_rinshankan_without_kan() {
        let hand = Hand::new_from_strings(
            vec![
                "123p".to_string(),
                "505s".to_string(),
                "EEEw".to_string(),
                "999m".to_string(),
                "rrd".to_string(),
            ],
            "rd".to_string(),
            "Ew".to_string(),
            "Ew".to_string(),
        )
        .unwrap();
        let actual =
            validate_scoring_conditions(&hand, true, false, false, false, false, true, false)
                .unwrap();
        assert_eq!(CalcErr::RinshanKanWithoutKan, actual)
    }

    #[test]
    fn validate_scoring_conditions_chankan_tsumo() {
        let hand = Hand::new_from_strings(
            vec![
                "123p".to_string(),
                "505s".to_string(),
                "EEEw".to_string(),
                "999m".to_string(),
                "rrd".to_string(),
            ],
            "rd".to_string(),
            "Ew".to_string(),
            "Ew".to_string(),
        )
        .unwrap();
        let actual =
            validate_scoring_conditions(&hand, true, false, false, false, false, false, true)
                .unwrap();
        assert_eq!(CalcErr::ChankanTsumo, actual)
    }
    #[test]
    fn validate_scoring_conditions_rinshan_without_tsumo() {
        let hand = Hand::new_from_strings(
            vec![
                "123p".to_string(),
                "505s".to_string(),
                "EEEw".to_string(),
                "999m".to_string(),
                "rrd".to_string(),
            ],
            "rd".to_string(),
            "Ew".to_string(),
            "Ew".to_string(),
        )
        .unwrap();
        let actual =
            validate_scoring_conditions(&hand, false, false, false, false, false, true, false)
                .unwrap();
        assert_eq!(CalcErr::RinshanWithoutTsumo, actual)
    }

    #[test]
    fn validate_scoring_conditions_rinshan_ippatsu() {
        let hand = Hand::new_from_strings(
            vec![
                "123p".to_string(),
                "505s".to_string(),
                "EEEw".to_string(),
                "999m".to_string(),
                "rrd".to_string(),
            ],
            "rd".to_string(),
            "Ew".to_string(),
            "Ew".to_string(),
        )
        .unwrap();
        let actual =
            validate_scoring_conditions(&hand, true, false, false, true, false, true, false)
                .unwrap();
        assert_eq!(CalcErr::RinshanIppatsu, actual)
    }

    #[test]
    fn validate_scoring_conditions_double_riichi() {
        let hand = Hand::new_from_strings(
            vec![
                "123p".to_string(),
                "505s".to_string(),
                "EEEw".to_string(),
                "999m".to_string(),
                "rrd".to_string(),
            ],
            "rd".to_string(),
            "Ew".to_string(),
            "Ew".to_string(),
        )
        .unwrap();
        let actual =
            validate_scoring_conditions(&hand, false, true, true, true, false, false, false)
                .unwrap();
        assert_eq!(CalcErr::DuplicateRiichi, actual)
    }

    #[test]
    fn validate_scoring_conditions_ippatsu_without_riichi() {
        let hand = Hand::new_from_strings(
            vec![
                "123p".to_string(),
                "505s".to_string(),
                "EEEw".to_string(),
                "999m".to_string(),
                "rrd".to_string(),
            ],
            "rd".to_string(),
            "Ew".to_string(),
            "Ew".to_string(),
        )
        .unwrap();
        let actual =
            validate_scoring_conditions(&hand, false, false, false, true, false, false, false)
                .unwrap();
        assert_eq!(CalcErr::IppatsuWithoutRiichi, actual);
    }

    #[test]
    fn validate_scoring_conditions_double_riichi_haitei_ippatsu() {
        let hand = Hand::new_from_strings(
            vec![
                "123p".to_string(),
                "505s".to_string(),
                "EEEw".to_string(),
                "999m".to_string(),
                "rrd".to_string(),
            ],
            "rd".to_string(),
            "Ew".to_string(),
            "Ew".to_string(),
        )
        .unwrap();
        let actual =
            validate_scoring_conditions(&hand, false, false, true, true, true, false, false)
                .unwrap();
        assert_eq!(CalcErr::DoubleRiichiHaiteiIppatsu, actual);
    }

    #[test]
    fn validate_scoring_conditions_double_riichi_haitei_chankan() {
        let hand = Hand::new_from_strings(
            vec![
                "123p".to_string(),
                "505s".to_string(),
                "EEEw".to_string(),
                "999m".to_string(),
                "rrd".to_string(),
            ],
            "rd".to_string(),
            "Ew".to_string(),
            "Ew".to_string(),
        )
        .unwrap();
        let actual =
            validate_scoring_conditions(&hand, false, false, true, false, true, false, true)
                .unwrap();
        assert_eq!(CalcErr::DoubleRiichiHaiteiChankan, actual);
    }

    #[test]
    fn hand_parser_get_kans() {
        let one_sou: Tile = "1s".to_string().try_into().unwrap(); //4
        let red_dragon: Tile = "rd".to_string().try_into().unwrap(); //3
        let east_wind: Tile = "Ew".to_string().try_into().unwrap(); //4
        let nine_sou: Tile = "9s".to_string().try_into().unwrap(); //3
        let west_wind: Tile = "Ww".to_string().try_into().unwrap(); //2
        let tiles: Vec<Tile> = vec![
            red_dragon.clone(),
            east_wind.clone(),
            west_wind.clone(),
            west_wind.clone(),
            one_sou.clone(),
            one_sou.clone(),
            east_wind.clone(),
            nine_sou.clone(),
            one_sou.clone(),
            red_dragon.clone(),
            east_wind.clone(),
            nine_sou.clone(),
            east_wind.clone(),
            nine_sou.clone(),
            one_sou.clone(),
            red_dragon.clone(),
        ];
        let kans = get_kans(&tiles);
        assert_eq!(kans.len(), 2);
    }

    #[test]
    fn hand_parser_get_trips() {
        let one_sou: Tile = "1s".to_string().try_into().unwrap(); //4
        let red_dragon: Tile = "rd".to_string().try_into().unwrap(); //3
        let east_wind: Tile = "Ew".to_string().try_into().unwrap(); //4
        let nine_sou: Tile = "9s".to_string().try_into().unwrap(); //3
        let west_wind: Tile = "Ww".to_string().try_into().unwrap(); //2
        let tiles: Vec<Tile> = vec![
            red_dragon.clone(),
            east_wind.clone(),
            west_wind.clone(),
            west_wind.clone(),
            one_sou.clone(),
            one_sou.clone(),
            east_wind.clone(),
            nine_sou.clone(),
            one_sou.clone(),
            red_dragon.clone(),
            east_wind.clone(),
            nine_sou.clone(),
            east_wind.clone(),
            nine_sou.clone(),
            one_sou.clone(),
            red_dragon.clone(),
        ];
        let trips = get_triplets(&tiles);
        assert_eq!(trips.len(), 2);
    }

    #[test]
    fn hand_parser_get_pairs() {
        let one_sou: Tile = "1s".to_string().try_into().unwrap(); //4
        let red_dragon: Tile = "rd".to_string().try_into().unwrap(); //3
        let east_wind: Tile = "Ew".to_string().try_into().unwrap(); //4
        let nine_sou: Tile = "9s".to_string().try_into().unwrap(); //3
        let west_wind: Tile = "Ww".to_string().try_into().unwrap(); //2
        let tiles: Vec<Tile> = vec![
            red_dragon.clone(),
            red_dragon.clone(),
            red_dragon.clone(),
            east_wind.clone(),
            east_wind.clone(),
            east_wind.clone(),
            east_wind.clone(),
            one_sou.clone(),
            one_sou.clone(),
            one_sou.clone(),
            one_sou.clone(),
            nine_sou.clone(),
            nine_sou.clone(),
            nine_sou.clone(),
            west_wind.clone(),
            west_wind.clone(),
        ];
        let pairs = get_pairs(&tiles);
        assert_eq!(pairs.len(), 7);
    }

    #[test]
    fn hand_parser_get_seqs() {
        let one_sou: Tile = "1s".to_string().try_into().unwrap(); //4
        let two_sou: Tile = "2s".to_string().try_into().unwrap(); //3
        let three_sou: Tile = "3s".to_string().try_into().unwrap(); //3
        let nine_sou: Tile = "9s".to_string().try_into().unwrap(); //3
        let eight_sou: Tile = "8s".to_string().try_into().unwrap(); //2
        let tiles: Vec<Tile> = vec![
            one_sou.clone(),
            one_sou.clone(),
            one_sou.clone(),
            one_sou.clone(),
            two_sou.clone(),
            two_sou.clone(),
            two_sou.clone(),
            three_sou.clone(),
            three_sou.clone(),
            three_sou.clone(),
            nine_sou.clone(),
            nine_sou.clone(),
            nine_sou.clone(),
            eight_sou.clone(),
            eight_sou.clone(),
        ];
        let seqs = get_sequences(&tiles);
        assert_eq!(seqs.len(), 4);
    }

    #[test]
    fn hand_parser_get_singles() {
        let one_sou: Tile = "1s".to_string().try_into().unwrap(); //4
        let two_sou: Tile = "2s".to_string().try_into().unwrap(); //3
        let three_sou: Tile = "3s".to_string().try_into().unwrap(); //3
        let nine_sou: Tile = "9s".to_string().try_into().unwrap(); //2
        let eight_sou: Tile = "8s".to_string().try_into().unwrap(); //1
        let tiles: Vec<Tile> = vec![
            nine_sou.clone(),
            one_sou.clone(),
            one_sou.clone(),
            one_sou.clone(),
            one_sou.clone(),
            two_sou.clone(),
            two_sou.clone(),
            two_sou.clone(),
            three_sou.clone(),
            three_sou.clone(),
            three_sou.clone(),
            eight_sou.clone(),
            nine_sou.clone(),
        ];
        let singles = get_singles(&tiles);
        assert_eq!(singles.len(), 1);
    }

    #[test]
    fn hand_parser_get_valid_hand_shapes_chitoi() {
        let one_sou: Tile = "1s".to_string().try_into().unwrap();
        let two_sou: Tile = "2s".to_string().try_into().unwrap();
        let three_sou: Tile = "3s".to_string().try_into().unwrap();
        let nine_sou: Tile = "9s".to_string().try_into().unwrap();
        let eight_sou: Tile = "8s".to_string().try_into().unwrap();
        let red_dragon: Tile = "rd".to_string().try_into().unwrap();
        let east_wind: Tile = "Ew".to_string().try_into().unwrap();

        let tiles: Vec<Tile> = vec![
            one_sou.clone(),
            one_sou.clone(),
            two_sou.clone(),
            two_sou.clone(),
            three_sou.clone(),
            three_sou.clone(),
            nine_sou.clone(),
            nine_sou.clone(),
            eight_sou.clone(),
            eight_sou.clone(),
            red_dragon.clone(),
            red_dragon.clone(),
            east_wind.clone(),
            east_wind.clone(),
        ];
        let hand_shapes = get_valid_hand_shapes(&tiles);
        assert_eq!(hand_shapes.len(), 1);

        let tiles: Vec<Tile> = vec![
            one_sou.clone(),
            one_sou.clone(),
            two_sou.clone(),
            two_sou.clone(),
            three_sou.clone(),
            three_sou.clone(),
            nine_sou.clone(),
            nine_sou.clone(),
            eight_sou.clone(),
            eight_sou.clone(),
            red_dragon.clone(),
            red_dragon.clone(),
            east_wind.clone(),
            east_wind.clone(),
            east_wind.clone(),
        ];
        let hand_shapes = get_valid_hand_shapes(&tiles);
        assert_eq!(hand_shapes.len(), 0);
    }

    #[test]
    fn hand_parser_get_valid_hand_shapes_kokushi() {
        let one_pin: Tile = Tile::Pin(MpsValue::One);
        let nine_pin: Tile = Tile::Pin(MpsValue::Nine);
        let one_man: Tile = Tile::Man(MpsValue::One);
        let nine_man: Tile = Tile::Man(MpsValue::Nine);
        let one_sou: Tile = Tile::Sou(MpsValue::One);
        let nine_sou: Tile = Tile::Sou(MpsValue::Nine);
        let red_dragon: Tile = "rd".to_string().try_into().unwrap();
        let green_dragon: Tile = "gd".to_string().try_into().unwrap();
        let white_dragon: Tile = "wd".to_string().try_into().unwrap();
        let east_wind: Tile = "Ew".to_string().try_into().unwrap();
        let south_wind: Tile = "Sw".to_string().try_into().unwrap();
        let west_wind: Tile = "Ww".to_string().try_into().unwrap();
        let north_wind: Tile = "Nw".to_string().try_into().unwrap();

        let tiles: Vec<Tile> = vec![
            one_sou.clone(),
            nine_sou.clone(),
            one_man.clone(),
            nine_man.clone(),
            one_pin.clone(),
            nine_pin.clone(),
            red_dragon.clone(),
            green_dragon.clone(),
            white_dragon.clone(),
            east_wind.clone(),
            south_wind.clone(),
            north_wind.clone(),
            west_wind.clone(),
            north_wind.clone(),
            north_wind.clone(),
        ];
        let hand_shapes = get_valid_hand_shapes(&tiles);
        assert_eq!(hand_shapes.len(), 0);

        let tiles: Vec<Tile> = vec![
            one_sou.clone(),
            nine_sou.clone(),
            one_man.clone(),
            nine_man.clone(),
            one_pin.clone(),
            nine_pin.clone(),
            red_dragon.clone(),
            green_dragon.clone(),
            white_dragon.clone(),
            east_wind.clone(),
            south_wind.clone(),
            north_wind.clone(),
            west_wind.clone(),
            north_wind.clone(),
        ];
        let hand_shapes = get_valid_hand_shapes(&tiles);
        assert_eq!(hand_shapes.len(), 1);
    }

    #[test]
    fn get_valid_hand_shapes_triple_seq() {
        let one_sou: Tile = "1s".to_string().try_into().unwrap();
        let two_sou: Tile = "2s".to_string().try_into().unwrap();
        let three_sou: Tile = "3s".to_string().try_into().unwrap();
        let red_dragon: Tile = "rd".to_string().try_into().unwrap();
        let north_wind: Tile = "Nw".to_string().try_into().unwrap();

        let tiles: Vec<Tile> = vec![
            one_sou.clone(),
            two_sou.clone(),
            three_sou.clone(),
            one_sou.clone(),
            two_sou.clone(),
            three_sou.clone(),
            one_sou.clone(),
            two_sou.clone(),
            three_sou.clone(),
            red_dragon.clone(),
            red_dragon.clone(),
            red_dragon.clone(),
            north_wind.clone(),
            north_wind.clone(),
        ];
        let hand_shapes = get_valid_hand_shapes(&tiles);
        assert_eq!(hand_shapes.len(), 2);
    }

    #[test]
    fn get_valid_hand_shapes_extra_tiles() {
        let one_sou: Tile = "1s".to_string().try_into().unwrap();
        let two_sou: Tile = "2s".to_string().try_into().unwrap();
        let three_sou: Tile = "3s".to_string().try_into().unwrap();
        let four_sou: Tile = "4s".to_string().try_into().unwrap();
        let five_sou: Tile = "5s".to_string().try_into().unwrap();
        let six_sou: Tile = "6s".to_string().try_into().unwrap();
        let red_dragon: Tile = "rd".to_string().try_into().unwrap();

        let tiles: Vec<Tile> = vec![
            one_sou.clone(),
            one_sou.clone(),
            two_sou.clone(),
            two_sou.clone(),
            three_sou.clone(),
            three_sou.clone(),
            four_sou.clone(),
            four_sou.clone(),
            five_sou.clone(),
            five_sou.clone(),
            six_sou.clone(),
            six_sou.clone(),
            red_dragon.clone(),
            red_dragon.clone(),
            red_dragon.clone(),
        ];
        let hand_shapes = get_valid_hand_shapes(&tiles);
        assert_eq!(hand_shapes.len(), 0);

        let tiles: Vec<Tile> = vec![
            one_sou.clone(),
            two_sou.clone(),
            three_sou.clone(),
            three_sou.clone(),
            four_sou.clone(),
            five_sou.clone(),
            six_sou.clone(),
            six_sou.clone(),
            six_sou.clone(),
            red_dragon.clone(),
            red_dragon.clone(),
            red_dragon.clone(),
            four_sou.clone(),
            four_sou.clone(),
            four_sou.clone(),
        ];

        let hand_shapes = get_valid_hand_shapes(&tiles);
        assert_eq!(hand_shapes.len(), 0);
    }

    #[test]
    fn get_valid_hand_shapes_with_aka() {
        let five_sou: Tile = "5s".to_string().try_into().unwrap();
        let aka_five_sou: Tile = "0s".to_string().try_into().unwrap();
        let nine_pin: Tile = "9p".to_string().try_into().unwrap();
        let three_pin: Tile = "3p".to_string().try_into().unwrap();
        let four_sou: Tile = "4s".to_string().try_into().unwrap();
        let three_sou: Tile = "3s".to_string().try_into().unwrap();
        let two_sou: Tile = "2s".to_string().try_into().unwrap();
        let red_dragon: Tile = "rd".to_string().try_into().unwrap();
        let north_wind: Tile = "Nw".to_string().try_into().unwrap();

        let tiles: Vec<Tile> = vec![
            five_sou.clone(),
            four_sou.clone(),
            three_sou.clone(),
            aka_five_sou.clone(),
            four_sou.clone(),
            three_sou.clone(),
            five_sou.clone(),
            four_sou.clone(),
            three_sou.clone(),
            red_dragon.clone(),
            red_dragon.clone(),
            red_dragon.clone(),
            north_wind.clone(),
            north_wind.clone(),
        ];
        let hand_shapes = get_valid_hand_shapes(&tiles);
        assert_eq!(hand_shapes.len(), 2);
        assert!(hand_shapes.iter().all(|h| {
            h.iter()
                .flat_map(|g| g.tiles())
                .filter(|t| t.is_aka())
                .count()
                == 1
        }));

        let tiles: Vec<Tile> = vec![
            nine_pin.clone(),
            nine_pin.clone(),
            three_pin.clone(),
            three_pin.clone(),
            five_sou.clone(),
            aka_five_sou.clone(),
            four_sou.clone(),
            four_sou.clone(),
            two_sou.clone(),
            two_sou.clone(),
            red_dragon.clone(),
            red_dragon.clone(),
            north_wind.clone(),
            north_wind.clone(),
        ];
        let hand_shapes = get_valid_hand_shapes(&tiles);
        assert_eq!(hand_shapes.len(), 1);
    }

    #[test]
    fn get_valid_hand_shapes_no_aka_in_input_means_no_aka_in_output() {
        let tiles: Vec<Tile> = vec![
            "2s", "3s", "4s", "3p", "4p", "5p", "6m", "7m", "8m", "rd", "rd", "rd", "Nw", "Nw",
        ]
        .into_iter()
        .map(|s| s.to_string().try_into().unwrap())
        .collect();

        let hand_shapes = get_valid_hand_shapes(&tiles);

        assert!(!hand_shapes.is_empty());
        assert!(
            hand_shapes
                .iter()
                .all(|h| h.iter().flat_map(|g| g.tiles()).all(|t| !t.is_aka()))
        );
    }

    #[test]
    fn get_valid_hand_shapes_aka_count_is_never_exceeded() {
        let tiles: Vec<Tile> = vec![
            "0s", "0s", "3s", "4s", "5s", "3p", "4p", "5p", "6m", "7m", "8m", "rd", "rd", "Nw",
        ]
        .into_iter()
        .map(|s| s.to_string().try_into().unwrap())
        .collect();

        let hand_shapes = get_valid_hand_shapes(&tiles);

        assert!(hand_shapes.iter().all(|h| {
            h.iter()
                .flat_map(|g| g.tiles())
                .filter(|t| t.is_aka())
                .count()
                <= 2
        }));
    }

    #[test]
    fn get_valid_hand_shapes_aka_does_not_create_extra_shapes() {
        let base_tiles: Vec<Tile> = vec![
            "5s", "5s", "3s", "4s", "3p", "4p", "5p", "6m", "7m", "8m", "rd", "rd", "rd", "Nw",
        ]
        .into_iter()
        .map(|s| s.to_string().try_into().unwrap())
        .collect();

        let aka_tiles: Vec<Tile> = vec![
            "0s", "5s", "3s", "4s", "3p", "4p", "5p", "6m", "7m", "8m", "rd", "rd", "rd", "Nw",
        ]
        .into_iter()
        .map(|s| s.to_string().try_into().unwrap())
        .collect();

        let base_shapes = get_valid_hand_shapes(&base_tiles);
        let aka_shapes = get_valid_hand_shapes(&aka_tiles);

        assert_eq!(base_shapes.len(), aka_shapes.len());
    }

    #[test]
    fn get_valid_hand_shapes_aka_can_be_used_in_sequences() {
        let tiles: Vec<Tile> = vec![
            "3s", "4s", "0s", "3p", "4p", "5p", "6m", "7m", "8m", "rd", "rd", "rd", "Nw", "Nw",
        ]
        .into_iter()
        .map(|s| s.to_string().try_into().unwrap())
        .collect();

        let hand_shapes = get_valid_hand_shapes(&tiles);

        assert!(!hand_shapes.is_empty());
        assert!(
            hand_shapes
                .iter()
                .any(|h| h.iter().flat_map(|g| g.tiles()).any(|t| t.is_aka()))
        );
    }

    #[test]
    fn get_valid_hand_shapes_aka_is_never_honor() {
        let tiles: Vec<Tile> = vec![
            "0s", "5s", "3s", "4s", "3p", "4p", "5p", "6m", "7m", "8m", "rd", "rd", "rd", "Nw",
        ]
        .into_iter()
        .map(|s| s.to_string().try_into().unwrap())
        .collect();

        let hand_shapes = get_valid_hand_shapes(&tiles);

        assert!(hand_shapes.iter().all(|h| {
            h.iter()
                .flat_map(|g| g.tiles())
                .filter(|t| t.is_aka())
                .all(|t| !t.is_honor())
        }));
    }

    #[test]
    fn get_valid_hand_shapes_not_enough_tiles() {
        let one_sou: Tile = "1s".to_string().try_into().unwrap();
        let two_sou: Tile = "2s".to_string().try_into().unwrap();
        let three_sou: Tile = "3s".to_string().try_into().unwrap();

        let tiles: Vec<Tile> = vec![
            one_sou.clone(),
            one_sou.clone(),
            two_sou.clone(),
            two_sou.clone(),
            three_sou.clone(),
            three_sou.clone(),
            one_sou.clone(),
            two_sou.clone(),
        ];
        let hand_shapes = get_valid_hand_shapes(&tiles);
        assert_eq!(hand_shapes.len(), 0);
    }

    #[test]
    fn get_valid_hand_shapes_not_enough_tile_groups_for_kans() {
        let one_sou: Tile = "1s".to_string().try_into().unwrap();
        let two_sou: Tile = "2s".to_string().try_into().unwrap();
        let three_sou: Tile = "3s".to_string().try_into().unwrap();
        let four_sou: Tile = "4s".to_string().try_into().unwrap();

        let tiles: Vec<Tile> = vec![
            one_sou.clone(),
            one_sou.clone(),
            one_sou.clone(),
            one_sou.clone(),
            two_sou.clone(),
            two_sou.clone(),
            two_sou.clone(),
            two_sou.clone(),
            three_sou.clone(),
            three_sou.clone(),
            three_sou.clone(),
            three_sou.clone(),
            four_sou.clone(),
            four_sou.clone(),
        ];
        let hand_shapes = get_valid_hand_shapes(&tiles);
        assert_eq!(hand_shapes.len(), 3);
    }
}
