use crate::bit::Bit;
use crate::bit::Bit::{I, O};

pub fn nand(a: Bit, b: Bit) -> Bit {
    match (a, b) {
        (O, O) => I,
        (O, I) => I,
        (I, O) => I,
        (I, I) => O,
    }
}

#[cfg(test)]
mod tests {
    use super::{nand, I, O};

    #[test]
    fn test_nand() {
        assert_eq!(nand(O, O), I);
        assert_eq!(nand(O, I), I);
        assert_eq!(nand(I, O), I);
        assert_eq!(nand(I, I), O);
    }
}
