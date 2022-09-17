pub fn transform_from_byte_to_usize(bits: &[u8]) -> usize {
    match bits {
        [0, 0, 0] => 0,
        [0, 0, 1] => 1,
        [0, 1, 0] => 2,
        [0, 1, 1] => 3,
        [1, 0, 0] => 4,
        [1, 0, 1] => 5,
        [1, 1, 0] => 6,
        [1, 1, 1] => 7,
        [1, 1, 1] => 7,
        _ => unreachable!(),
    }
}

pub fn parse_address(adderss: &[u8]) -> (&[u8], usize) {
    let higher_bit: &[u8] = &adderss[0..3];
    let lower_bit: &[u8] = &adderss[3..];

    let index = transform_from_byte_to_usize(higher_bit);

    (lower_bit, index)
}
