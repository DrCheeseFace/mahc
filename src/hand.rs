pub mod error;
use crate::fu::Fu;
use crate::suit::Suit;
use crate::tile::*;
use crate::tile_group::{GroupType, TileGroup};
use error::HandErr;

#[derive(Debug)]
pub struct Hand {
    groups: Vec<TileGroup>,
    win_tile: Tile,
    seat_tile: Tile,
    /// Prevalent or round wind.
    prev_tile: Tile,
    isopen: bool,
}

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        for group in self.groups.iter() {
            out.push_str(&group.to_string());
            out.push(' ');
        }
        out.push_str("| ");
        out.push_str(&self.win_tile.to_string());
        write!(f, "{}", out)
    }
}

impl Hand {
    pub fn new(
        groups: Vec<TileGroup>,
        win_tile: Tile,
        seat_tile: Tile,
        prev_tile: Tile,
    ) -> Result<Self, HandErr> {
        //TODO: standard hand ONLY CHECK MUST FIX FOR KOKUSHI
        let mut full_shape_count = 0;
        let mut pair_count = 0;
        let mut no_shape_count = 0;
        let mut isopen = false;

        for group in &groups {
            match group.group_type() {
                GroupType::Triplet | GroupType::Sequence | GroupType::Kan => full_shape_count += 1,
                GroupType::Pair => pair_count += 1,
                GroupType::None => no_shape_count += 1,
            }
            if group.isopen() {
                isopen = true;
            }
        }

        if !(full_shape_count == 4 && pair_count == 1)
            && pair_count != 7
            && !(no_shape_count == 12 && pair_count == 1)
        {
            return Err(HandErr::InvalidShape);
        }
        // check if last group contains the winning tile
        // FUCK handling kokuushi
        let tilecount: u8 = groups.iter().map(|s| s.group_type().tile_count()).sum();
        if tilecount == 14 {
            let last_group = groups.last().unwrap();
            // last group should not be open
            if last_group.isopen() {
                return Err(HandErr::InvalidShape);
            }
            match last_group.group_type() {
                GroupType::Sequence => {
                    if win_tile.suit() != last_group.suit() {
                        return Err(HandErr::InvalidShape);
                    }

                    let win_int = win_tile.parse_u8()?;
                    let last_int = last_group.parse_u8()?;

                    if win_int != last_int && win_int != last_int + 1 && win_int != last_int + 2 {
                        return Err(HandErr::InvalidShape);
                    }
                }
                GroupType::Triplet | GroupType::Pair => {
                    if last_group.value() != win_tile.value()
                        || last_group.suit() != win_tile.suit()
                    {
                        return Err(HandErr::InvalidShape);
                    }
                }
                GroupType::Kan => return Err(HandErr::InvalidShape),
                GroupType::None => {
                    // TODO(zhiyao): A hacky way to check for kokushi.
                    // Maybe we should find a more explicit way.
                    if !(no_shape_count == 12 && pair_count == 1) {
                        return Err(HandErr::InvalidShape);
                    }
                }
            }
        }

        Ok(Hand {
            groups,
            win_tile,
            seat_tile,
            prev_tile,
            isopen,
        })
    }
    pub fn new_from_strings(
        tiles: Vec<String>,
        win: String,
        prev: String,
        seat: String,
    ) -> Result<Self, HandErr> {
        let mut tile_groups: Vec<TileGroup> = Vec::new();

        // NOTE: Strings are complicated in Rust and needs evaluation about how to iterate over one. Because the string is expected to contain ASCII characters, `.chars()` should be okay.
        for i in &tiles {
            let tile: TileGroup = i.to_string().try_into()?;
            tile_groups.push(tile);
        }

        let win_tile: Tile = win.try_into()?;
        let seat_tile: Tile = seat.try_into()?;
        let prev_tile: Tile = prev.try_into()?;

        let hand = Hand::new(tile_groups, win_tile, seat_tile, prev_tile)?;

        Ok(hand)
    }

    /// Calculate the fu types in the hand.
    pub fn calculate_fu(&self, tsumo: bool) -> Vec<Fu> {
        let mut fu_types: Vec<Fu> = vec![];
        if self.is_chiitoitsu() {
            return vec![Fu::BasePointsChitoi];
        }

        fu_types.push(Fu::BasePoints);

        if tsumo {
            fu_types.push(Fu::Tsumo);
        } else if !self.is_open() {
            fu_types.push(Fu::ClosedRon);
        }

        //meld fu cal
        for tile_group in &self.triplets() {
            let group_is_terminal_or_honor = tile_group.is_honor() || tile_group.is_terminal();

            if tile_group == self.groups.last().unwrap() {
                if tsumo {
                    if group_is_terminal_or_honor {
                        fu_types.push(Fu::NonSimpleClosedTriplet);
                    } else {
                        fu_types.push(Fu::SimpleClosedTriplet);
                    }
                } else if group_is_terminal_or_honor {
                    fu_types.push(Fu::NonSimpleOpenTriplet);
                } else {
                    fu_types.push(Fu::SimpleOpenTriplet);
                }
                continue;
            }

            if !group_is_terminal_or_honor && tile_group.isopen() {
                fu_types.push(Fu::SimpleOpenTriplet);
            }

            if !tile_group.isopen() {
                if group_is_terminal_or_honor {
                    fu_types.push(Fu::NonSimpleClosedTriplet);
                } else {
                    fu_types.push(Fu::SimpleClosedTriplet);
                }
            } else if group_is_terminal_or_honor {
                fu_types.push(Fu::NonSimpleOpenTriplet);
            }
        }

        for kan in &self.kans() {
            let group_is_terminal_or_honor = kan.is_honor() || kan.is_terminal();

            if group_is_terminal_or_honor {
                if !kan.isopen() {
                    fu_types.push(Fu::NonSimpleClosedKan);
                } else {
                    fu_types.push(Fu::NonSimpleOpenKan);
                }
            } else if !kan.isopen() {
                fu_types.push(Fu::SimpleClosedKan);
            } else {
                fu_types.push(Fu::SimpleOpenKan);
            }
        }

        for pair in self.pairs() {
            if pair.tiles()[0] == self.prev_tile
                || pair.tiles()[0] == self.seat_tile
                || pair.tiles()[0].suit() == Suit::Dragon
            {
                fu_types.push(Fu::Toitsu);
            }
        }

        //fu wait cal
        if let Some(group) = self.groups.last() {
            match group.group_type() {
                GroupType::Pair => fu_types.push(Fu::SingleWait),
                GroupType::Sequence => {
                    let mid_tile = group.tiles()[0].clone().next().unwrap();
                    if self.win_tile() == mid_tile
                        || !self.win_tile().is_terminal() && group.is_terminal()
                    {
                        fu_types.push(Fu::SingleWait);
                    }
                }
                _ => {}
            }
        }

        //handling pinfu :(
        if self.groups.len() != 13 && !self.isopen {
            if tsumo {
                if fu_types
                    .iter()
                    .all(|fu| matches!(fu, Fu::BasePoints | Fu::ClosedRon | Fu::Tsumo))
                {
                    return vec![Fu::BasePoints];
                }
            } else if fu_types
                .iter()
                .all(|fu| matches!(fu, Fu::BasePoints | Fu::ClosedRon))
            {
                return vec![Fu::BasePoints, Fu::ClosedRon];
            }
        }

        fu_types
    }

