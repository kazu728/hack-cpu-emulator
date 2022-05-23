use super::{nand, not};
use crate::bit::Bit;

pub fn and(a: Bit, b: Bit) -> Bit {
    let x = nand(a, b);
    not(x)
}

#[cfg(test)]
mod tests {
    use super::and;
    use crate::bit::Bit::{I, O};
    #[test]
    fn test_and() {
        assert_eq!(and(O, O), O);
        assert_eq!(and(O, I), O);
        assert_eq!(and(I, O), O);
        assert_eq!(and(I, I), I);
    }
}
