use crate::bit::Bit;

use super::{nand, not};

pub fn or(a: Bit, b: Bit) -> Bit {
    let x = not(a);
    let y = not(b);

    nand(x, y)
}

#[cfg(test)]
mod tests {
    use super::or;
    use crate::bit::Bit::{I, O};

    #[test]
    fn test_or() {
        assert_eq!(or(O, O), O);
        assert_eq!(or(O, I), I);
        assert_eq!(or(I, O), I);
        assert_eq!(or(I, I), I);
    }
}
