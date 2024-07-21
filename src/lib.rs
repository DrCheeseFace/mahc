#[derive(Debug)]
pub enum LimitHands {
    Mangan,
    Haneman,
    Baiman,
    Sanbaiman,
    KazoeYakuman,
}

#[derive(Debug)]
pub struct Hand {
    groups: Vec<TileGroup>,
    win_tile: TileGroup,
    seat_tile: TileGroup,
    prev_tile: TileGroup,
    isopen: bool,
}
#[derive(Debug, PartialEq)]
pub enum HandErr {
    InvalidGroup,
    InvalidSuit,
    InvalidShape,
}

#[derive(Debug, PartialEq)]
pub enum Fu {
    BasePoints,
    BasePointsChitoi,
    ClosedRon,
    Tsumo,
    NonSimpleClosedTriplet,
    SimpleClosedTriplet,
    NonSimpleOpenTriplet,
    SimpleOpenTriplet,
    NonSimpleClosedKan,
    SimpleClosedKan,
    NonSimpleOpenKan,
    SimpleOpenKan,
    Toitsu,
    SingleWait,
}

impl Fu {
    pub fn to_string(&self) -> String {
        match self {
            Fu::BasePoints => "BasePoints: 20".to_string(),
            Fu::BasePointsChitoi => "BasePoints: 25".to_string(),
            Fu::ClosedRon => "ClosedRon: 10".to_string(),
            Fu::Tsumo => "Tsumo: 2".to_string(),
            Fu::NonSimpleClosedTriplet => "NonSimpleClosedTriplet: 8".to_string(),
            Fu::SimpleClosedTriplet => "ClosedTriplet: 4".to_string(),
            Fu::NonSimpleOpenTriplet => "NonSimpleOpenTriplet: 4".to_string(),
            Fu::SimpleOpenTriplet => "OpenTriplet: 2".to_string(),
            Fu::NonSimpleClosedKan => "NonSimpleClosedKan: 32".to_string(),
            Fu::SimpleClosedKan => "ClosedKan: 16".to_string(),
            Fu::NonSimpleOpenKan => "NonSimpleOpenKan: 16".to_string(),
            Fu::SimpleOpenKan => "OpenKan: 8".to_string(),
            Fu::Toitsu => "Toitsu: 2".to_string(),
            Fu::SingleWait => "SingleWait: 2".to_string(),
        }
    }
}

impl Hand {
    pub fn new(
        tiles: Vec<String>,
        win: String,
        prev: String,
        seat: String,
    ) -> Result<Self, HandErr> {
        let mut tile_groups: Vec<TileGroup> = Vec::new();
        let mut ishandopen = false;
        for i in &tiles {
            let tile = TileGroup::new(i.to_string())?;
            if tile.isopen {
                ishandopen = true;
            }
            tile_groups.push(tile);
        }

        //TODO: standard hand ONLY CHECK MUST FIX FOR CHITOIT AND KOKUSHI
        //TODO: this can FORSURE be shorter
        let (mut tripcount, mut seqcount, mut paircount, mut kancount) = (0, 0, 0, 0);
        for i in &tile_groups {
            if i.group_type == GroupType::Triplet {
                tripcount += 1;
            } else if i.group_type == GroupType::Sequence {
                seqcount += 1;
            } else if i.group_type == GroupType::Pair {
                paircount += 1;
            } else if i.group_type == GroupType::Kan {
                kancount += 1;
            }
        }

        if !(tripcount + seqcount + kancount == 4 && paircount == 1) && paircount != 7 {
            return Err(HandErr::InvalidShape);
        }

        // AHAHAHAHAHAHAHAHAh (these are special cases for singular tiles)
        let win_tile = TileGroup {
            value: win.chars().nth(0).unwrap().to_string(),
            suit: Suit::suit_from_string(win.chars().nth(1).unwrap().to_string())?,
            isopen: false,
            group_type: GroupType::None,
            isterminal: "19ESWNrgw".contains(win.chars().nth(0).unwrap()),
        };

        let seat_tile = TileGroup {
            value: seat.chars().nth(0).unwrap().to_string(),
            suit: Suit::suit_from_string(seat.chars().nth(1).unwrap().to_string())?,
            isopen: false,
            group_type: GroupType::None,
            isterminal: "19ESWNrgw".contains(seat.chars().nth(0).unwrap()),
        };

        let prev_tile = TileGroup {
            value: prev.chars().nth(0).unwrap().to_string(),
            suit: Suit::suit_from_string(prev.chars().nth(1).unwrap().to_string())?,
            isopen: false,
            group_type: GroupType::None,
            isterminal: "19ESWNrgw".contains(prev.chars().nth(0).unwrap()),
        };

        let hand = Hand {
            groups: tile_groups,
            win_tile,
            seat_tile,
            prev_tile,
            isopen: ishandopen,
        };

        return Ok(hand);
    }

