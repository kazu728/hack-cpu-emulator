use crate::bit::Bit;

use super::{nand::nand, not::not, or::or};

pub fn xor(a: Bit, b: Bit) -> Bit {
    let nota = not(a);
    let notb = not(b);

    let x = or(nota, b);
    let y = or(a, notb);

    nand(x, y)
}

#[cfg(test)]
mod tests {
    use super::xor;
    use crate::bit::Bit::{I, O};

    #[test]
    fn test_xor() {
        assert_eq!(xor(O, O), O);
        assert_eq!(xor(O, I), I);
        assert_eq!(xor(I, O), I);
        assert_eq!(xor(I, I), O);
    }
}
