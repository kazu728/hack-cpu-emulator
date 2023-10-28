use crate::bit::{ Bit, BitSliceToUsize};

pub fn parse_address(adderss: &[Bit]) -> (&[Bit], usize) {
    let higher_bit: &[Bit] = &adderss[0..3];
    let lower_bit: &[Bit] = &adderss[3..];

    let index = higher_bit.to_usize();

    (lower_bit, index)
}