    pub fn calculate_fu(&self, tsumo: bool) -> (u16, Vec<Fu>) {
        //TODO REMOVE THESE PRINTS (and make this calculation less fucky)
        let mut fu_types: Vec<Fu> = vec![];
        let mut totalfu = 20;
        fu_types.push(Fu::BasePoints);
        if tsumo {
            totalfu += 2;
            fu_types.push(Fu::Tsumo);
        }
        if !self.is_open() {
            totalfu += 10;
            fu_types.push(Fu::ClosedRon);
        }
        //meld fu cal
        for i in &self.triplets() {
            if i == self.groups.last().unwrap() {
                if tsumo {
                    if i.suit == Suit::Wind || i.suit == Suit::Dragon || i.isterminal {
                        fu_types.push(Fu::NonSimpleClosedTriplet);
                        totalfu += 8;
                    } else {
                        fu_types.push(Fu::SimpleClosedTriplet);
                        totalfu += 4;
                    }
                } else {
                    if i.suit == Suit::Wind || i.suit == Suit::Dragon || i.isterminal {
                        totalfu += 4;
                        fu_types.push(Fu::NonSimpleOpenTriplet);
                    } else {
                        totalfu += 2;
                        fu_types.push(Fu::SimpleOpenTriplet);
                    }
                }
                continue;
            }
            if !(i.suit == Suit::Wind || i.suit == Suit::Dragon || i.isterminal) && i.isopen {
                totalfu += 2;
                fu_types.push(Fu::SimpleOpenTriplet);
            }
            if !i.isopen {
                if i.suit == Suit::Wind || i.suit == Suit::Dragon || i.isterminal {
                    totalfu += 8;
                    fu_types.push(Fu::NonSimpleClosedTriplet);
                } else {
                    totalfu += 4;
                    fu_types.push(Fu::SimpleClosedTriplet);
                }
            } else {
                if i.suit == Suit::Wind || i.suit == Suit::Dragon || i.isterminal {
                    totalfu += 4;
                    fu_types.push(Fu::NonSimpleOpenTriplet);
                }
            }
        }
        for i in &self.kans() {
            if i.suit == Suit::Wind || i.suit == Suit::Dragon || i.isterminal {
                if !i.isopen {
                    totalfu += 32;
                    fu_types.push(Fu::NonSimpleClosedKan);
                } else {
                    fu_types.push(Fu::NonSimpleOpenKan);
                    totalfu += 16;
                }
            } else {
                if !i.isopen {
                    fu_types.push(Fu::SimpleClosedKan);
                    totalfu += 16;
                } else {
                    fu_types.push(Fu::SimpleOpenKan);
                    totalfu += 8;
                }
            }
        }
        for i in self.pairs() {
            if i.value == self.prev_tile.value
                || i.value == self.seat_tile.value
                || i.suit == Suit::Dragon
            {
                fu_types.push(Fu::Toitsu);
                totalfu += 2;
            }
        }
        //fu wait cal
        if self.groups.last().unwrap().group_type == GroupType::Pair {
            fu_types.push(Fu::SingleWait);
            totalfu += 2;
        }
        if self.groups.last().unwrap().group_type == GroupType::Sequence {
            let midtile = self.groups.last().unwrap().value.parse::<u8>().unwrap() + 1;
            if self.win_tile().value == midtile.to_string() {
                fu_types.push(Fu::SingleWait);
                totalfu += 2;
            }
            if !(self.win_tile().value == "1" || self.win_tile().value == "9")
                && self.groups.last().unwrap().isterminal
            {
                fu_types.push(Fu::SingleWait);
                totalfu += 2;
            }
        }
        //works cuz ints
        return (((totalfu + 9) / 10) * 10, fu_types);
    }

