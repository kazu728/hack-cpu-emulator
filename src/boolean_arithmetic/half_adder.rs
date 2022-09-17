use super::super::boolean_gate::{and, xor};

pub fn half_adder(a: u8, b: u8) -> (u8, u8) {
    let sum = xor(a, b);
    let carry = and(a, b);

    (carry, sum)
}

#[cfg(test)]
mod tests {
    use super::half_adder;

    #[test]
    fn test_half_adder() {
        assert_eq!(half_adder(0, 0), (0, 0));
        assert_eq!(half_adder(0, 1), (0, 1));
        assert_eq!(half_adder(1, 0), (0, 1));
        assert_eq!(half_adder(1, 1), (1, 0));
    }
}
