use crate::payment::Payment;
use crate::score::{FuValue, HanValue};

#[derive(Debug)]
pub enum LimitHands {
    Mangan,
    Haneman,
    Baiman,
    Sanbaiman,
    KazoeYakuman,
}

impl LimitHands {
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

        if han <= 5 {
            Some(Self::Mangan)
        } else if han <= 7 {
            Some(Self::Haneman)
        } else if han <= 10 {
            Some(Self::Baiman)
        } else if han <= 12 {
            Some(Self::Sanbaiman)
        } else {
            Some(Self::KazoeYakuman)
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

        Payment::new(base_points)
    }
}