    /// Get the dora count in the hand from dora indicator tiles.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mahc::hand::Hand;
    /// use mahc::tile::Tile;
    /// let hand = Hand::new_from_strings(
    ///     vec![
    ///         "123p".to_string(),
    ///         "505s".to_string(),
    ///         "EEEw".to_string(),
    ///         "9999m".to_string(),
    ///         "rrd".to_string(),
    ///     ],
    ///     "rd".to_string(),
    ///     "Ew".to_string(),
    ///     "Ew".to_string(),
    /// )
    /// .unwrap();
    /// let doras: Vec<Tile> = vec![
    ///    "1p".to_string().try_into().unwrap(),
    ///    "4s".to_string().try_into().unwrap(),
    ///    "Nw".to_string().try_into().unwrap(),
    ///    "8m".to_string().try_into().unwrap(),
    ///    "gd".to_string().try_into().unwrap(),
    /// ];
    ///
    /// let dora = hand.get_dora_count(&Some(doras));
    /// assert_eq!(dora, 14);
    /// ```
    pub fn get_dora_count(&self, dora_indicator_tiles: &Option<Vec<Tile>>) -> u32 {
        let mut count = 0;
        for group in &self.groups {
            for tile in group.tiles().iter() {
                if tile.is_aka() {
                    count += 1;
                }
            }
        }
        if dora_indicator_tiles.is_none() {
            return count;
        }
        for tile in dora_indicator_tiles.clone().unwrap() {
            let dora_tile = tile.clone().next().unwrap();
            for triplet in self.triplets() {
                if triplet.tiles()[0] == dora_tile {
                    count += 3;
                }
            }
            for kan in self.kans() {
                if kan.tiles()[0] == dora_tile {
                    count += 4;
                }
            }
            for pair in self.pairs() {
                if pair.tiles()[0] == dora_tile {
                    count += 2;
                }
            }
            for sequence in self.sequences() {
                for tile in sequence.tiles() {
                    if tile == dora_tile {
                        count += 1
                    }
                }
            }
        }
        count
    }

    /// Get the sequence groups in the hand.
    pub fn sequences(&self) -> Vec<TileGroup> {
        // TODO: We can do better than cloning into `into_iter()`.
        self.groups
            .clone()
            .into_iter()
            .filter(|group| matches!(group.group_type(), GroupType::Sequence))
            .collect()
    }

    /// Get the triplet groups in the hand.
    pub fn triplets(&self) -> Vec<TileGroup> {
        // TODO: We can do better than cloning into `into_iter()`.
        self.groups
            .clone()
            .into_iter()
            .filter(|group| matches!(group.group_type(), GroupType::Triplet))
            .collect()
    }

    /// Get the kan groups in the hand.
    pub fn kans(&self) -> Vec<TileGroup> {
        // TODO: We can do better than cloning into `into_iter()`.
        self.groups
            .clone()
            .into_iter()
            .filter(|group| matches!(group.group_type(), GroupType::Kan))
            .collect()
    }

    /// Get the pair groups in the hand.
    pub fn pairs(&self) -> Vec<TileGroup> {
        // TODO: We can do better than cloning into `into_iter()`.
        self.groups
            .clone()
            .into_iter()
            .filter(|group| matches!(group.group_type(), GroupType::Pair))
            .collect()
    }

    /// Get the groups with no shape in the hand.
    ///
    /// This can be used to check for kokushi musou (thirteen orphans).
    pub fn singles(&self) -> Vec<TileGroup> {
        // TODO: We can do better than cloning into `into_iter()`.
        self.groups
            .clone()
            .into_iter()
            .filter(|group| matches!(group.group_type(), GroupType::None))
            .collect()
    }

    /// Get the winning tile the completes the hand.
    pub fn win_tile(&self) -> Tile {
        self.win_tile.clone()
    }

    /// Get the seat wind.
    pub fn seat_tile(&self) -> Tile {
        self.seat_tile.clone()
    }

    /// Get the prevalent wind.
    pub fn prev_tile(&self) -> Tile {
        self.prev_tile.clone()
    }

    /// Get the state of whether or not the hand has been opened.
    pub fn is_open(&self) -> bool {
        self.isopen
    }

    //yaku validation

    /// Check if the hand only contains simple tiles -- no terminal or honor tiles.
    pub fn is_tanyao(&self) -> bool {
        self.groups
            .iter()
            .all(|group| !group.is_terminal() && !group.is_honor())
    }

    /// Check if the hand contains two unique identical sequences.
    pub fn is_ryanpeikou(&self) -> bool {
        if self.is_open() {
            return false;
        }

        let mut seqs: Vec<TileGroup> = self.sequences();

        if seqs.len() != 4 {
            return false;
        }

        seqs.sort();
        if seqs[1] == seqs[2] {
            seqs.dedup();
            seqs.len() == 1
        } else {
            seqs.dedup();
            seqs.len() == 2
        }
    }

    /// Check if the hand contains two identical sequences.
    pub fn is_iipeikou(&self) -> bool {
        if self.is_open() {
            return false;
        }

        let mut seqs: Vec<TileGroup> = self.sequences();
        seqs.sort();

        let mut seqs_dedup: Vec<TileGroup> = seqs.clone();
        seqs_dedup.dedup();

        match seqs.len() - seqs_dedup.len() {
            1 => true,
            2 => seqs[1] == seqs[2],
            _ => false,
        }
    }

    /// Check if the hand contains value honors.
    pub fn is_yakuhai(&self) -> u16 {
        // i do it like this because a single group can have multiple yakuhai
        let mut count = 0;

        for group in self.triplets().iter().chain(self.kans().iter()) {
            if group.value() == self.prev_tile.value() {
                count += 1;
            }
            if group.value() == self.seat_tile.value() {
                count += 1;
            }
            if group.suit() == Suit::Dragon {
                count += 1;
            }
        }

        count
    }

    /// Check if the hand contains all triplets and kans.
    pub fn is_toitoi(&self) -> bool {
        self.triplets().len() + self.kans().len() == 4
    }

    /// Check if the hand contains three concealed triplets (including kans).
    ///
    /// If the last necessary triplet is formed from a ron, it is not considered concealed and sanankou is not granted.
    pub fn is_sanankou(&self, tsumo: bool) -> bool {
        let mut closed_triplet_count = 0;

        for triplet_group in self.triplets().iter().chain(self.kans().iter()) {
            if !triplet_group.isopen() {
                closed_triplet_count += 1;
            }
        }

        if !tsumo && self.groups.last().unwrap().group_type() == GroupType::Triplet {
            closed_triplet_count -= 1;
        }

        closed_triplet_count == 3
    }

    /// Check if the hand contains a mixed triple sequence (ex: `123m 123p 123s`).
    pub fn is_sanshokudoujun(&self) -> bool {
        if self.sequences().len() < 3 {
            return false;
        }

        let binding = self.sequences();
        let mut seqs: Vec<_> = binding
            .iter()
            .map(|group| (group.value(), group.suit().clone()))
            .collect();

        seqs.sort();
        seqs.dedup();

        match seqs.len() {
            3 => seqs[1].0 == seqs[2].0 && seqs[0].0 == seqs[1].0,
            4 => seqs[1].0 == seqs[2].0 && (seqs[0].0 == seqs[1].0 || seqs[2].0 == seqs[3].0),
            _ => false,
        }
    }

    /// Check if the hand only contains tiles of one suit and any honor tiles.
    ///
    /// This is commonly referred to as a "half flush".
    pub fn is_honitsu(&self) -> bool {
        if self.groups.len() == 13 {
            return false;
        }

        let mut has_honor = false;
        let mut has_normal = false;
        let mut suit: Option<Suit> = None;
        for group in &self.groups {
            if group.is_honor() {
                has_honor = true;
            } else {
                has_normal = true;
                suit = Some(group.suit());
            }
        }

        if !has_normal || !has_honor {
            return false;
        }

        if let Some(s) = suit {
            for group in &self.groups {
                if group.suit() != s && !group.is_honor() {
                    return false;
                }
            }
        } else {
            return false;
        }

        true
    }

