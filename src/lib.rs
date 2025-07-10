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

const VALID_SEQUENCE_VALUES: &[&str] = &[
    "123", "234", "345", "456", "567", "678", "789", "340", "406", "067",
];

#[unsafe(no_mangle)]
extern "C" fn sanity_check() {
    println!("Hello from the rust world!");
}
