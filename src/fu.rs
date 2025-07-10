use crate::score::FuValue;

#[repr(C)]
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

impl std::fmt::Display for Fu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BasePoints => write!(f, "BasePoints: 20"),
            Self::BasePointsChitoi => write!(f, "BasePoints: 25"),
            Self::ClosedRon => write!(f, "ClosedRon: 10"),
            Self::Tsumo => write!(f, "Tsumo: 2"),
            Self::NonSimpleClosedTriplet => write!(f, "NonSimpleClosedTriplet: 8"),
            Self::SimpleClosedTriplet => write!(f, "ClosedTriplet: 4"),
            Self::NonSimpleOpenTriplet => write!(f, "NonSimpleOpenTriplet: 4"),
            Self::SimpleOpenTriplet => write!(f, "OpenTriplet: 2"),
            Self::NonSimpleClosedKan => write!(f, "NonSimpleClosedKan: 32"),
            Self::SimpleClosedKan => write!(f, "ClosedKan: 16"),
            Self::NonSimpleOpenKan => write!(f, "NonSimpleOpenKan: 16"),
            Self::SimpleOpenKan => write!(f, "OpenKan: 8"),
            Self::Toitsu => write!(f, "Toitsu: 2"),
            Self::SingleWait => write!(f, "SingleWait: 2"),
        }
    }
}

impl Fu {
    /// Get the minipoint value.
    pub fn value(&self) -> FuValue {
        match self {
            Self::BasePoints => 20,
            Self::BasePointsChitoi => 25,
            Self::ClosedRon => 10,
            Self::Tsumo => 2,
            Self::NonSimpleClosedTriplet => 8,
            Self::SimpleClosedTriplet => 4,
            Self::NonSimpleOpenTriplet => 4,
            Self::SimpleOpenTriplet => 2,
            Self::NonSimpleClosedKan => 32,
            Self::SimpleClosedKan => 16,
            Self::NonSimpleOpenKan => 16,
            Self::SimpleOpenKan => 8,
            Self::Toitsu => 2,
            Self::SingleWait => 2,
        }
    }
}

/// Sum up all of the fu, rounding to the nearest 10.
pub fn calculate_total_fu_value(fu: &[Fu]) -> FuValue {
    fu.iter().map(|f| f.value()).sum::<FuValue>().div_ceil(10) * 10
}

#[cfg(test)]
mod tests {
    use super::{Fu, calculate_total_fu_value};
    use crate::hand::Hand;

