pub mod calc;
pub mod fu;
pub mod hand;
pub mod limit_hand;
pub mod payment;
pub mod round_context;
pub mod score;
pub mod suit;
pub mod tile_group;
pub mod yaku;

/// Characters that represent terminal or honor tiles.
// Using a fixed array gets stored on the stack rather than a `String` which gets stored on the heap.
const TERMINAL_CHARS: [char; 9] = ['1', '9', 'E', 'S', 'W', 'N', 'r', 'g', 'w'];
