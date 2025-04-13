pub mod error;
mod utils;

use crate::calc::utils::{
    get_kans, get_melds_from_tile_counts, get_pairs, get_singles, get_triplets,
};
use crate::fu::{calculate_total_fu_value, Fu};
use crate::hand::validate_hand_shape;
use crate::hand::Hand;
use crate::limit_hand::LimitHands;
use crate::payment::Payment;
use crate::score::{FuValue, HanValue, HonbaCounter, Score};
use crate::tile::Tile;
use crate::tile_group::TileGroup;
use crate::yaku::Yaku;
use error::CalcErr;
use std::collections::HashMap;

/// Get the score breakdown of the hand.
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
        }
        if !y.is_yakuman() {
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

pub fn get_valid_hand_shapes(tiles: &Vec<Tile>) -> Vec<Vec<TileGroup>> {
    let mut hands: Vec<Vec<TileGroup>> = Vec::new();
    let pairs = get_pairs(tiles);
    let trips = get_triplets(tiles);
    let kans = get_kans(tiles);
    let singles = get_singles(tiles);

    // all hands require atleast 1 pair
    if pairs.is_empty() {
        return hands;
    }

    //kokushi chitoi check
    if trips.is_empty() && kans.is_empty() {
        let kokushi_chitoi: Vec<TileGroup> = [singles, trips, kans, pairs.clone()].concat();
        match validate_hand_shape(&kokushi_chitoi) {
            Some(_) => {}
            None => hands.push(kokushi_chitoi),
        }
    }

    let mut tile_counts: HashMap<Tile, u8> = HashMap::new();
    for tile in tiles {
        tile_counts
            .entry(*tile)
            .and_modify(|x| *x += 1)
            .or_insert(0);
    }

    // TODO good lord all mighty this can be optimised ALOT
    let mut hand_combos: Vec<Vec<TileGroup>> = Vec::new();
    let all_melds = get_melds_from_tile_counts(&tile_counts);
    for pair in pairs {
        for meld_1 in &all_melds {
            for meld_2 in &all_melds {
                for meld_3 in &all_melds {
                    for meld_4 in &all_melds {
                        let mut combined_melds = vec![
                            meld_1.clone(),
                            meld_2.clone(),
                            meld_3.clone(),
                            meld_4.clone(),
                            pair.clone(),
                        ];
                        combined_melds.sort();
                        if !hand_combos.contains(&combined_melds) {
                            hand_combos.push(combined_melds.clone());

                            let mut hand_tile_counts: HashMap<Tile, u8> = HashMap::new();
                            for tile in combined_melds.iter().flat_map(|m| m.tiles()) {
                                hand_tile_counts
                                    .entry(*tile)
                                    .and_modify(|x| *x += 1)
                                    .or_insert(0);
                            }

                            if hand_tile_counts == tile_counts {
                                hands.push(combined_melds);
                            }
                        }
                    }
                }
            }
        }
    }

    hands
}

#[cfg(test)]
mod tests {
    use crate::{
        calc::{
            error::CalcErr,
            get_hand_score, get_valid_hand_shapes,
            utils::{get_pairs, get_sequences, get_singles, get_triplets},
        },
        hand::Hand,
        tile::{MpsValue, Tile},
    };

    use super::{get_kans, validate_scoring_conditions};

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
}
