use crate::boolean_gate::or;

use super::half_adder::half_adder;

pub fn full_adder(a: u8, b: u8, c: u8) -> (u8, u8) {
    let (carry_ab, sum_ab) = half_adder(a, b);
    let (carry_abc, sum) = half_adder(sum_ab, c);

    let carry = or(carry_ab, carry_abc);

    (carry, sum)
}

#[cfg(test)]
mod tests {
    use super::full_adder;

    #[test]
    fn test_full_adder() {
        assert_eq!(full_adder(0, 0, 0), (0, 0));
        assert_eq!(full_adder(0, 0, 1), (0, 1));
        assert_eq!(full_adder(0, 1, 0), (0, 1));
        assert_eq!(full_adder(0, 1, 1), (1, 0));
        assert_eq!(full_adder(1, 0, 0), (0, 1));
        assert_eq!(full_adder(1, 0, 1), (1, 0));
        assert_eq!(full_adder(1, 1, 0), (1, 0));
        assert_eq!(full_adder(1, 1, 1), (1, 1));
    }
}