    #[test]
    fn fu_calc_simpleopenkan_simpleclosedkan() {
        let out = Hand::new_from_strings(
            vec![
                "rrrdo".to_string(),
                "5555mo".to_string(),
                "11s".to_string(),
                "8888s".to_string(),
                "789m".to_string(),
            ],
            "7m".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        let actual_fu = out.calculate_fu(true);
        assert_eq!(calculate_total_fu_value(&actual_fu), 60);
        assert_eq!(
            actual_fu,
            [
                Fu::BasePoints,
                Fu::Tsumo,
                Fu::NonSimpleOpenTriplet,
                Fu::SimpleOpenKan,
                Fu::SimpleClosedKan,
                Fu::SingleWait,
            ]
        );
    }

    #[test]
    fn fu_calc_edge_wait() {
        let out = Hand::new_from_strings(
            vec![
                "555po".to_string(),
                "234m".to_string(),
                "11s".to_string(),
                "rrrdo".to_string(),
                "789m".to_string(),
            ],
            "7m".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        let actual_fu = out.calculate_fu(true);
        assert_eq!(calculate_total_fu_value(&actual_fu), 30);
        assert_eq!(
            actual_fu,
            [
                Fu::BasePoints,
                Fu::Tsumo,
                Fu::SimpleOpenTriplet,
                Fu::NonSimpleOpenTriplet,
                Fu::SingleWait,
            ]
        );
    }

    #[test]
    fn random_fu() {
        let out = Hand::new_from_strings(
            vec![
                "rrrdo".to_string(),
                "567m".to_string(),
                "567p".to_string(),
                "55s".to_string(),
                "456s".to_string(),
            ],
            "6s".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        let actual_fu = out.calculate_fu(true);
        assert_eq!(calculate_total_fu_value(&actual_fu), 30);
        assert_eq!(
            actual_fu,
            [Fu::BasePoints, Fu::Tsumo, Fu::NonSimpleOpenTriplet,]
        );
    }

    #[test]
    fn fu_cal_middle_wait() {
        let out = Hand::new_from_strings(
            vec![
                "123mo".to_string(),
                "rrrrdo".to_string(),
                "EEEEw".to_string(),
                "WWw".to_string(),
                "456p".to_string(),
            ],
            "5p".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        let actual_fu = out.calculate_fu(true);
        assert_eq!(calculate_total_fu_value(&actual_fu), 80);
        assert_eq!(
            actual_fu,
            [
                Fu::BasePoints,
                Fu::Tsumo,
                Fu::NonSimpleOpenKan,
                Fu::NonSimpleClosedKan,
                Fu::Toitsu,
                Fu::SingleWait,
            ]
        );
    }

    #[test]
    fn fu_cal_kans_seat_wind() {
        let out = Hand::new_from_strings(
            vec![
                "123mo".to_string(),
                "rrrrdo".to_string(),
                "456po".to_string(),
                "EEEEw".to_string(),
                "WWw".to_string(),
            ],
            "Ww".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        let actual_fu = out.calculate_fu(true);
        assert_eq!(calculate_total_fu_value(&actual_fu), 80);
        assert_eq!(
            actual_fu,
            [
                Fu::BasePoints,
                Fu::Tsumo,
                Fu::NonSimpleOpenKan,
                Fu::NonSimpleClosedKan,
                Fu::Toitsu,
                Fu::SingleWait,
            ]
        );
    }

    #[test]
    fn fu_cal_nontimple_closed_trip() {
        let out = Hand::new_from_strings(
            vec![
                "111mo".to_string(),
                "rrrd".to_string(),
                "345s".to_string(),
                "11s".to_string(),
                "EEEw".to_string(),
            ],
            "Ew".to_string(),
            "Ew".to_string(),
            "Ew".to_string(),
        )
        .unwrap();
        let actual_fu = out.calculate_fu(false);
        assert_eq!(calculate_total_fu_value(&actual_fu), 40);
        assert_eq!(
            actual_fu,
            [
                Fu::BasePoints,
                Fu::NonSimpleOpenTriplet,
                Fu::NonSimpleClosedTriplet,
                Fu::NonSimpleOpenTriplet
            ]
        );
    }

    #[test]
    fn fu_cal_tsu_singlewait_simple_trip_closed_simple_trip_closed_nonsimple_kan() {
        let out = Hand::new_from_strings(
            vec![
                "444m".to_string(),
                "789p".to_string(),
                "555so".to_string(),
                "rrrrd".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Ew".to_string(),
            "Ew".to_string(),
        )
        .unwrap();
        let actual_fu = out.calculate_fu(true);
        assert_eq!(calculate_total_fu_value(&actual_fu), 70);
        assert_eq!(
            actual_fu,
            [
                Fu::BasePoints,
                Fu::Tsumo,
                Fu::SimpleClosedTriplet,
                Fu::SimpleOpenTriplet,
                Fu::NonSimpleClosedKan,
                Fu::SingleWait,
            ]
        );
    }

    #[test]
    fn fu_cal_single_wait_sequence_with_terminal() {
        let out = Hand::new_from_strings(
            vec![
                "444m".to_string(),
                "555so".to_string(),
                "rrrrd".to_string(),
                "11s".to_string(),
                "789p".to_string(),
            ],
            "8p".to_string(),
            "Ew".to_string(),
            "Ew".to_string(),
        )
        .unwrap();
        let actual_fu = out.calculate_fu(true);
        assert_eq!(calculate_total_fu_value(&actual_fu), 70);
        assert_eq!(
            actual_fu,
            [
                Fu::BasePoints,
                Fu::Tsumo,
                Fu::SimpleClosedTriplet,
                Fu::SimpleOpenTriplet,
                Fu::NonSimpleClosedKan,
                Fu::SingleWait,
            ]
        );
    }

    #[test]
    fn fu_cal_single_wait_sequence_not_terminal() {
        let out = Hand::new_from_strings(
            vec![
                "444m".to_string(),
                "555so".to_string(),
                "rrrrd".to_string(),
                "11s".to_string(),
                "456p".to_string(),
            ],
            "5p".to_string(),
            "Ew".to_string(),
            "Ew".to_string(),
        )
        .unwrap();
        let actual_fu = out.calculate_fu(true);
        assert_eq!(calculate_total_fu_value(&actual_fu), 70);
        assert_eq!(
            actual_fu,
            [
                Fu::BasePoints,
                Fu::Tsumo,
                Fu::SimpleClosedTriplet,
                Fu::SimpleOpenTriplet,
                Fu::NonSimpleClosedKan,
                Fu::SingleWait,
            ]
        );
    }

    #[test]
    fn fu_cal_toitsu() {
        let out = Hand::new_from_strings(
            vec![
                "444m".to_string(),
                "555so".to_string(),
                "rrrrd".to_string(),
                "456p".to_string(),
                "EEw".to_string(),
            ],
            "Ew".to_string(),
            "Ew".to_string(),
            "Ew".to_string(),
        )
        .unwrap();
        let actual_fu = out.calculate_fu(true);
        assert_eq!(calculate_total_fu_value(&actual_fu), 70);
        assert_eq!(
            actual_fu,
            [
                Fu::BasePoints,
                Fu::Tsumo,
                Fu::SimpleClosedTriplet,
                Fu::SimpleOpenTriplet,
                Fu::NonSimpleClosedKan,
                Fu::Toitsu,
                Fu::SingleWait,
            ]
        );

        let out = Hand::new_from_strings(
            vec![
                "444m".to_string(),
                "555so".to_string(),
                "rrrrd".to_string(),
                "456p".to_string(),
                "rrd".to_string(),
            ],
            "rd".to_string(),
            "Ew".to_string(),
            "Ew".to_string(),
        )
        .unwrap();
        let actual_fu = out.calculate_fu(true);
        assert_eq!(calculate_total_fu_value(&actual_fu), 70);
        assert_eq!(
            actual_fu,
            [
                Fu::BasePoints,
                Fu::Tsumo,
                Fu::SimpleClosedTriplet,
                Fu::SimpleOpenTriplet,
                Fu::NonSimpleClosedKan,
                Fu::Toitsu,
                Fu::SingleWait,
            ]
        );

        let out = Hand::new_from_strings(
            vec![
                "444m".to_string(),
                "555so".to_string(),
                "rrrrd".to_string(),
                "456p".to_string(),
                "WWw".to_string(),
            ],
            "Ww".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        let actual_fu = out.calculate_fu(true);
        assert_eq!(calculate_total_fu_value(&actual_fu), 70);
        assert_eq!(
            actual_fu,
            [
                Fu::BasePoints,
                Fu::Tsumo,
                Fu::SimpleClosedTriplet,
                Fu::SimpleOpenTriplet,
                Fu::NonSimpleClosedKan,
                Fu::Toitsu,
                Fu::SingleWait,
            ]
        );
    }

    #[test]
    fn fu_cal_tsu_closed() {
        let out = Hand::new_from_strings(
            vec![
                "EEEw".to_string(),
                "WWWw".to_string(),
                "123p".to_string(),
                "123s".to_string(),
                "11m".to_string(),
            ],
            "1m".to_string(),
            "Ew".to_string(),
            "Ew".to_string(),
        )
        .unwrap();
        let actual_fu = out.calculate_fu(true);
        assert_eq!(calculate_total_fu_value(&actual_fu), 40);
        assert_eq!(
            actual_fu,
            [
                Fu::BasePoints,
                Fu::Tsumo,
                Fu::NonSimpleClosedTriplet,
                Fu::NonSimpleClosedTriplet,
                Fu::SingleWait,
            ]
        );
    }
}
