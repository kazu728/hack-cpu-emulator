use std::convert::TryInto;

use crate::bit::Bit;
use crate::bit::Bit::{I, O};

pub fn transform_from_byte_to_usize(bits: &[Bit]) -> usize {
    match bits {
        [O, O, O] => 0,
        [O, O, I] => 1,
        [O, I, O] => 2,
        [O, I, I] => 3,
        [I, O, O] => 4,
        [I, O, I] => 5,
        [I, I, O] => 6,
        [I, I, I] => 7,
        _ => unreachable!(),
    }
}

pub fn parse_address(adderss: &[Bit]) -> (&[Bit], usize) {
    let higher_bit: &[Bit] = &adderss[0..3];
    let lower_bit: &[Bit] = &adderss[3..];

    let index = transform_from_byte_to_usize(higher_bit);

    (lower_bit, index)
}
