use std::convert::TryInto;

use crate::bit::Bit;

use super::{dmux::dmux, dmux4way::dmux4way};

fn dmux8way(i: Bit, sel: [Bit; 3]) -> (Bit, Bit, Bit, Bit, Bit, Bit, Bit, Bit) {
    let (ab, cd, ef, gh) = dmux4way(i, sel[0..2].try_into().unwrap());

    let (a, b) = dmux(ab, sel[2]);
    let (c, d) = dmux(cd, sel[2]);
    let (e, f) = dmux(ef, sel[2]);
    let (g, h) = dmux(gh, sel[2]);

    (a, b, c, d, e, f, g, h)
}

#[cfg(test)]
mod tests {
    use crate::bit::Bit::{I, O};
    use crate::boolean_gate::dmux8way::dmux8way;

    #[test]
    fn test_dmux8way() {
        assert_eq!(dmux8way(I, [O, O, O]), (I, O, O, O, O, O, O, O));
        assert_eq!(dmux8way(I, [O, O, I]), (O, I, O, O, O, O, O, O));
        assert_eq!(dmux8way(I, [O, I, O]), (O, O, I, O, O, O, O, O));
        assert_eq!(dmux8way(I, [O, I, I]), (O, O, O, I, O, O, O, O));
        assert_eq!(dmux8way(I, [I, O, O]), (O, O, O, O, I, O, O, O));
        assert_eq!(dmux8way(I, [I, O, I]), (O, O, O, O, O, I, O, O));
        assert_eq!(dmux8way(I, [I, I, O]), (O, O, O, O, O, O, I, O));
        assert_eq!(dmux8way(I, [I, I, I]), (O, O, O, O, O, O, O, I));
    }
}
