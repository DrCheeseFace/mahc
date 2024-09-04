use crate::score::{
    FuValue, HanValue, Payment, DEALER_RON_MULTIPLIER, DEALER_TSUMO_MULTIPLIER,
    NON_DEALER_RON_MULTIPLIER, NON_DEALER_TSUMO_TO_DEALER_MULTIPLIER,
    NON_DEALER_TSUMO_TO_NON_DEALER_MULTIPLIER,
};

#[derive(Debug)]
pub enum LimitHands {
    Mangan,
    Haneman,
    Baiman,
    Sanbaiman,
    KazoeYakuman,
}

impl LimitHands {
    //TODO: MOVE THIS INTO A SUITABLE STRUCT LATER
    /// Check if the score of the hand is limited (no aotenjou).
    fn is_limit_hand(han: HanValue, fu: FuValue) -> bool {
        if han >= 5 {
            return true;
        }

        if han == 4 && fu >= 40 {
            return true;
        }

        if han == 3 && fu >= 70 {
            return true;
        }

        false
    }

    /// Calculate the limit hand type from the han and fu scores.
    pub fn get_limit_hand(han: HanValue, fu: FuValue) -> Option<Self> {
        if !Self::is_limit_hand(han, fu) {
            return None;
        }

        // TODO: Allow (3 han, 70+ fu) and (4 han, 40+ fu) to be considered manga.
        if han <= 5 {
            Some(Self::Mangan)
        } else if han <= 7 {
            return Some(Self::Haneman);
        } else if han <= 10 {
            return Some(Self::Baiman);
        } else if han <= 12 {
            return Some(Self::Sanbaiman);
        } else {
            return Some(Self::KazoeYakuman);
        }
    }

    /// Get the payment amounts.
    pub fn get_score(&self) -> Payment {
        let base_points = match self {
            Self::Mangan => 2_000,
            Self::Haneman => 3_000,
            Self::Baiman => 4_000,
            Self::Sanbaiman => 6_000,
            Self::KazoeYakuman => 8_000,
        };

        Payment::new(
            base_points * DEALER_RON_MULTIPLIER,
            base_points * DEALER_TSUMO_MULTIPLIER,
            base_points * NON_DEALER_RON_MULTIPLIER,
            base_points * NON_DEALER_TSUMO_TO_NON_DEALER_MULTIPLIER,
            base_points * NON_DEALER_TSUMO_TO_DEALER_MULTIPLIER,
        )
    }
}