    pub fn sequences(&self) -> Vec<TileGroup> {
        let mut seqs: Vec<TileGroup> = vec![];
        for i in self.groups.clone() {
            if i.group_type == GroupType::Sequence {
                seqs.push(i);
            }
        }
        seqs
    }
    pub fn triplets(&self) -> Vec<TileGroup> {
        let mut trips: Vec<TileGroup> = vec![];
        for i in self.groups.clone() {
            if i.group_type == GroupType::Triplet {
                trips.push(i);
            }
        }
        trips
    }
    pub fn kans(&self) -> Vec<TileGroup> {
        let mut kans: Vec<TileGroup> = vec![];
        for i in self.groups.clone() {
            if i.group_type == GroupType::Kan {
                kans.push(i);
            }
        }
        kans
    }
    pub fn pairs(&self) -> Vec<TileGroup> {
        let mut pairs: Vec<TileGroup> = vec![];
        for i in self.groups.clone() {
            if i.group_type == GroupType::Pair {
                pairs.push(i);
            }
        }
        pairs
    }
    pub fn win_tile(&self) -> TileGroup {
        self.win_tile.clone()
    }
    pub fn seat_tile(&self) -> TileGroup {
        self.seat_tile.clone()
    }
    pub fn prev_tile(&self) -> TileGroup {
        self.prev_tile.clone()
    }
    pub fn is_open(&self) -> bool {
        self.isopen
    }

