#[derive(Debug, PartialEq)]
pub enum Yaku {
    // One Han Yaku
    Tanyao,   //done
    Iipeikou, //done
    Yakuhai,  //done
    MenzenTsumo,
    Pinfu,
    Riichi,  //(bascially done by default)
    Ippatsu, //(bascially done by default)
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
    Ryanpeikou, //done

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

            //TODO gota be a better way of doing this
            Self::KazoeYakuman => "Kazoe Yakuman ",
            Self::KokushiMusou => "KokushiMusou Yakuman",
            Self::KokushiMusou13SidedWait => "KokushiMusou Yakuman 13 sided wait",
            Self::Suuankou => "Suuankou Yakuman",
            Self::Daisangen => "Daisangen Yakuman",
            Self::Shousuushii => "Shousuushii Yakuman",
            Self::Daisuushii => "Daisuushii Yakuman",
            Self::Tsuuiisou => "Tsuuiisou Yakuman",
            Self::Chinroutou => "Chinroutou Yakuman",
            Self::Ryuuiisou => "Ryuuiisou Yakuman",
            Self::ChuurenPoutou => "ChuurenPoutou Yakuman",
            Self::Suukantsu => "Suukantsu Yakuman",
            Self::Tenhou => "Tenhou Yakuman",
            Self::Chiihou => "Chiihou Yakuman",
            Self::SuuankouTankiWait => "Suuankou Yakuman Tanki Wait ",
            Self::Daichiishin => "Daichiishin Yakuman",
            Self::ChuurenPoutou9SidedWait => "ChuurenPoutou Yakuman 9 sided wait ",
        }
        .to_string()
    }

    //TODO adjust for open or closed !!!!
    pub fn get_han(&self, is_open: bool) -> u16 {
        match self {
            Self::Tanyao => 1,
            Self::Iipeikou => 1,
            Self::Yakuhai => 1,
            Self::MenzenTsumo => 1,
            Self::Pinfu => 1,
            Self::Riichi => 1,
            Self::Ippatsu => 1,
            Self::Haitei => 1,
            Self::RinshanKaihou => 1,
            Self::Chankan => 1,

            Self::DoubleRiichi => 2,
            Self::Toitoi => 2,
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
            Self::Chantaiyao => 2,
            Self::Sanankou => 2,
            Self::SanshokuDoukou => 2,
            Self::Sankantsu => 2,
            Self::Honroutou => 2,
            Self::Shousangen => 2,
            Self::Chiitoitsu => 2,

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

            //TODO gota be a better way of doing this
            Self::KazoeYakuman => 1,
            Self::KokushiMusou => 1,
            Self::KokushiMusou13SidedWait => 1,
            Self::Suuankou => 1,
            Self::Daisangen => 1,
            Self::Shousuushii => 1,
            Self::Daisuushii => 1,
            Self::Tsuuiisou => 1,
            Self::Chinroutou => 1,
            Self::Ryuuiisou => 1,
            Self::ChuurenPoutou => 1,
            Self::ChuurenPoutou9SidedWait => 1,
            Self::Suukantsu => 1,
            Self::Tenhou => 1,
            Self::Chiihou => 1,
            Self::SuuankouTankiWait => 1,
            Self::Daichiishin => 1,
        }
    }
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
