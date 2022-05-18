use super::nand::nand;
use crate::bit::Bit;

pub fn not(a: Bit) -> Bit {
    nand(a, a)
}

#[cfg(test)]
mod tests {
    use super::Bit::{I, O};
    use super::*;

    #[test]
    fn test_not() {
        assert_eq!(not(I), O);
        assert_eq!(not(O), I);
    }
}