    //yaku validation
    pub fn is_tanyao(&self) -> bool {
        for i in self.groups.clone() {
            if i.isterminal || i.suit == Suit::Dragon || i.suit == Suit::Wind {
                return false;
            }
        }
        return true;
    }
    pub fn is_ryanpeikou(&self) -> bool {
        let mut seqs: Vec<TileGroup> = self.sequences();
        if seqs.len() != 4 {
            return false;
        }
        seqs.dedup();
        seqs.len() == 2
    }
    pub fn is_iipeikou(&self) -> bool {
        let mut seqs: Vec<TileGroup> = self.sequences();
        seqs.dedup();
        !(self.sequences().len() == seqs.len() || self.is_open() || self.is_ryanpeikou())
    }
    pub fn is_yakuhai(&self) -> u16 {
        // i do it like this because a single group can have multiple yakuhai
        let mut count = 0;
        for i in self.triplets() {
            if i.value == self.prev_tile.value {
                count += 1;
            }
            if i.value == self.seat_tile.value {
                count += 1;
            }
            if i.suit == Suit::Dragon {
                count += 1;
            }
        }
        for i in self.kans() {
            if i.value == self.prev_tile.value {
                count += 1;
            }
            if i.value == self.seat_tile.value {
                count += 1;
            }
            if i.suit == Suit::Dragon {
                count += 1;
            }
        }
        return count;
    }
    pub fn is_toitoi(&self) -> bool {
        self.triplets().len() + self.kans().len() == 4 && self.is_open()
    }
    pub fn is_sanankou(&self, tsumo: bool) -> bool {
        let mut closed_triplet_count = 0;
        for i in self.triplets() {
            if !i.isopen {
                closed_triplet_count += 1;
            }
        }
        for i in self.kans() {
            if !i.isopen {
                closed_triplet_count += 1;
            }
        }
        if !tsumo && self.groups.last().unwrap().group_type == GroupType::Triplet {
            closed_triplet_count -= 1;
        }

        closed_triplet_count == 3
    }
    pub fn is_sanshokudoujun(&self) -> bool {
        if self.sequences().len() < 3 {
            return false;
        }
        let mut list_of_vals: Vec<String> = vec![];
        for i in self.sequences() {
            list_of_vals.push(i.value.clone());
        }
        list_of_vals.dedup();

        if self.sequences().len() == 3 {
            if list_of_vals.len() == 1 {
                return true;
            }
        } else {
            if list_of_vals.len() == 2 {
                return true;
            }
        }
        return false;
    }
    pub fn is_honitsu(&self) -> bool {
        let tile_groups = self.groups.clone();
        let mut has_honor = false;
        let mut has_normal = false;
        let mut suit: Option<Suit> = None;
        for i in &tile_groups {
            if i.suit == Suit::Dragon || i.suit == Suit::Wind {
                has_honor = true;
            } else {
                has_normal = true;
                suit = Some(i.suit.clone());
            }
        }

        if !has_normal || !has_honor {
            return false;
        }

        if let Some(s) = suit {
            for i in &tile_groups {
                if i.suit != s && i.suit != Suit::Dragon && i.suit != Suit::Wind {
                    return false;
                }
            }
        } else {
            return false;
        }

        return true;
    }
    pub fn is_shousangen(&self) -> bool {
        let dragon_count = self
            .triplets()
            .iter()
            .chain(self.kans().iter())
            .filter(|i| i.suit == Suit::Dragon)
            .count();
        dragon_count == 2 && self.pairs()[0].suit == Suit::Dragon
    }
    pub fn is_junchantaiyao(&self) -> bool {
        for i in self.groups.clone() {
            if i.suit == Suit::Dragon || i.suit == Suit::Wind || !i.isterminal {
                return false;
            }
        }
        !(self.sequences().len() == 0)
    }
    pub fn is_honroutou(&self) -> bool {
        if self.sequences().len() != 0 {
            return false;
        }
        let mut has_terminal: bool = false;
        let mut has_honor: bool = false;
        for i in self.groups.clone() {
            if i.isterminal {
                has_terminal = true;
            } else if i.suit == Suit::Dragon || i.suit == Suit::Wind {
                has_honor = true;
            } else {
                return false;
            }
        }
        has_terminal && has_honor
    }
    pub fn is_sankantsu(&self) -> bool {
        self.kans().len() == 3
    }
    pub fn is_ittsuu(&self) -> bool {
        //there has GOTTO be a better way to do this
        let suits = [Suit::Pinzu, Suit::Manzu, Suit::Souzu];
        suits.iter().any(|suit| {
            let values: Vec<String> = self
                .sequences()
                .iter()
                .filter(|&x| x.suit == *suit)
                .map(|x| x.value.clone())
                .collect();
            values.contains(&"1".to_string())
                && values.contains(&"4".to_string())
                && values.contains(&"7".to_string())
        })
    }
    pub fn is_chantaiyao(&self) -> bool {
        if self.sequences().len() == 0 {
            return false;
        }
        let mut has_terminal: bool = false;
        let mut has_honor: bool = false;
        for i in self.groups.clone() {
            if i.isterminal {
                has_terminal = true;
            } else if i.suit == Suit::Dragon || i.suit == Suit::Wind {
                has_honor = true;
            } else {
                return false;
            }
        }
        has_terminal && has_honor
    }
    pub fn is_chiitoitsu(&self) -> bool {
        self.pairs().len() == 7
    }
    pub fn is_menzentsumo(&self, tsumo: bool) -> bool {
        !self.isopen && tsumo
    }
    pub fn is_pinfu(&self) -> bool {
        if self.isopen {
            return false;
        }
        let fu = self.calculate_fu(false);
        for i in fu.1 {
            if i != Fu::ClosedRon && i != Fu::BasePoints {
                return false;
            }
        }

        return true;
    }
    pub fn is_sanshokudoukou(&self) -> bool {
        if self.triplets().len() + self.kans().len() < 3 {
            return false;
        }
        let mut list_of_vals: Vec<String> = vec![];
        for i in self.triplets().iter().chain(self.kans().iter()) {
            list_of_vals.push(i.value.clone());
        }
        list_of_vals.dedup();

        if self.triplets().len() + self.kans().len() == 3 {
            if list_of_vals.len() == 1 {
                return true;
            }
        } else {
            if list_of_vals.len() == 2 {
                return true;
            }
        }
        return false;
    }
    pub fn is_chinitsu(&self) -> bool {
        let mut suits: Vec<Suit> = self.groups.iter().map(|x| x.suit.clone()).collect();
        println!("{:?}", suits);
        suits.dedup();
        suits.len() == 1
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TileGroup {
    pub value: String,
    pub suit: Suit,
    pub isopen: bool,
    pub group_type: GroupType,
    pub isterminal: bool,
}

impl TileGroup {
    pub fn new(group: String) -> Result<Self, HandErr> {
        let isopen = group.chars().last().unwrap().to_string() == "o";
        let value = group.chars().nth(0).unwrap().to_string();
        let mut suit: String = Default::default();

        if !isopen {
            suit = group.chars().last().unwrap().to_string();
        } else {
            suit = group.chars().nth(group.len() - 2).unwrap().to_string();
        }

        let suit = Suit::suit_from_string(suit)?;
        let group_type = GroupType::group_type_from_string(group.to_string())?;
        let mut isterminal = false;
        if group_type == GroupType::Sequence {
            if value == "1" || value == "7" {
                isterminal = true;
            }
        } else {
            if value == "1" || value == "9" {
                isterminal = true;
            }
        }
        let tile = TileGroup {
            value,
            suit,
            isopen,
            group_type,
            isterminal,
        };
        return Ok(tile);
    }
}

//AHAHAHAHAHAHAHAH I DONT NEED THIS
//turns our i did need this :)
#[derive(Debug, Clone, PartialEq)]
pub enum GroupType {
    Sequence,
    Triplet,
    Kan,
    Pair,
    None,
}

impl GroupType {
    pub fn group_type_from_string(group: String) -> Result<GroupType, HandErr> {
        let mut count = 0;
        if group.contains('o') {
            count = group.len() - 2;
        } else {
            count = group.len() - 1;
        }

        for i in group.get(0..count).unwrap().chars() {
            if "123456789ESWNrgw".contains(i) {
                continue;
            } else {
                return Err(HandErr::InvalidGroup);
            }
        }

        match count {
            2 => return Ok(GroupType::Pair),
            3 => {
                if group.chars().nth(0).unwrap() == group.chars().nth(1).unwrap()
                    && group.chars().nth(1).unwrap() == group.chars().nth(2).unwrap()
                {
                    return Ok(GroupType::Triplet);
                } else if ["123", "234", "345", "456", "567", "678", "789"]
                    .iter()
                    .cloned()
                    .collect::<std::collections::HashSet<&str>>()
                    .contains(group.get(0..count).unwrap())
                {
                    return Ok(GroupType::Sequence);
                } else {
                    return Err(HandErr::InvalidGroup);
                }
            }
            4 => return Ok(GroupType::Kan),
            _ => return Err(HandErr::InvalidGroup),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum Suit {
    Manzu,
    Pinzu,
    Souzu,
    Wind,
    Dragon,
}

impl Suit {
    pub fn suit_from_string(suit: String) -> Result<Suit, HandErr> {
        match suit.as_str() {
            "s" => return Ok(Suit::Souzu),
            "p" => return Ok(Suit::Pinzu),
            "m" => return Ok(Suit::Manzu),
            "w" => return Ok(Suit::Wind),
            "d" => return Ok(Suit::Dragon),
            _ => return Err(HandErr::InvalidSuit),
        }
    }
}

//TODO: MOVE THIS INTO A SUITABLE STRUCT LATER
pub fn is_limit_hand(han: u16, fu: u16) -> bool {
    if han >= 5 {
        return true;
    }
    if han == 4 && fu >= 40 {
        return true;
    }
    if han == 3 && fu >= 70 {
        return true;
    }
    return false;
}

impl LimitHands {
    //TODO: MOVE THIS INTO A SUITABLE STRUCT LATER
    pub fn get_limit_hand(han: u16, fu: u16) -> Option<LimitHands> {
        if !is_limit_hand(han, fu) {
            return None;
        }
        if han <= 5 {
            return Some(LimitHands::Mangan);
        } else if han <= 7 {
            return Some(LimitHands::Haneman);
        } else if han <= 10 {
            return Some(LimitHands::Baiman);
        } else if han <= 12 {
            return Some(LimitHands::Sanbaiman);
        } else {
            return Some(LimitHands::KazoeYakuman);
        }
    }
    pub fn get_score(&self) -> Vec<u16> {
        match self {
            LimitHands::Mangan => {
                vec![12000, 4000, 8000, 2000, 4000]
            }
            LimitHands::Haneman => {
                let vec = LimitHands::Mangan.get_score();
                let mut out: Vec<u16> = Vec::new();
                for i in vec {
                    let j = i / 2;
                    out.push(i + j)
                }
                out
            }
            LimitHands::Baiman => {
                let vec = LimitHands::Mangan.get_score();
                let mut out: Vec<u16> = Vec::new();
                for i in vec {
                    out.push(i * 2)
                }
                out
            }
            LimitHands::Sanbaiman => {
                let vec = LimitHands::Mangan.get_score();
                let mut out: Vec<u16> = Vec::new();
                for i in vec {
                    out.push(i * 3)
                }
                out
            }
            LimitHands::KazoeYakuman => {
                let vec = LimitHands::Mangan.get_score();
                let mut out: Vec<u16> = Vec::new();
                for i in vec {
                    out.push(i * 4)
                }
                out
            }
        }
    }
}
