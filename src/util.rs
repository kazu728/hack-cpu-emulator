use crate::bit::{to_decimal, Bit};

pub fn parse_address(adderss: &[Bit]) -> (&[Bit], usize) {
    let higher_bit: &[Bit] = &adderss[0..3];
    let lower_bit: &[Bit] = &adderss[3..];

    let index = to_decimal(higher_bit);

    (lower_bit, index)
}
