use super::dmux::dmux;
use crate::bit::Bit;

pub fn dmux4way(i: Bit, sel: [Bit; 2]) -> (Bit, Bit, Bit, Bit) {
    let (ab, cd) = dmux(i, sel[0]);

    let (a, b) = dmux(ab, sel[1]);
    let (c, d) = dmux(cd, sel[1]);

    (a, b, c, d)
}

#[cfg(test)]
mod tests {
    use super::dmux4way;
    use crate::bit::Bit::{I, O};

    #[test]
    fn test_dmux4way() {
        assert_eq!(dmux4way(I, [O, O]), (I, O, O, O));
        assert_eq!(dmux4way(I, [O, I]), (O, I, O, O));
        assert_eq!(dmux4way(I, [I, O]), (O, O, I, O));
        assert_eq!(dmux4way(I, [I, I]), (O, O, O, I));
    }
}
