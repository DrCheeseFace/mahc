pub mod error;

use error::CalcErr;

use crate::fu::{calculate_total_fu_value, Fu};
use crate::hand::Hand;
use crate::limit_hand::LimitHands;
use crate::payment::Payment;
use crate::score::{FuValue, HanValue, HonbaCounter, Score};
use crate::tile::Tile;
use crate::yaku::Yaku;

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

#[cfg(test)]
mod tests {
    use crate::{
        calc::{error::CalcErr, get_hand_score},
        hand::Hand,
        tile::Tile,
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
}