    /// Check if the hand has two dragon triplets or quads and a pair of dragon tiles.
    pub fn is_shousangen(&self) -> bool {
        let dragon_count = self
            .triplets()
            .iter()
            .chain(self.kans().iter())
            .filter(|group| group.suit() == Suit::Dragon)
            .count();

        dragon_count == 2 && self.pairs()[0].suit() == Suit::Dragon
    }

    /// Check if the hand only contains groups with at least one terminal tile.
    pub fn is_junchantaiyao(&self) -> bool {
        if self.groups.len() == 13 {
            return false;
        }

        if self
            .groups
            .iter()
            .any(|group| group.is_honor() || !group.is_terminal())
        {
            return false;
        }

        !self.sequences().is_empty()
    }

    /// Check if the hand only contains terminal and honor tiles.
    pub fn is_honroutou(&self) -> bool {
        if self.groups.len() == 13 {
            return false;
        }

        if !self.sequences().is_empty() {
            return false;
        }

        let mut has_terminal: bool = false;
        let mut has_honor: bool = false;
        for group in self.groups.clone() {
            for tile in group.tiles().iter() {
                if tile.is_honor() {
                    has_honor = true
                } else if tile.is_terminal() {
                    has_terminal = true
                } else {
                    return false;
                }
            }
        }

        has_terminal && has_honor
    }

    /// Check if the hand has three quads.
    pub fn is_sankantsu(&self) -> bool {
        self.kans().len() == 3
    }

    /// Check if the hand has three exact sequences of 1-2-3, 4-5-6, and 7-8-9 in the same suit.
    pub fn is_ittsuu(&self) -> bool {
        //there has GOTTO be a better way to do this
        let suits = [Suit::Pinzu, Suit::Manzu, Suit::Souzu];
        suits.iter().any(|suit| {
            let values: Vec<String> = self
                .sequences()
                .iter()
                .filter(|&x| x.suit() == *suit)
                .map(|x| x.value().to_string())
                .collect::<_>();

            values.contains(&ONE_VALUE.to_string())
                && values.contains(&FOUR_VALUE.to_string())
                && values.contains(&SEVEN_VALUE.to_string())
        })
    }

    /// Check if the hand only contains groups with terminal or honor tiles; sequences are permitted as long as they contain a terminal.
    ///
    /// There must be at least one sequence, otherwise the hand would either be junchan or tsuuiisou.
    pub fn is_chantaiyao(&self) -> bool {
        if self.groups.len() == 13 {
            return false;
        }

        if self.sequences().is_empty() {
            return false;
        }

        let mut has_terminal = false;
        let mut has_honor = false;
        for group in self.groups.clone() {
            if group.is_terminal() {
                has_terminal = true;
            } else if group.is_honor() {
                has_honor = true;
            } else {
                return false;
            }
        }

        has_terminal && has_honor
    }

    /// Check if the hand consists of 7 unique pairs.
    pub fn is_chiitoitsu(&self) -> bool {
        self.pairs().len() == 7
    }

    /// Check if the hand has won with a self-draw.
    ///
    /// The hand must be closed.
    pub fn is_menzentsumo(&self, tsumo: bool) -> bool {
        !self.isopen && tsumo
    }

    /// Check if the hand has won with all sequences, with the last shape waiting to complete a sequence.
    ///
    /// The hand must be closed.
    pub fn is_pinfu(&self) -> bool {
        if self.groups.len() == 13 {
            return false;
        }

        if self.isopen {
            return false;
        }

        self.calculate_fu(false)
            .iter()
            .all(|fu| matches!(fu, Fu::BasePoints | Fu::ClosedRon))
    }

    /// Check if the hand contains three triplets (or quads) of the same value across the three numerical suits (manzu, pinzu, and souzu).
    pub fn is_sanshokudoukou(&self) -> bool {
        if self.triplets().len() + self.kans().len() < 3 {
            return false;
        }

        let mut list_of_vals: Vec<String> = vec![];
        for group in self.triplets().iter().chain(self.kans().iter()) {
            list_of_vals.push(group.value().to_string());
        }
        list_of_vals.sort();

        if list_of_vals[1] == list_of_vals[2] {
            if list_of_vals[0] == list_of_vals[1] {
                return true;
            }
            if list_of_vals.len() == 4 && list_of_vals[2] == list_of_vals[3] {
                return true;
            }
        }

        false
    }

    /// Check if the hand only contains tiles of a single suit.
    pub fn is_chinitsu(&self) -> bool {
        if self.groups.len() == 13 {
            return false;
        }

        let mut suits: Vec<Suit> = self.groups.iter().map(|x| x.suit().clone()).collect();
        suits.dedup();

        suits.len() == 1
    }

    //yakuman

    /// Check if the hand contains three dragon triplets (or quads).
    pub fn is_daisangen(&self) -> bool {
        let trips: Vec<Tile> = self
            .triplets()
            .iter()
            .chain(self.kans().iter())
            .map(|x| x.tiles()[0].clone())
            .collect();

        trips.contains(&Tile::Dragon(DValue::Red))
            && trips.contains(&Tile::Dragon(DValue::Green))
            && trips.contains(&Tile::Dragon(DValue::White))
    }

    /// Check if the hand contains four concealed triplets.
    ///
    /// If winning by ron, the last completed shape cannot be the triplet (e.g. only the pair).
    pub fn is_suuankou(&self, tsumo: bool) -> bool {
        if self.groups.len() == 13 {
            return false;
        }

        if self.triplets().len() + self.kans().len() != 4 || self.isopen {
            return false;
        }

        if !tsumo && self.groups.last().unwrap().group_type() == GroupType::Triplet {
            return false;
        }

        true
    }

    /// Check if the hand contains four concealed triplets, with the last completed shape being the pair.
    ///
    /// This yaku can be awarded on a ron with a tanki (single pair) wait.
    pub fn is_suuankoutankiwait(&self) -> bool {
        if self.groups.len() == 13 {
            return false;
        }

        if self.triplets().len() + self.kans().len() != 4 || self.isopen {
            return false;
        }

        if self.groups.last().unwrap().group_type() == GroupType::Pair {
            return true;
        }

        false
    }

    /// Check if the hand only contains terminal tiles.
    pub fn is_chinroutou(&self) -> bool {
        if self.groups.len() == 13 {
            return false;
        }

        if self.kans().len() + self.triplets().len() != 4 {
            return false;
        }

        for group in self.groups.clone() {
            if !group.is_terminal() {
                return false;
            }
        }

        true
    }

    /// Check if the hand only contains the 2-3-4-6-8 sou (bamboo) tiles and green dragon tile.
    ///
    /// This is commonly known as "all greens".
    pub fn is_ryuuiisou(&self) -> bool {
        if self.groups.len() == 13 {
            return false;
        }

        for groups in self.groups.iter() {
            for tile in groups.tiles().iter() {
                if ![
                    Tile::Dragon(DValue::Green),
                    Tile::Sou(MpsValue::Two),
                    Tile::Sou(MpsValue::Three),
                    Tile::Sou(MpsValue::Four),
                    Tile::Sou(MpsValue::Six),
                    Tile::Sou(MpsValue::Eight),
                ]
                .contains(tile)
                {
                    return false;
                }
            }
        }
        true
    }

