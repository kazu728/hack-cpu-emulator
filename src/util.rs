use crate::bit::Bit;
use crate::bit::Bit::{I, O};

pub fn transform_from_byte_to_usize(bits: [Bit; 3]) -> usize {
    match bits {
        [O, O, O] => 0,
        [O, O, I] => 1,
        [O, I, O] => 2,
        [O, I, I] => 3,
        [I, O, O] => 4,
        [I, O, I] => 5,
        [I, I, O] => 6,
        [I, I, I] => 7,
    }
}
