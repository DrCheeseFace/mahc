use crate::fu::Fu;
use crate::payment::Payment;
use crate::yaku::Yaku;

/// Han value.
pub type HanValue = u32;
/// Fu (minipoints) value.
pub type FuValue = u64;
/// Number of honba (repeat counts).
pub type HonbaCounter = u64;

/// Detailed breakdown of the winning hand's score.
#[derive(Debug)]
pub struct Score {
    /// Breakdown of payment amounts.
    payment: Payment,
    /// List of yaku that were awarded.
    yaku: Vec<Yaku>,
    /// List of fu that were awarded.
    fu: Vec<Fu>,
    /// Total score of the yaku that were awarded including dora.
    han: HanValue,
    /// Total score of the fu that were awarded.
    fu_score: FuValue,
    /// Number of repeat counters.
    honba: HonbaCounter,
    /// Is the hand open when it scored?
    is_open: bool,
    /// total number of han from dora
    dora_count: u32,
}

impl Score {
    /// Create a new [`Score`].
    pub fn new(
        payment: Payment,
        yaku: Vec<Yaku>,
        fu: Vec<Fu>,
        han: HanValue,
        fu_score: FuValue,
        honba: HonbaCounter,
        is_open: bool,
        dora_count: u32,
    ) -> Self {
        Self {
            payment,
            yaku,
            fu,
            han,
            fu_score,
            honba,
            is_open,
            dora_count,
        }
    }

    /// Get the payment breakdown.
    pub fn payment(&self) -> &Payment {
        &self.payment
    }

    /// Get the list of yaku that were awarded.
    pub fn yaku(&self) -> &[Yaku] {
        &self.yaku
    }

    /// Get the list of fu that were awarded.
    pub fn fu(&self) -> &[Fu] {
        &self.fu
    }

    /// Get the total han value of the hand.
    pub fn han(&self) -> HanValue {
        self.han
    }

    /// Get the total fu (minipoints) value of the hand.
    pub fn fu_score(&self) -> FuValue {
        self.fu_score
    }

    /// Get the number of repeat counters.
    pub fn honba(&self) -> HonbaCounter {
        self.honba
    }

    /// Get the state of whether or not the hand was opened.
    pub fn is_open(&self) -> bool {
        self.is_open
    }

    /// Get the total number of han from dora.
    pub fn dora_count(&self) -> u32 {
        self.dora_count
    }
}
