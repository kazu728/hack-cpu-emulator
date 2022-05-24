use super::super::boolean_gate::{and, xor};
use crate::bit::Bit;

pub fn half_adder(a: Bit, b: Bit) -> (Bit, Bit) {
    let sum = xor(a, b);
    let carry = and(a, b);

    (carry, sum)
}

#[cfg(test)]
mod tests {
    use super::half_adder;
    use crate::bit::Bit::{I, O};

    #[test]
    fn test_half_adder() {
        assert_eq!(half_adder(O, O), (O, O));
        assert_eq!(half_adder(O, I), (O, I));
        assert_eq!(half_adder(I, O), (O, I));
        assert_eq!(half_adder(I, I), (I, O));
    }
}
