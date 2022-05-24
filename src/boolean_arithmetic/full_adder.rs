use super::half_adder::half_adder;
use crate::{bit::Bit, boolean_gate::or};

pub fn full_adder(a: Bit, b: Bit, c: Bit) -> (Bit, Bit) {
    let (carry_ab, sum_ab) = half_adder(a, b);
    let (carry_abc, sum) = half_adder(sum_ab, c);

    let carry = or(carry_ab, carry_abc);

    (carry, sum)
}

#[cfg(test)]
mod tests {
    use super::full_adder;
    use crate::bit::Bit::{I, O};

    #[test]
    fn test_full_adder() {
        assert_eq!(full_adder(O, O, O), (O, O));
        assert_eq!(full_adder(O, O, I), (O, I));
        assert_eq!(full_adder(O, I, O), (O, I));
        assert_eq!(full_adder(O, I, I), (I, O));
        assert_eq!(full_adder(I, O, O), (O, I));
        assert_eq!(full_adder(I, O, I), (I, O));
        assert_eq!(full_adder(I, I, O), (I, O));
        assert_eq!(full_adder(I, I, I), (I, I));
    }
}
