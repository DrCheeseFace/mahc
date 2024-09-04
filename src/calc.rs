use crate::fu::{calculate_total_fu_value, Fu};
use crate::hand::error::HandErr;
use crate::hand::Hand;
use crate::limit_hand::LimitHands;
use crate::payment::Payment;
use crate::round_context::{Riichi, RoundContext};
use crate::score::{FuValue, HanValue, HonbaCounter, Score};
use crate::yaku::Yaku;

#[derive(Debug, PartialEq)]
pub enum CalculatorErrors {
    NoHan,
    NoFu,
    NoYaku,
}

impl std::fmt::Display for CalculatorErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoHan => write!(f, "No han provided!"),
            Self::NoFu => write!(f, "No fu provided!"),
            Self::NoYaku => write!(f, "No Yaku!"),
        }
    }
}

/// Get the score breakdown of the hand.
pub fn get_hand_score(
    tiles: Vec<String>,
    win: String,
    dora: u32,
    seat: String,
    prev: String,
    round_context: &RoundContext,
    honba: HonbaCounter,
) -> Result<Score, HandErr> {
    let hand = Hand::new(tiles, win, seat, prev)?;
    if hand.kans().is_empty() && round_context.rinshan() {
        return Err(HandErr::RinshanKanWithoutKan);
    }

    let yaku = get_yaku_han(&hand, round_context);

    if yaku.0 == 0 {
        return Err(HandErr::NoYaku);
    }

    //fuck you chiitoiistu, why u gota be different, AND YOU TOO PINFU
    //i can move this to calculatefu method maybe?
    let fu = {
        if yaku.1.contains(&Yaku::Chiitoitsu) {
            vec![Fu::BasePointsChitoi]
        } else if yaku.1.contains(&Yaku::Pinfu) {
            if round_context.tsumo() {
                vec![Fu::BasePoints]
            } else {
                vec![Fu::BasePoints, Fu::ClosedRon]
            }
        } else {
            hand.calculate_fu(round_context.tsumo())
        }
    };
    let han = yaku.0 + dora;
    let fu_value = calculate_total_fu_value(&fu);

    let mut has_yakuman = false;
    for y in &yaku.1 {
        if y.is_yakuman() {
            has_yakuman = true;
        }
    }

    let payment = if has_yakuman {
        calculate_yakuman(&yaku.1)?
    } else {
        //can unwrap here because check for yaku earlier
        calculate(han, fu_value).unwrap()
    };
    let score = Score::new(payment, yaku.1, fu, han, fu_value, honba, hand.is_open());

    Ok(score)
}

/// Get the yaku score and list of yaku given a hand and some round context.
pub fn get_yaku_han(hand: &Hand, round_context: &RoundContext) -> (HanValue, Vec<Yaku>) {
    let mut yaku: Vec<Yaku> = vec![];

    let conditions = [
        (round_context.ippatsu(), Yaku::Ippatsu),
        (round_context.haitei(), Yaku::Haitei),
        (round_context.rinshan(), Yaku::RinshanKaihou),
        (round_context.chankan(), Yaku::Chankan),
        (hand.is_tanyao(), Yaku::Tanyao),
        (hand.is_iipeikou(), Yaku::Iipeikou),
        (hand.is_ryanpeikou(), Yaku::Ryanpeikou),
        (hand.is_toitoi(), Yaku::Toitoi),
        (hand.is_sanshokudoujun(), Yaku::SanshokuDoujun),
        (hand.is_sanankou(round_context.tsumo()), Yaku::Sanankou),
        (hand.is_honitsu(), Yaku::Honitsu),
        (hand.is_shousangen(), Yaku::Shousangen),
        (hand.is_junchantaiyao(), Yaku::JunchanTaiyao),
        (hand.is_honroutou(), Yaku::Honroutou),
        (hand.is_sankantsu(), Yaku::Sankantsu),
        (hand.is_ittsuu(), Yaku::Ittsuu),
        (hand.is_chantaiyao(), Yaku::Chantaiyao),
        (hand.is_chiitoitsu(), Yaku::Chiitoitsu),
        (
            hand.is_menzentsumo(round_context.tsumo()),
            Yaku::MenzenTsumo,
        ),
        (hand.is_pinfu(), Yaku::Pinfu),
        (hand.is_sanshokudoukou(), Yaku::SanshokuDoukou),
        (hand.is_chinitsu(), Yaku::Chinitsu),
    ];

    //check if there are many yakuman, if so return only yakuman
    //this is so unbelievably jank but it works
    let mut yakuman: Vec<Yaku> = vec![];
    let yakumanconditions = [
        (hand.is_daisangen(), Yaku::Daisangen),
        (hand.is_suuankou(round_context.tsumo()), Yaku::Suuankou),
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
        (hand.is_tenhou(round_context.tenhou()), Yaku::Tenhou),
        (hand.is_chiihou(round_context.tenhou()), Yaku::Chiihou),
    ];

    for (condition, yaku_type) in yakumanconditions {
        if condition {
            yakuman.push(yaku_type);
        }
    }
    if !yakuman.is_empty() {
        return (yakuman.len() as HanValue, yakuman);
    }

    if let Some(riichi) = round_context.riichi() {
        match riichi {
            Riichi::Riichi => yaku.push(Yaku::Riichi),
            Riichi::DoubleRiichi => yaku.push(Yaku::DoubleRiichi),
        }
    }
    for (condition, yaku_type) in conditions {
        if condition {
            yaku.push(yaku_type);
        }
    }

    for _i in 0..hand.is_yakuhai() {
        yaku.push(Yaku::Yakuhai);
    }

    let mut yaku_han = 0;
    for y in &yaku {
        yaku_han += y.get_han(hand.is_open());
    }

    (yaku_han, yaku)
}

/// Calculate the payment amounts from the list of yakuman yaku.
pub fn calculate_yakuman(yaku: &Vec<Yaku>) -> Result<Payment, HandErr> {
    let mut total = 0;
    for y in yaku {
        if y.is_yakuman() {
            total += y.get_han(false);
        }
    }
    if total == 0 {
        return Err(HandErr::NoYaku);
    }

    let basepoints: u64 = (8_000 * total).into();
    let payment = Payment::new(basepoints);

    Ok(payment)
}

/// Calculate the payment amounts from the han, fu, and number of honba (repeat counters).
pub fn calculate(han: HanValue, fu: FuValue) -> Result<Payment, HandErr> {
    if han == 0 {
        return Err(HandErr::NoHan);
    }

    if fu == 0 {
        return Err(HandErr::NoFu);
    }

    let k = LimitHands::get_limit_hand(han, fu);
    if let Some(limithand) = k {
        let payment = limithand.get_score();

        return Ok(payment);
    }

    let payment = Payment::from_han_and_fu(han, fu);

    Ok(payment)
}
