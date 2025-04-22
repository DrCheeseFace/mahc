use crate::score::{FuValue, HanValue, HonbaCounter};

/// Number of points players pay to the winner.
// NOTE: `u64` allows for scoring with aotenjou (no limits).
pub(crate) type Points = u64;

const DEALER_RON_MULTIPLIER: u64 = 6;
const DEALER_TSUMO_MULTIPLIER: u64 = 2;
const NON_DEALER_RON_MULTIPLIER: u64 = 4;
const NON_DEALER_TSUMO_TO_NON_DEALER_MULTIPLIER: u64 = 1;
const NON_DEALER_TSUMO_TO_DEALER_MULTIPLIER: u64 = 2;

/// Methods to provide a breakdown of payment amounts players will pay.
///
/// This struct stores the base points the hand was awarded.
///
/// # Base value forumla
///
/// The formula for calculating the base value without caps or limits applied (aotenjou)
/// or han < 5 (no aotenjou) is:
///
/// ```text
/// fu * 2 ^ (2 + han)
/// ```
///
/// # Limit Scores Base Points
///
/// These are the base points for han >= 5.
///
/// | Limit Name | Han   | Base Points |
/// |:----------:|:-----:|:-----------:|
/// | Mangan     | 5     | 2,000       |
/// | Haneman    | 6-7   | 3,000       |
/// | Baiman     | 8-10  | 4,000       |
/// | Sanbaiman  | 11-12 | 6,000       |
/// | Yakuman    | 13+   | 8,000       |
///
/// # Payment
///
/// Multiply the basic point value by the amount listed in the table below.
///
/// |                | Tsumo                                           | Ron                           |
/// |----------------|-------------------------------------------------|-------------------------------|
/// | **Non-dealer** | - 1x by other non-dealers<br>- 2x by the dealer | - 4x by the discarding player |
/// | **Dealer**     | - 2x from all other players                     | - 6x by the discarding player |
///
/// Each payment is rounded up to the nearest 100.
///
/// In addition to the payment, the winner is paid an additional amount of points based on the number of honba counters on the table.
///
/// # Examples
///
/// < 5 han
///
/// ```rust
/// use mahc::payment::Payment;
///
/// let han = 2;
/// let fu = 40;
/// let payment = Payment::from_han_and_fu(han, fu);
/// let expected_base_points = 640;
/// assert_eq!(payment.base_points(), expected_base_points);
///
/// let honba = 0;
/// let expected_payment_ron = 2_600;
/// assert_eq!(payment.non_dealer_ron(honba), expected_payment_ron);
/// ```
///
/// Mangan:
///
/// ```rust
/// use mahc::payment::Payment;
///
/// let base_points_mangan = 2_000;
/// let payment = Payment::new(base_points_mangan);
///
/// let honba = 0;
/// let expected_dealer_ron = 12_000;
/// assert_eq!(payment.dealer_ron(honba), expected_dealer_ron);
///
/// let honba = 2;
/// let expected_dealer_tsumo = 4_200;
/// assert_eq!(payment.dealer_tsumo(honba), expected_dealer_tsumo);
/// ```
#[derive(Debug)]
pub struct Payment {
    /// Base score for the hand.
    base_points: Points,
    /// The number of points each honba (repeat counter) is worth.
    tsumibou: Points,
}

impl Payment {
    /// Create a new [`Payment`].
    pub fn new(base_points: Points) -> Self {
        Self {
            base_points,
            tsumibou: 300,
        }
    }

    /// Calculate the base points with the given han and fu.
    ///
    /// <div class="warning">
    ///
    /// This will not factor in any limitations (assumes aotenjou).
    ///
    /// </div>
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mahc::payment::Payment;
    ///
    /// let han = 1;
    /// let fu = 32;
    /// let payment = Payment::from_han_and_fu(han, fu);
    /// let expected_base_value = 320;
    /// assert_eq!(payment.base_points(), expected_base_value);
    /// ```
    pub fn from_han_and_fu(han: HanValue, fu: FuValue) -> Self {
        let fu = if fu == 25 {
            // Chiitoitsu (seven pairs) does not round the fu.
            fu
        } else {
            // Round up to the nearest 10.
            fu.div_ceil(10) * 10
        };

        Self::new(fu * 2u64.pow(han + 2))
    }

    /// Get the base points.
    pub fn base_points(&self) -> Points {
        self.base_points
    }

    /// Round the payment amount to the nearest hundredth.
    fn round_payment(&self, unrounded_payment: Points) -> Points {
        unrounded_payment.div_ceil(100) * 100
    }

    /// Get the amount of points the player that dealt-in has to pay to a dealer.
    pub fn dealer_ron(&self, honba: HonbaCounter) -> Points {
        self.round_payment((self.base_points * DEALER_RON_MULTIPLIER) + (self.tsumibou * honba))
    }

    /// Get the amount of points each player pays when the dealer tsumos.
    pub fn dealer_tsumo(&self, honba: HonbaCounter) -> Points {
        // NOTE: This is assuming a 4-player game where the tsumibou is divided by the number of other players.
        self.round_payment(
            (self.base_points * DEALER_TSUMO_MULTIPLIER) + ((self.tsumibou / 3) * honba),
        )
    }

    /// Get the amount the player that dealt-in pays to a non-dealer.
    pub fn non_dealer_ron(&self, honba: HonbaCounter) -> Points {
        self.round_payment((self.base_points * NON_DEALER_RON_MULTIPLIER) + (self.tsumibou * honba))
    }

    /// Get the amount the dealer pays when a non-dealer wins by tsumo.
    pub fn non_dealer_tsumo_to_dealer(&self, honba: HonbaCounter) -> Points {
        // NOTE: This is assuming a 4-player game where the tsumibou is divided by the number of other players.
        self.round_payment(
            (self.base_points * NON_DEALER_TSUMO_TO_DEALER_MULTIPLIER)
                + ((self.tsumibou / 3) * honba),
        )
    }

    /// Get the amount non-dealer players pay when a non-dealer wins by tsumo.
    pub fn non_dealer_tsumo_to_non_dealer(&self, honba: HonbaCounter) -> Points {
        // NOTE: This is assuming a 4-player game where the tsumibou is divided by the number of other players.
        self.round_payment(
            (self.base_points * NON_DEALER_TSUMO_TO_NON_DEALER_MULTIPLIER)
                + ((self.tsumibou / 3) * honba),
        )
    }
}
