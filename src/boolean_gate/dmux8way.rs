use super::{dmux, dmux4way};
use std::convert::TryInto;

pub fn dmux8way(i: u8, sel: [u8; 3]) -> (u8, u8, u8, u8, u8, u8, u8, u8) {
    let (ab, cd, ef, gh) = dmux4way(i, sel[0..2].try_into().unwrap());

    let (a, b) = dmux(ab, sel[2]);
    let (c, d) = dmux(cd, sel[2]);
    let (e, f) = dmux(ef, sel[2]);
    let (g, h) = dmux(gh, sel[2]);

    (a, b, c, d, e, f, g, h)
}

#[cfg(test)]
mod tests {
    use crate::boolean_gate::dmux8way;

    #[test]
    fn test_dmux8way() {
        assert_eq!(dmux8way(1, [0, 0, 0]), (1, 0, 0, 0, 0, 0, 0, 0));
        assert_eq!(dmux8way(1, [0, 0, 1]), (0, 1, 0, 0, 0, 0, 0, 0));
        assert_eq!(dmux8way(1, [0, 1, 0]), (0, 0, 1, 0, 0, 0, 0, 0));
        assert_eq!(dmux8way(1, [0, 1, 1]), (0, 0, 0, 1, 0, 0, 0, 0));
        assert_eq!(dmux8way(1, [1, 0, 0]), (0, 0, 0, 0, 1, 0, 0, 0));
        assert_eq!(dmux8way(1, [1, 0, 1]), (0, 0, 0, 0, 0, 1, 0, 0));
        assert_eq!(dmux8way(1, [1, 1, 0]), (0, 0, 0, 0, 0, 0, 1, 0));
        assert_eq!(dmux8way(1, [1, 1, 1]), (0, 0, 0, 0, 0, 0, 0, 1));
    }
}
