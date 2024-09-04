/// Variations of declaring riichi.
#[derive(Debug, PartialEq)]
pub enum Riichi {
    /// Declared a ready closed hand.
    Riichi,
    /// Called riichi on the first turn.
    DoubleRiichi,
}

#[derive(Debug, Default)]
pub struct RoundContext {
    /// Winning off of a self-draw with a closed hand.
    tsumo: bool,

    /// Declared a ready closed hand.
    riichi: Option<Riichi>,
    /// Winning on the next uninterrupted draw after declaring riichi.
    ippatsu: bool,

    /// Winning with the replacement tile after calling kan.
    rinshan: bool,
    /// Robbing a kan.
    chankan: bool,

    /// Winning from the starting hand.
    haitei: bool,
    /// Winning as the dealer with the haitei (initial draw).
    tenhou: bool,
}

impl RoundContext {
    /// Create a new [`RoundContext`].
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        tsumo: bool,
        riichi: Option<Riichi>,
        ippatsu: bool,
        rinshan: bool,
        chankan: bool,
        haitei: bool,
        tenhou: bool,
    ) -> Self {
        Self {
            tsumo,
            riichi,
            ippatsu,
            rinshan,
            chankan,
            haitei,
            tenhou,
        }
    }

    /// Create a new `RoundContext` builder.
    pub fn builder() -> RoundContextBuilder {
        RoundContextBuilder::default()
    }

    /// Get the tsumo state.
    pub fn tsumo(&self) -> bool {
        self.tsumo
    }

    /// Get the riichi status.
    pub fn riichi(&self) -> &Option<Riichi> {
        &self.riichi
    }

    /// Get the ippatsu state.
    pub fn ippatsu(&self) -> bool {
        self.ippatsu
    }

    /// Get the rinshan kaihou status.
    pub fn rinshan(&self) -> bool {
        self.rinshan
    }

    /// Get the chankan status.
    pub fn chankan(&self) -> bool {
        self.chankan
    }

    /// Get the state of whether or not it is the haitei (initial draw).
    pub fn haitei(&self) -> bool {
        self.haitei
    }

    /// Did the dealer win on the haitei (initial draw)?
    pub fn tenhou(&self) -> bool {
        self.tenhou
    }
}

/// Builder to create a [`RoundContext`].
///
/// # Examples
///
/// ```rust
/// use mahc::round_context::{Riichi, RoundContext};
///
/// let round_context = RoundContext::builder()
///     .tsumo(true)
///     .riichi(Some(Riichi::Riichi))
///     .ippatsu(true)
///     .build();
///
/// assert!(round_context.tsumo());
/// let expected_riichi = Some(Riichi::Riichi);
/// assert_eq!(round_context.riichi(), &expected_riichi);
/// assert!(round_context.ippatsu());
/// assert!(!round_context.rinshan());
/// assert!(!round_context.chankan());
/// assert!(!round_context.haitei());
/// assert!(!round_context.tenhou());
/// ```
#[derive(Debug, Default)]
pub struct RoundContextBuilder {
    /// Winning off of a self-draw with a closed hand.
    tsumo: bool,

    /// Declared a ready closed hand.
    riichi: Option<Riichi>,
    /// Winning on the next uninterrupted draw after declaring riichi.
    ippatsu: bool,

    /// Winning with the replacement tile after calling kan.
    rinshan: bool,
    /// Robbing a kan.
    chankan: bool,

    /// Winning from the starting hand.
    haitei: bool,
    /// Winning as the dealer with the haitei (initial draw).
    tenhou: bool,
}

impl RoundContextBuilder {
    /// Create a [`RoundContext`] from this builder struct.
    pub fn build(self) -> RoundContext {
        RoundContext::new(
            self.tsumo,
            self.riichi,
            self.ippatsu,
            self.rinshan,
            self.chankan,
            self.haitei,
            self.tenhou,
        )
    }

    /// Set the `tsumo` value.
    pub fn tsumo(mut self, tsumo: bool) -> Self {
        self.tsumo = tsumo;

        self
    }

    /// Set the `riichi` value.
    pub fn riichi(mut self, riichi: Option<Riichi>) -> Self {
        self.riichi = riichi;

        self
    }

    /// Set the `ippatsu` value.
    pub fn ippatsu(mut self, ippatsu: bool) -> Self {
        self.ippatsu = ippatsu;

        self
    }

    /// Set the `rinshan` value.
    pub fn rinshan(mut self, rinshan: bool) -> Self {
        self.rinshan = rinshan;

        self
    }

    /// Set the `chankan` value.
    pub fn chankan(mut self, chankan: bool) -> Self {
        self.chankan = chankan;

        self
    }

    /// Set the `haitei` value.
    pub fn haitei(mut self, haitei: bool) -> Self {
        self.haitei = haitei;

        self
    }

    /// Set the `tenhou` value.
    pub fn tenhou(mut self, tenhou: bool) -> Self {
        self.tenhou = tenhou;

        self
    }
}