    /// Check if the hand consists of 1112345678999 in the same suit, plus one additional tile of that suit.
    pub fn is_chuurenpoutou(&self) -> bool {
        if self.groups.len() == 13 {
            return false;
        }

        let suit: Suit = self.groups[0].suit().clone();
        if self.triplets().len() != 2 || self.sequences().len() != 2 || self.pairs().len() != 1 {
            return false;
        }

        for group in self.groups.clone() {
            if group.suit() != suit {
                return false;
            }
        }

        let has_1 = self
            .triplets()
            .clone()
            .iter()
            .any(|i| i.value() == ONE_VALUE);
        let has_9 = self
            .triplets()
            .clone()
            .iter()
            .any(|i| i.value() == NINE_VALUE);
        if !has_1 || !has_9 {
            return false;
        }

        let mut vals: Vec<u8> = vec![];
        for sequence_group in self.sequences() {
            let int = sequence_group.value().to_string().parse::<u8>().unwrap();
            vals.push(int);
            vals.push(int + 1);
            vals.push(int + 2);
        }

        for pair_group in self.pairs() {
            let int = pair_group.value().to_string().parse::<u8>().unwrap();
            vals.push(int);
        }

        vals.sort();
        if vals != [2, 3, 4, 5, 6, 7, 8] {
            return false;
        }

        true
    }

    /// Check if the hand consists of 1112345678999 in the same suit, plus one additional tile of that suit.
    ///
    /// This variant checks that the hand was completed with a 9-sided wait.
    pub fn is_chuurenpoutou9sided(&self) -> bool {
        if self.groups.len() == 13 {
            return false;
        }

        if !self.is_chuurenpoutou() {
            return false;
        }

        if self.groups.last().unwrap().group_type() != GroupType::Pair {
            return false;
        }

        true
    }

    /// Check if the hand only consists of honor tiles.
    pub fn is_tsuuiisou(&self) -> bool {
        if self.groups.len() == 13 {
            return false;
        }

        for group in self.groups.clone() {
            if !group.is_honor() {
                return false;
            }
        }

        true
    }

    /// Check if the hand only consists of honor tiles as seven pairs.
    pub fn is_daichiishin(&self) -> bool {
        self.is_tsuuiisou() && self.pairs().len() == 7
    }

    /// Check if the hand has four quads.
    pub fn is_suukantsu(&self) -> bool {
        self.kans().len() == 4
    }

    /// Check if the hand has three wind triplets (or quads) and a wind pair.
    pub fn is_shousuushii(&self) -> bool {
        if self.pairs()[0].suit() != Suit::Wind {
            return false;
        }
        self.triplets()
            .iter()
            .chain(self.kans().iter())
            .filter(|i| i.suit() == Suit::Wind)
            .count()
            == 3
    }

    /// Check if the hand has four wind triplets (or quads).
    pub fn is_daisuushii(&self) -> bool {
        self.triplets()
            .iter()
            .chain(self.kans().iter())
            .filter(|i| i.suit() == Suit::Wind)
            .count()
            == 4
    }

    /// Check if the hand has one of each type of terminal and honor tile and one additional terminal or honor tile.
    pub fn is_kokushi(&self) -> bool {
        if self.singles().len() != 12 || self.pairs().len() != 1 {
            return false;
        }

        let mut orphans = vec![
            Tile::Man(MpsValue::One),
            Tile::Man(MpsValue::Nine),
            Tile::Sou(MpsValue::One),
            Tile::Sou(MpsValue::Nine),
            Tile::Pin(MpsValue::One),
            Tile::Pin(MpsValue::Nine),
            Tile::Wind(WValue::East),
            Tile::Wind(WValue::South),
            Tile::Wind(WValue::West),
            Tile::Wind(WValue::North),
            Tile::Dragon(DValue::Red),
            Tile::Dragon(DValue::White),
            Tile::Dragon(DValue::Green),
        ];

        for tile in self.groups.iter() {
            if let Some(pos) = orphans.iter().position(|orphan| orphan == &tile.tiles()[0]) {
                orphans.remove(pos);
            } else {
                return false;
            }
        }

        orphans.is_empty()
    }

    /// Check if the hand has one of each type of terminal and honor tile and one additional terminal or honor tile, on a 13-sided wait.
    pub fn is_kokushi13sided(&self) -> bool {
        self.is_kokushi() && self.groups.last().unwrap().group_type() == GroupType::Pair
    }

    /// Check if the player is the dealer and has a winning hand in the uninterrupted first turn.
    ///
    /// Calling a kan counts as interrupting the turn order.
    pub fn is_tenhou(&self, tenhou: bool) -> bool {
        if tenhou && self.seat_tile() == Tile::Wind(WValue::East) {
            return true;
        }
        false
    }

    /// Check if the player is in a non-dealer seat and has a winning hand in the first non-interrupted turn.
    ///
    /// Calling a kan counts as interrupting the turn order.
    pub fn is_chiihou(&self, tenhou: bool) -> bool {
        if tenhou && self.seat_tile() == Tile::Wind(WValue::East) {
            return true;
        }
        false
    }

    /// get emoji format for hand
    pub fn get_emoji(&self) -> String {
        let mut out = String::new();
        for group in self.groups.iter() {
            out.push_str(&group.get_emoji());
            out.push(' ');
        }
        out.push_str("| ");
        out.push_str(self.win_tile.get_emoji());
        out
    }
}

#[cfg(test)]
mod tests {
    use super::Hand;
    use crate::hand::error::HandErr;

    #[test]
    fn yaku_kokushi() {
        let out = Hand::new_from_strings(
            vec![
                "1s".to_string(),
                "2s".to_string(),
                "1m".to_string(),
                "9m".to_string(),
                "1p".to_string(),
                "9p".to_string(),
                "Ew".to_string(),
                "Sw".to_string(),
                "Ww".to_string(),
                "Nw".to_string(),
                "gd".to_string(),
                "rd".to_string(),
                "wwd".to_string(),
            ],
            "wd".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_kokushi());
        let out = Hand::new_from_strings(
            vec![
                "1s".to_string(),
                "2s".to_string(),
                "1m".to_string(),
                "9m".to_string(),
                "1p".to_string(),
                "9p".to_string(),
                "Ew".to_string(),
                "Sw".to_string(),
                "Ww".to_string(),
                "Nw".to_string(),
                "gd".to_string(),
                "rd".to_string(),
                "wwd".to_string(),
            ],
            "wd".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_kokushi());
        let out = Hand::new_from_strings(
            vec![
                "1s".to_string(),
                "9s".to_string(),
                "1m".to_string(),
                "9m".to_string(),
                "1p".to_string(),
                "9p".to_string(),
                "Ew".to_string(),
                "Sw".to_string(),
                "Ww".to_string(),
                "Nw".to_string(),
                "gd".to_string(),
                "rd".to_string(),
                "wwd".to_string(),
            ],
            "wd".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_kokushi());
        assert!(out.is_kokushi13sided());
        let out = Hand::new_from_strings(
            vec![
                "1s".to_string(),
                "9s".to_string(),
                "1m".to_string(),
                "9m".to_string(),
                "1p".to_string(),
                "9p".to_string(),
                "Ew".to_string(),
                "Sw".to_string(),
                "Ww".to_string(),
                "Nw".to_string(),
                "gd".to_string(),
                "wwd".to_string(),
                "rd".to_string(),
            ],
            "rd".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_kokushi());
        assert!(!out.is_kokushi13sided());
        let out = Hand::new_from_strings(
            vec![
                "9s".to_string(),
                "9s".to_string(),
                "1m".to_string(),
                "9m".to_string(),
                "1p".to_string(),
                "9p".to_string(),
                "Ew".to_string(),
                "Sw".to_string(),
                "Ww".to_string(),
                "Nw".to_string(),
                "gd".to_string(),
                "rd".to_string(),
                "wwd".to_string(),
            ],
            "wd".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_kokushi());
    }

