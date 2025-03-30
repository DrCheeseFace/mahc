pub mod calc;
pub mod fu;
pub mod hand;
pub mod limit_hand;
pub mod payment;
pub mod score;
pub mod suit;
pub mod tile;
pub mod tile_group;
pub mod yaku;

const EAST_VALUE: char = 'E';
const SOUTH_VALUE: char = 'S';
const WEST_VALUE: char = 'W';
const NORTH_VALUE: char = 'N';

const EAST_VALUE_Z: char = '1';
const SOUTH_VALUE_Z: char = '2';
const WEST_VALUE_Z: char = '3';
const NORTH_VALUE_Z: char = '4';

const RED_VALUE: char = 'r';
const GREEN_VALUE: char = 'g';
const WHITE_VALUE: char = 'w';

const RED_VALUE_Z: char = '7';
const GREEN_VALUE_Z: char = '6';
const WHITE_VALUE_Z: char = '5';

const ONE_VALUE: char = '1';
const TWO_VALUE: char = '2';
const THREE_VALUE: char = '3';
const FOUR_VALUE: char = '4';
const FIVE_VALUE: char = '5';
const AKAFIVE_VALUE: char = '0';
const SIX_VALUE: char = '6';
const SEVEN_VALUE: char = '7';
const EIGHT_VALUE: char = '8';
const NINE_VALUE: char = '9';

const OPEN_CHAR: char = 'o';

const MAN_SUIT_CHAR: char = 'm';
const PIN_SUIT_CHAR: char = 'p';
const SOU_SUIT_CHAR: char = 's';

const DRAGON_SUIT_CHAR: char = 'd';
const WIND_SUIT_CHAR: char = 'w';
const Z_SUIT_CHAR: char = 'z';

const VALID_SEQUENCE_VALUES: &[&str] = &[
    "123", "234", "345", "456", "567", "678", "789", "340", "406", "067",
];
