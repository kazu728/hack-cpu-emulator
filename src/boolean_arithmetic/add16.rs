use crate::bit::Bit;

use super::{full_adder, half_adder};

pub fn add16(a: [Bit; 16], b: [Bit; 16]) -> [Bit; 16] {
    let (carry_15, sum_15) = half_adder(a[15], b[15]);
    let (carry_14, sum_14) = full_adder(a[14], b[14], carry_15);
    let (carry_13, sum_13) = full_adder(a[13], b[13], carry_14);
    let (carry_12, sum_12) = full_adder(a[12], b[12], carry_13);
    let (carry_11, sum_11) = full_adder(a[11], b[11], carry_12);
    let (carry_10, sum_10) = full_adder(a[10], b[10], carry_11);
    let (carry_9, sum_9) = full_adder(a[9], b[9], carry_10);
    let (carry_8, sum_8) = full_adder(a[8], b[8], carry_9);
    let (carry_7, sum_7) = full_adder(a[7], b[7], carry_8);
    let (carry_6, sum_6) = full_adder(a[6], b[6], carry_7);
    let (carry_5, sum_5) = full_adder(a[5], b[5], carry_6);
    let (carry_4, sum_4) = full_adder(a[4], b[4], carry_5);
    let (carry_3, sum_3) = full_adder(a[3], b[3], carry_4);
    let (carry_2, sum_2) = full_adder(a[2], b[2], carry_3);
    let (carry_1, sum_1) = full_adder(a[1], b[1], carry_2);
    let (_, sum_0) = full_adder(a[0], b[0], carry_1);

    [
        sum_0, sum_1, sum_2, sum_3, sum_4, sum_5, sum_6, sum_7, sum_8, sum_9, sum_10, sum_11,
        sum_12, sum_13, sum_14, sum_15,
    ]
}

#[cfg(test)]
mod tests {
    use crate::bit::Bit::{I, O};

    use super::add16;

    #[test]
    fn test_add16() {
        let a = [O, I, I, O, I, O, I, O, O, O, I, I, O, I, O, I];
        let b = [O, I, I, I, O, O, I, O, I, I, I, I, O, I, I, I];

        let output = [I, I, O, I, I, I, O, I, O, O, I, O, I, I, O, O];

        assert_eq!(add16(a, b), output);
    }
}