    #[test]
    fn yaku_daisuushii() {
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
        let out = Hand::new_from_strings(
            vec![
                "EEEEw".to_string(),
                "SSw".to_string(),
                "WWWw".to_string(),
                "NNNw".to_string(),
                "999s".to_string(),
            ],
            "9s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_daisuushii());
    }

    #[test]
    fn yaku_shousuushi() {
        let out = Hand::new_from_strings(
            vec![
                "EEEEw".to_string(),
                "SSw".to_string(),
                "WWWw".to_string(),
                "NNNw".to_string(),
                "999s".to_string(),
            ],
            "9s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_shousuushii());
        let out = Hand::new_from_strings(
            vec![
                "EEEEw".to_string(),
                "22s".to_string(),
                "WWWw".to_string(),
                "NNNw".to_string(),
                "999s".to_string(),
            ],
            "9s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_shousuushii());
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
        assert!(!out.is_shousuushii());
    }

    #[test]
    fn yaku_suukantsu() {
        let out = Hand::new_from_strings(
            vec![
                "EEEEw".to_string(),
                "2222p".to_string(),
                "1111mo".to_string(),
                "7777s".to_string(),
                "99s".to_string(),
            ],
            "9s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_suukantsu());
        let out = Hand::new_from_strings(
            vec![
                "EEEw".to_string(),
                "2222p".to_string(),
                "1111mo".to_string(),
                "7777s".to_string(),
                "99s".to_string(),
            ],
            "9s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_suukantsu());
    }

    #[test]
    fn yaku_daichiishin() {
        let out = Hand::new_from_strings(
            vec![
                "SSw".to_string(),
                "rrd".to_string(),
                "ggd".to_string(),
                "wwd".to_string(),
                "NNw".to_string(),
                "SSw".to_string(),
                "EEw".to_string(),
            ],
            "Ew".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_daichiishin());
        let out = Hand::new_from_strings(
            vec![
                "WWw".to_string(),
                "NNNw".to_string(),
                "gggd".to_string(),
                "rrrd".to_string(),
                "EEEw".to_string(),
            ],
            "Ew".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_daichiishin());
    }

    #[test]
    fn yaku_tsuuiisou() {
        let out = Hand::new_from_strings(
            vec![
                "SSw".to_string(),
                "NNNw".to_string(),
                "gggd".to_string(),
                "rrrd".to_string(),
                "EEEw".to_string(),
            ],
            "Ew".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_tsuuiisou());
        let out = Hand::new_from_strings(
            vec![
                "11s".to_string(),
                "NNNw".to_string(),
                "gggd".to_string(),
                "rrrd".to_string(),
                "EEEw".to_string(),
            ],
            "Ew".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_tsuuiisou());
    }

    #[test]
    fn yaku_cuurenpoutou() {
        let out = Hand::new_from_strings(
            vec![
                "111s".to_string(),
                "234s".to_string(),
                "55s".to_string(),
                "678s".to_string(),
                "999s".to_string(),
            ],
            "9s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_chuurenpoutou());
        let out = Hand::new_from_strings(
            vec![
                "111s".to_string(),
                "234s".to_string(),
                "55m".to_string(),
                "678s".to_string(),
                "999s".to_string(),
            ],
            "9s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_chuurenpoutou());
        let out = Hand::new_from_strings(
            vec![
                "123s".to_string(),
                "234s".to_string(),
                "555s".to_string(),
                "678s".to_string(),
                "99s".to_string(),
            ],
            "9s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_chuurenpoutou());
        let out = Hand::new_from_strings(
            vec![
                "111s".to_string(),
                "234s".to_string(),
                "678s".to_string(),
                "999s".to_string(),
                "55s".to_string(),
            ],
            "5s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_chuurenpoutou9sided());
        let out = Hand::new_from_strings(
            vec![
                "111s".to_string(),
                "234s".to_string(),
                "678s".to_string(),
                "55s".to_string(),
                "999s".to_string(),
            ],
            "9s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_chuurenpoutou9sided());
    }

    #[test]
    fn yaku_ryuuiisou() {
        let out = Hand::new_from_strings(
            vec![
                "234p".to_string(),
                "234s".to_string(),
                "66s".to_string(),
                "gggd".to_string(),
                "888s".to_string(),
            ],
            "8s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_ryuuiisou());
        let out = Hand::new_from_strings(
            vec![
                "234s".to_string(),
                "234s".to_string(),
                "66s".to_string(),
                "gggd".to_string(),
                "888s".to_string(),
            ],
            "8s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_ryuuiisou());
        let out = Hand::new_from_strings(
            vec![
                "345s".to_string(),
                "234s".to_string(),
                "66s".to_string(),
                "gggd".to_string(),
                "888s".to_string(),
            ],
            "8s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_ryuuiisou());

        let out = Hand::new_from_strings(
            vec![
                "234s".to_string(),
                "234s".to_string(),
                "666s".to_string(),
                "gggd".to_string(),
                "99s".to_string(),
            ],
            "9s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_ryuuiisou());
    }

    #[test]
    fn yaku_chinroutou() {
        let out = Hand::new_from_strings(
            vec![
                "111so".to_string(),
                "1111m".to_string(),
                "999s".to_string(),
                "999p".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_chinroutou());

        let out = Hand::new_from_strings(
            vec![
                "rrrd".to_string(),
                "1111m".to_string(),
                "999s".to_string(),
                "999p".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_chinroutou());

        let out = Hand::new_from_strings(
            vec![
                "111s".to_string(),
                "1111m".to_string(),
                "789s".to_string(),
                "999p".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_chinroutou());
    }

    #[test]
    fn yaku_suuankoutankiwait() {
        let out = Hand::new_from_strings(
            vec![
                "rrrd".to_string(),
                "888p".to_string(),
                "777s".to_string(),
                "111s".to_string(),
                "11m".to_string(),
            ],
            "1m".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_suuankoutankiwait());
        let out = Hand::new_from_strings(
            vec![
                "rrrd".to_string(),
                "888p".to_string(),
                "777s".to_string(),
                "11m".to_string(),
                "111s".to_string(),
            ],
            "1s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_suuankoutankiwait());
    }

    #[test]
    fn yaku_suuankou() {
        let out = Hand::new_from_strings(
            vec![
                "rrrd".to_string(),
                "888p".to_string(),
                "777s".to_string(),
                "111s".to_string(),
                "11m".to_string(),
            ],
            "1m".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_suuankou(false));

        let out = Hand::new_from_strings(
            vec![
                "rrrd".to_string(),
                "888p".to_string(),
                "777s".to_string(),
                "11m".to_string(),
                "111s".to_string(),
            ],
            "1s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_suuankou(true));
        let out = Hand::new_from_strings(
            vec![
                "rrrd".to_string(),
                "888p".to_string(),
                "777s".to_string(),
                "11m".to_string(),
                "111s".to_string(),
            ],
            "1s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_suuankou(false));
    }

    #[test]
    fn yaku_daisangen() {
        let out = Hand::new_from_strings(
            vec![
                "rrrd".to_string(),
                "gggd".to_string(),
                "wwwwd".to_string(),
                "88p".to_string(),
                "567p".to_string(),
            ],
            "6p".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_daisangen());
        let out = Hand::new_from_strings(
            vec![
                "rrrrd".to_string(),
                "ggggd".to_string(),
                "wwwwd".to_string(),
                "88p".to_string(),
                "567p".to_string(),
            ],
            "6p".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_daisangen());

        let out = Hand::new_from_strings(
            vec![
                "rrrrd".to_string(),
                "gggd".to_string(),
                "wwd".to_string(),
                "888p".to_string(),
                "567p".to_string(),
            ],
            "6p".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_daisangen());
    }

    #[test]
    fn yaku_chinitsu() {
        let out = Hand::new_from_strings(
            vec![
                "222p".to_string(),
                "123p".to_string(),
                "345p".to_string(),
                "88p".to_string(),
                "567p".to_string(),
            ],
            "6p".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_chinitsu());
        let out = Hand::new_from_strings(
            vec![
                "222p".to_string(),
                "123p".to_string(),
                "345p".to_string(),
                "88s".to_string(),
                "567p".to_string(),
            ],
            "6p".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_chinitsu());
    }

    #[test]
    fn yaku_sanshokudoukou() {
        let out = Hand::new_from_strings(
            vec![
                "222p".to_string(),
                "222m".to_string(),
                "222s".to_string(),
                "11s".to_string(),
                "456m".to_string(),
            ],
            "6m".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_sanshokudoukou());
        let out = Hand::new_from_strings(
            vec![
                "222p".to_string(),
                "2222m".to_string(),
                "222s".to_string(),
                "3333s".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_sanshokudoukou());
        let out = Hand::new_from_strings(
            vec![
                "222p".to_string(),
                "333m".to_string(),
                "222s".to_string(),
                "11s".to_string(),
                "333s".to_string(),
            ],
            "3s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_sanshokudoukou());
        let out = Hand::new_from_strings(
            vec![
                "444s".to_string(),
                "444m".to_string(),
                "222s".to_string(),
                "11p".to_string(),
                "222p".to_string(),
            ],
            "2p".to_string(),
            "Ew".to_string(),
            "Ew".to_string(),
        )
        .unwrap();
        assert!(!out.is_sanshokudoukou());
    }

    #[test]
    fn yaku_pinfu() {
        let out = Hand::new_from_strings(
            vec![
                "123p".to_string(),
                "678p".to_string(),
                "345s".to_string(),
                "11s".to_string(),
                "456m".to_string(),
            ],
            "6m".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_pinfu());

        let out = Hand::new_from_strings(
            vec![
                "123p".to_string(),
                "678po".to_string(),
                "345s".to_string(),
                "11s".to_string(),
                "456m".to_string(),
            ],
            "5m".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_pinfu());
        let out = Hand::new_from_strings(
            vec![
                "123p".to_string(),
                "678p".to_string(),
                "345s".to_string(),
                "11s".to_string(),
                "456m".to_string(),
            ],
            "5m".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_pinfu());
        let out = Hand::new_from_strings(
            vec![
                "123p".to_string(),
                "678p".to_string(),
                "345s".to_string(),
                "11s".to_string(),
                "rrrd".to_string(),
            ],
            "rd".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_pinfu());
    }

    #[test]
    fn yaku_menzentsumo() {
        let out = Hand::new_from_strings(
            vec![
                "11s".to_string(),
                "22p".to_string(),
                "33p".to_string(),
                "44p".to_string(),
                "55p".to_string(),
                "66p".to_string(),
                "77p".to_string(),
            ],
            "7p".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_menzentsumo(true));

        let out = Hand::new_from_strings(
            vec![
                "11s".to_string(),
                "22p".to_string(),
                "33p".to_string(),
                "44p".to_string(),
                "55p".to_string(),
                "66p".to_string(),
                "77p".to_string(),
            ],
            "7p".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_menzentsumo(false));

        let out = Hand::new_from_strings(
            vec![
                "123p".to_string(),
                "999p".to_string(),
                "111p".to_string(),
                "rrrdo".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_menzentsumo(false));
    }

    #[test]
    fn yaku_chiitoitsu() {
        let out = Hand::new_from_strings(
            vec![
                "11s".to_string(),
                "22p".to_string(),
                "33p".to_string(),
                "44p".to_string(),
                "55p".to_string(),
                "66p".to_string(),
                "77p".to_string(),
            ],
            "7p".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_chiitoitsu());
        let out = Hand::new_from_strings(
            vec![
                "123p".to_string(),
                "999p".to_string(),
                "111p".to_string(),
                "rrrdo".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_chiitoitsu());
    }

    #[test]
    fn yaku_chantaiyao() {
        let out = Hand::new_from_strings(
            vec![
                "123p".to_string(),
                "999p".to_string(),
                "111p".to_string(),
                "rrrdo".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_chantaiyao());

        let out = Hand::new_from_strings(
            vec![
                "123p".to_string(),
                "999p".to_string(),
                "111p".to_string(),
                "999m".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_chantaiyao());
        let out = Hand::new_from_strings(
            vec![
                "111s".to_string(),
                "999p".to_string(),
                "111p".to_string(),
                "999m".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_chantaiyao());
    }

    #[test]
    fn yaku_ittsuu() {
        let out = Hand::new_from_strings(
            vec![
                "123p".to_string(),
                "456p".to_string(),
                "789p".to_string(),
                "rrrdo".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_ittsuu());

        let out = Hand::new_from_strings(
            vec![
                "789m".to_string(),
                "456m".to_string(),
                "123m".to_string(),
                "234m".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_ittsuu());
        let out = Hand::new_from_strings(
            vec![
                "123s".to_string(),
                "789s".to_string(),
                "456s".to_string(),
                "234m".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_ittsuu());
        let out = Hand::new_from_strings(
            vec![
                "123m".to_string(),
                "456m".to_string(),
                "678m".to_string(),
                "234m".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_ittsuu());
    }

    #[test]
    fn yaku_sankantsu() {
        let out = Hand::new_from_strings(
            vec![
                "9999so".to_string(),
                "123p".to_string(),
                "SSSSw".to_string(),
                "EEEEw".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_sankantsu());
        let out = Hand::new_from_strings(
            vec![
                "9999so".to_string(),
                "123p".to_string(),
                "SSSw".to_string(),
                "EEEEw".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_sankantsu());
    }

    #[test]
    fn yaku_honroutou() {
        let out = Hand::new_from_strings(
            vec![
                "999s".to_string(),
                "111p".to_string(),
                "SSSw".to_string(),
                "EEEw".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_honroutou());

        let out = Hand::new_from_strings(
            vec![
                "999s".to_string(),
                "123p".to_string(),
                "SSSw".to_string(),
                "EEEw".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_honroutou());

        let out = Hand::new_from_strings(
            vec![
                "999s".to_string(),
                "111p".to_string(),
                "111s".to_string(),
                "999m".to_string(),
                "99p".to_string(),
            ],
            "9p".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_honroutou());
    }

    #[test]
    fn yaku_junchantaiyao() {
        let out = Hand::new_from_strings(
            vec![
                "123p".to_string(),
                "123p".to_string(),
                "999m".to_string(),
                "789s".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_junchantaiyao());

        let out = Hand::new_from_strings(
            vec![
                "123p".to_string(),
                "123p".to_string(),
                "999m".to_string(),
                "789s".to_string(),
                "ggd".to_string(),
            ],
            "gd".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_junchantaiyao());

        let out = Hand::new_from_strings(
            vec![
                "111s".to_string(),
                "111m".to_string(),
                "999m".to_string(),
                "999s".to_string(),
                "11p".to_string(),
            ],
            "1p".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_junchantaiyao());
    }

    #[test]
    fn yaku_shousangen() {
        let out = Hand::new_from_strings(
            vec![
                "123p".to_string(),
                "222p".to_string(),
                "wwwwd".to_string(),
                "rrd".to_string(),
                "gggd".to_string(),
            ],
            "gd".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_shousangen());
        let out = Hand::new_from_strings(
            vec![
                "123p".to_string(),
                "222p".to_string(),
                "wwwwd".to_string(),
                "rrd".to_string(),
                "234p".to_string(),
            ],
            "4p".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_shousangen());
    }

    #[test]
    fn yaku_honitsu() {
        let out = Hand::new_from_strings(
            vec![
                "123p".to_string(),
                "222p".to_string(),
                "567p".to_string(),
                "rrd".to_string(),
                "gggd".to_string(),
            ],
            "gd".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_honitsu());

        let out = Hand::new_from_strings(
            vec![
                "123p".to_string(),
                "222p".to_string(),
                "567m".to_string(),
                "rrd".to_string(),
                "gggd".to_string(),
            ],
            "gd".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_honitsu());

        let out = Hand::new_from_strings(
            vec![
                "123p".to_string(),
                "222p".to_string(),
                "567p".to_string(),
                "111p".to_string(),
                "33p".to_string(),
            ],
            "3p".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        //should be chinitsu not honitsu
        assert!(!out.is_honitsu());
    }

    #[test]
    fn yaku_sanankou() {
        let out = Hand::new_from_strings(
            vec![
                "123p".to_string(),
                "222p".to_string(),
                "555m".to_string(),
                "11s".to_string(),
                "333p".to_string(),
            ],
            "3p".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_sanankou(false));

        let out = Hand::new_from_strings(
            vec![
                "123p".to_string(),
                "222p".to_string(),
                "555m".to_string(),
                "11s".to_string(),
                "333p".to_string(),
            ],
            "3p".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_sanankou(true));

        let out = Hand::new_from_strings(
            vec![
                "123p".to_string(),
                "123s".to_string(),
                "555m".to_string(),
                "11s".to_string(),
                "333p".to_string(),
            ],
            "3p".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_sanankou(true));
    }

    #[test]
    fn yaku_sanshokudoujun() {
        let out = Hand::new_from_strings(
            vec![
                "rrrdo".to_string(),
                "234p".to_string(),
                "234m".to_string(),
                "234s".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_sanshokudoujun());

        let out = Hand::new_from_strings(
            vec![
                "678s".to_string(),
                "234p".to_string(),
                "234m".to_string(),
                "234s".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_sanshokudoujun());

        let out = Hand::new_from_strings(
            vec![
                "111p".to_string(),
                "234p".to_string(),
                "234m".to_string(),
                "rrrd".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_sanshokudoujun());
        let out = Hand::new_from_strings(
            vec![
                "678p".to_string(),
                "234s".to_string(),
                "234p".to_string(),
                "234p".to_string(),
                "11p".to_string(),
            ],
            "1p".to_string(),
            "Ew".to_string(),
            "Ew".to_string(),
        )
        .unwrap();
        assert!(!out.is_sanshokudoujun());
        let out = Hand::new_from_strings(
            vec![
                "234m".to_string(),
                "234s".to_string(),
                "234p".to_string(),
                "234p".to_string(),
                "11p".to_string(),
            ],
            "1p".to_string(),
            "Ew".to_string(),
            "Ew".to_string(),
        )
        .unwrap();
        assert!(out.is_sanshokudoujun());
    }

    #[test]
    fn yaku_toitoi() {
        let out = Hand::new_from_strings(
            vec![
                "rrrdo".to_string(),
                "EEEw".to_string(),
                "gggd".to_string(),
                "222p".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_toitoi());

        let out = Hand::new_from_strings(
            vec![
                "rrrdo".to_string(),
                "2222m".to_string(),
                "EEEw".to_string(),
                "11s".to_string(),
                "gggd".to_string(),
            ],
            "gd".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_toitoi());

        let out = Hand::new_from_strings(
            vec![
                "rrrdo".to_string(),
                "2222m".to_string(),
                "EEEw".to_string(),
                "11s".to_string(),
                "456so".to_string(),
            ],
            "5s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_toitoi());
    }

    #[test]
    fn yaku_yakuhai() {
        let out = Hand::new_from_strings(
            vec![
                "rrrdo".to_string(),
                "234m".to_string(),
                "22s".to_string(),
                "234m".to_string(),
                "678m".to_string(),
            ],
            "7m".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_yakuhai(), 1);
        let out = Hand::new_from_strings(
            vec![
                "EEEEwo".to_string(),
                "234m".to_string(),
                "22s".to_string(),
                "234m".to_string(),
                "rrrd".to_string(),
            ],
            "rd".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_yakuhai(), 2);
        let out = Hand::new_from_strings(
            vec![
                "3333mo".to_string(),
                "WWWw".to_string(),
                "22s".to_string(),
                "234m".to_string(),
                "678m".to_string(),
            ],
            "7m".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_yakuhai(), 1);
        let out = Hand::new_from_strings(
            vec![
                "3333mo".to_string(),
                "222m".to_string(),
                "22s".to_string(),
                "234m".to_string(),
                "678m".to_string(),
            ],
            "7m".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_yakuhai(), 0);
    }

    #[test]
    fn yaku_ryanpeikou() {
        let out = Hand::new_from_strings(
            vec![
                "123s".to_string(),
                "123s".to_string(),
                "789m".to_string(),
                "789m".to_string(),
                "77m".to_string(),
            ],
            "7m".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        //is open
        assert!(out.is_ryanpeikou());

        let out = Hand::new_from_strings(
            vec![
                "123s".to_string(),
                "123s".to_string(),
                "789m".to_string(),
                "678m".to_string(),
                "77m".to_string(),
            ],
            "7m".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        //is open
        assert!(!out.is_ryanpeikou());

        let out = Hand::new_from_strings(
            vec![
                "123s".to_string(),
                "123s".to_string(),
                "123s".to_string(),
                "678m".to_string(),
                "77m".to_string(),
            ],
            "7m".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_ryanpeikou());
    }

    #[test]
    fn yaku_iipeko() {
        let out = Hand::new_from_strings(
            vec![
                "123s".to_string(),
                "123s".to_string(),
                "789m".to_string(),
                "789m".to_string(),
                "77m".to_string(),
            ],
            "7m".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        //is open
        assert!(!out.is_iipeikou());

        let out = Hand::new_from_strings(
            vec![
                "rrrdo".to_string(),
                "234m".to_string(),
                "22s".to_string(),
                "234m".to_string(),
                "678m".to_string(),
            ],
            "7m".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        //is open
        assert!(!out.is_iipeikou());

        let out = Hand::new_from_strings(
            vec![
                "rrrd".to_string(),
                "234m".to_string(),
                "22s".to_string(),
                "234m".to_string(),
                "678m".to_string(),
            ],
            "7m".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_iipeikou());
    }

    #[test]
    fn yaku_tanyao() {
        let out = Hand::new_from_strings(
            vec![
                "rrrdo".to_string(),
                "5555mo".to_string(),
                "22s".to_string(),
                "8888s".to_string(),
                "678m".to_string(),
            ],
            "7m".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_tanyao());
        let out = Hand::new_from_strings(
            vec![
                "333mo".to_string(),
                "5555mo".to_string(),
                "11s".to_string(),
                "8888s".to_string(),
                "678m".to_string(),
            ],
            "7m".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(!out.is_tanyao());
        let out = Hand::new_from_strings(
            vec![
                "555mo".to_string(),
                "678p".to_string(),
                "22s".to_string(),
                "333s".to_string(),
                "345m".to_string(),
            ],
            "4m".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert!(out.is_tanyao());
    }

    #[test]
    fn invalid_group_sequence_not_in_order() {
        let out = Hand::new_from_strings(
            vec![
                "135m".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "Sw".to_string(),
            ],
            "3s".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        );
        assert_eq!(out.unwrap_err(), HandErr::InvalidGroup);
    }

    #[test]
    fn invalid_group_size_too_small() {
        let out = Hand::new_from_strings(
            vec![
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "Sw".to_string(),
                "ShSo".to_string(),
            ],
            "3s".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        );
        assert_eq!(out.unwrap_err(), HandErr::InvalidSuit);
    }

    #[test]
    fn invalid_group_size_too_big() {
        let out = Hand::new_from_strings(
            vec![
                "SSSSSw".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSw".to_string(),
                "ShSo".to_string(),
            ],
            "3s".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        );
        assert_eq!(out.unwrap_err(), HandErr::InvalidGroup);
    }

    #[test]
    fn invalid_suit() {
        let out = Hand::new_from_strings(
            vec![
                "hhhjo".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSw".to_string(),
            ],
            "3s".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        );
        assert_eq!(out.unwrap_err(), HandErr::InvalidSuit);
    }

    #[test]
    fn hand_too_small() {
        let out = Hand::new_from_strings(
            vec!["SSSw".to_string()],
            "3s".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        );
        assert_eq!(out.unwrap_err(), HandErr::InvalidShape);
        let out = Hand::new_from_strings(
            vec!["SSSw".to_string()],
            "3s".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        );
        assert_eq!(out.unwrap_err(), HandErr::InvalidShape);
    }
    #[test]
    fn hand_too_big() {
        let out = Hand::new_from_strings(
            vec![
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSw".to_string(),
                "SSSw".to_string(),
            ],
            "Sw".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        );
        assert_eq!(out.unwrap_err(), HandErr::InvalidShape);
    }
}

#[cfg(test)]
mod tile_group_tests {
    use super::Hand;
    use crate::suit::Suit;
    use crate::tile::*;
    use crate::tile_group::GroupType;

    #[test]
    fn identify_pair() {
        let out = Hand::new_from_strings(
            vec![
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSw".to_string(),
                "SSSw".to_string(),
            ],
            "Sw".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        )
        .unwrap();
        assert_eq!(out.pairs()[0].value(), SOUTH_VALUE);
        assert_eq!(out.pairs()[0].group_type(), GroupType::Pair);
        assert_eq!(out.pairs()[0].suit(), Suit::Wind);
        assert!(!out.pairs()[0].isopen());
    }

    #[test]
    #[allow(non_snake_case)]
    fn identify_tilegroup_closed_wind_trip_SSS() {
        let out = Hand::new_from_strings(
            vec![
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSw".to_string(),
                "SSSw".to_string(),
            ],
            "Sw".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        )
        .unwrap();
        assert_eq!(out.triplets()[0].value(), SOUTH_VALUE);
        assert_eq!(out.triplets()[0].group_type(), GroupType::Triplet);
        assert_eq!(out.triplets()[0].suit(), Suit::Wind);
        assert!(!out.triplets()[0].isopen());
    }

    #[test]
    #[allow(non_snake_case)]
    fn identify_tilegroup_open_wind_kan_EEEE() {
        let out = Hand::new_from_strings(
            vec![
                "EEEEwo".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSw".to_string(),
                "SSSw".to_string(),
            ],
            "Sw".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        )
        .unwrap();
        assert_eq!(out.kans()[0].value(), EAST_VALUE);
        assert_eq!(out.kans()[0].group_type(), GroupType::Kan);
        assert_eq!(out.kans()[0].suit(), Suit::Wind);
        assert!(out.kans()[0].isopen());
    }

    #[test]
    #[allow(non_snake_case)]
    fn identify_tilegroup_closed_dragon_kan_RRRR() {
        let out = Hand::new_from_strings(
            vec![
                "rrrrd".to_string(),
                "rrrrd".to_string(),
                "SSw".to_string(),
                "rrrrd".to_string(),
                "rrrd".to_string(),
            ],
            "rd".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        )
        .unwrap();
        assert_eq!(out.kans()[0].value(), RED_VALUE);
        assert_eq!(out.kans()[0].group_type(), GroupType::Kan);
        assert_eq!(out.kans()[0].suit(), Suit::Dragon);
        assert!(!out.kans()[0].isopen());
    }

    #[test]
    fn identify_tilegroup_closed_manzu_trip_111() {
        let out = Hand::new_from_strings(
            vec![
                "111m".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSw".to_string(),
                "SSSw".to_string(),
            ],
            "Sw".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        )
        .unwrap();
        assert_eq!(out.triplets()[0].value(), ONE_VALUE);
        assert_eq!(out.triplets()[0].group_type(), GroupType::Triplet);
        assert_eq!(out.triplets()[0].suit(), Suit::Manzu);
        assert!(!out.triplets()[0].isopen());
    }

    #[test]
    fn identify_tilegroup_closed_souzu_seq_789() {
        let out = Hand::new_from_strings(
            vec![
                "789s".to_string(),
                "SSSw".to_string(),
                "SSw".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
            ],
            "Sw".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        )
        .unwrap();
        assert_eq!(out.sequences()[0].value(), SEVEN_VALUE);
        assert_eq!(out.sequences()[0].group_type(), GroupType::Sequence);
        assert_eq!(out.sequences()[0].suit(), Suit::Souzu);
        assert!(!out.sequences()[0].isopen());
    }

    #[test]
    fn identify_tilegroup_open_pinzu_234() {
        let out = Hand::new_from_strings(
            vec![
                "234po".to_string(),
                "SSSw".to_string(),
                "SSw".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
            ],
            "Sw".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        )
        .unwrap();
        assert_eq!(out.sequences()[0].value(), TWO_VALUE);
        assert_eq!(out.sequences()[0].group_type(), GroupType::Sequence);
        assert_eq!(out.sequences()[0].suit(), Suit::Pinzu);
        assert!(out.sequences()[0].isopen());
    }
    #[test]
    fn dora_count_all() {
        let out = Hand::new_from_strings(
            vec![
                "123p".to_string(),
                "505s".to_string(),
                "EEEw".to_string(),
                "9999m".to_string(),
                "rrd".to_string(),
            ],
            "rd".to_string(),
            "Ew".to_string(),
            "Ew".to_string(),
        )
        .unwrap();
        let dora_1: Tile = "1p".to_string().try_into().unwrap();
        let dora_2: Tile = "4s".to_string().try_into().unwrap();
        let dora_3: Tile = "Nw".to_string().try_into().unwrap();
        let dora_4: Tile = "8m".to_string().try_into().unwrap();
        let dora_5: Tile = "gd".to_string().try_into().unwrap();
        let doras = vec![dora_1, dora_2, dora_3, dora_4, dora_5];

        let dora = out.get_dora_count(&Some(doras));
        assert_eq!(dora, 14);
    }
    #[test]
    fn dora_count_aka() {
        let out = Hand::new_from_strings(
            vec![
                "123p".to_string(),
                "505s".to_string(),
                "EEEw".to_string(),
                "9999m".to_string(),
                "rrd".to_string(),
            ],
            "rd".to_string(),
            "Ew".to_string(),
            "Ew".to_string(),
        )
        .unwrap();
        let dora = out.get_dora_count(&None);
        assert_eq!(dora, 1);
    }
}
