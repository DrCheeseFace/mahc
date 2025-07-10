use crate::score::HanValue;

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Yaku {
    // One Han Yaku
    Tanyao,
    Iipeikou,
    Yakuhai,
    MenzenTsumo,
    Pinfu,
    Riichi,
    Ippatsu,
    Haitei,
    RinshanKaihou,
    Chankan,

    // Two Han Yaku
    DoubleRiichi,
    Toitoi,
    Ittsuu,
    SanshokuDoujun,
    Chantaiyao,
    Sanankou,
    SanshokuDoukou,
    Sankantsu,
    Honroutou,
    Shousangen,
    Chiitoitsu,

    // Three Han Yaku
    Honitsu,
    JunchanTaiyao,
    Ryanpeikou,

    // Six Han Yaku
    Chinitsu,

    // Yakuman
    KazoeYakuman,
    KokushiMusou,
    KokushiMusou13SidedWait,
    Suuankou,
    SuuankouTankiWait,
    Daisangen,
    Shousuushii,
    Daisuushii,
    Tsuuiisou,
    Daichiishin,
    Chinroutou,
    Ryuuiisou,
    ChuurenPoutou,
    ChuurenPoutou9SidedWait,
    Suukantsu,
    Tenhou,
    Chiihou,
}

impl Yaku {
    /// Convert the value to a `String`.
    pub fn to_string(&self, is_open: bool) -> String {
        match self {
            Self::Tanyao => "Tanyao: 1",
            Self::Iipeikou => "Iipeikou: 1",
            Self::Yakuhai => "Yakuhai: 1",
            Self::MenzenTsumo => "MenzenTsumo: 1",
            Self::Pinfu => "Pinfu: 1",
            Self::Riichi => "Riichi: 1",
            Self::Ippatsu => "Ippatsu: 1",
            Self::Haitei => "Haitei: 1",
            Self::RinshanKaihou => "RinshanKaihou: 1",
            Self::Chankan => "Chankan: 1",

            Self::DoubleRiichi => "DoubleRiichi: 2",
            Self::Toitoi => "Toitoi: 2",
            Self::Ittsuu => {
                if is_open {
                    "Ittsuu: 1"
                } else {
                    "Ittsuu: 2"
                }
            }
            Self::SanshokuDoujun => {
                if is_open {
                    "Sanshoku Doujun: 1"
                } else {
                    "Sanshoku Doujun: 2"
                }
            }
            Self::Chantaiyao => {
                if is_open {
                    "Chantaiyao: 1"
                } else {
                    "Chantaiyao: 2"
                }
            }
            Self::Sanankou => "Sanankou: 2",
            Self::SanshokuDoukou => "SanshokuDoukou: 2",
            Self::Sankantsu => "Sankantsu: 2",
            Self::Honroutou => "Honroutou: 2",
            Self::Shousangen => "Shousangen: 2",
            Self::Chiitoitsu => "Chiitoitsu: 2",

            Self::Honitsu => {
                if is_open {
                    "Honitsu: 2"
                } else {
                    "Honitsu: 3"
                }
            }
            Self::JunchanTaiyao => {
                if is_open {
                    "JunchanTaiyao: 2"
                } else {
                    "JunchanTaiyao: 3"
                }
            }
            Self::Ryanpeikou => "Ryanpeikou: 3",

            Self::Chinitsu => {
                if is_open {
                    "Chinitsu: 5"
                } else {
                    "Chinitsu: 6"
                }
            }

            Self::KazoeYakuman => "Kazoe Yakuman: 1",
            Self::KokushiMusou => "KokushiMusou Yakuman: 1",
            Self::KokushiMusou13SidedWait => "KokushiMusou Yakuman 13 sided wait: 1",
            Self::Suuankou => "Suuankou Yakuman: 1",
            Self::Daisangen => "Daisangen Yakuman: 1",
            Self::Shousuushii => "Shousuushii Yakuman: 1",
            Self::Daisuushii => "Daisuushii Yakuman: 2",
            Self::Tsuuiisou => "Tsuuiisou Yakuman: 1",
            Self::Chinroutou => "Chinroutou Yakuman: 1",
            Self::Ryuuiisou => "Ryuuiisou Yakuman: 1",
            Self::ChuurenPoutou => "ChuurenPoutou Yakuman: 1",
            Self::Suukantsu => "Suukantsu Yakuman: 1",
            Self::Tenhou => "Tenhou Yakuman: 1",
            Self::Chiihou => "Chiihou Yakuman: 1",
            Self::SuuankouTankiWait => "Suuankou Yakuman Tanki Wait: 1",
            Self::Daichiishin => "Daichiishin Yakuman: 1",
            Self::ChuurenPoutou9SidedWait => "ChuurenPoutou Yakuman 9 sided wait: 1",
        }
        .to_string()
    }

    /// Get the han value of the yaku.
    pub fn get_han(&self, is_open: bool) -> HanValue {
        match self {
            Self::Tanyao
            | Self::Iipeikou
            | Self::Yakuhai
            | Self::MenzenTsumo
            | Self::Pinfu
            | Self::Riichi
            | Self::Ippatsu
            | Self::Haitei
            | Self::RinshanKaihou
            | Self::Chankan => 1,

            Self::Ittsuu => {
                if is_open {
                    return 1;
                }
                2
            }
            Self::SanshokuDoujun => {
                if is_open {
                    return 1;
                }
                2
            }
            Self::DoubleRiichi
            | Self::Toitoi
            | Self::Chantaiyao
            | Self::Sanankou
            | Self::SanshokuDoukou
            | Self::Sankantsu
            | Self::Honroutou
            | Self::Shousangen
            | Self::Chiitoitsu => 2,

            Self::Honitsu => {
                if is_open {
                    return 2;
                }
                3
            }
            Self::JunchanTaiyao => {
                if is_open {
                    return 2;
                }
                3
            }
            Self::Ryanpeikou => 3,

            Self::Chinitsu => {
                if is_open {
                    return 5;
                }
                6
            }

            Self::KazoeYakuman
            | Self::KokushiMusou
            | Self::KokushiMusou13SidedWait
            | Self::Suuankou
            | Self::Daisangen
            | Self::Shousuushii
            | Self::Tsuuiisou
            | Self::Chinroutou
            | Self::Ryuuiisou
            | Self::ChuurenPoutou
            | Self::ChuurenPoutou9SidedWait
            | Self::Suukantsu
            | Self::Tenhou
            | Self::Chiihou
            | Self::SuuankouTankiWait
            | Self::Daichiishin => 1,
            Self::Daisuushii => 2,
        }
    }

    /// Check if the yaku is considered a yakuman.
    pub fn is_yakuman(&self) -> bool {
        matches!(
            self,
            Self::KazoeYakuman
                | Self::KokushiMusou
                | Self::KokushiMusou13SidedWait
                | Self::Suuankou
                | Self::SuuankouTankiWait
                | Self::Daisangen
                | Self::Shousuushii
                | Self::Daisuushii
                | Self::Tsuuiisou
                | Self::Daichiishin
                | Self::Chinroutou
                | Self::Ryuuiisou
                | Self::ChuurenPoutou
                | Self::ChuurenPoutou9SidedWait
                | Self::Suukantsu
                | Self::Tenhou
                | Self::Chiihou
        )
    }
}
