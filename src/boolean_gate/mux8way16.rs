use super::{mux16, mux4way16};
use std::convert::TryInto;

pub fn mux8way16(
    a: [u8; 16],
    b: [u8; 16],
    c: [u8; 16],
    d: [u8; 16],
    e: [u8; 16],
    f: [u8; 16],
    g: [u8; 16],
    h: [u8; 16],
    sel: [u8; 3],
) -> [u8; 16] {
    let o1 = mux4way16(a, b, c, d, sel[1..3].try_into().unwrap());
    let o2 = mux4way16(e, f, g, h, sel[1..3].try_into().unwrap());

    mux16(o1, o2, sel[0])
}

#[cfg(test)]
mod tests {
    use super::mux8way16;

    #[test]
    fn test_mux8way16() {
        let a = [1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 0, 1];
        let b = [1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 0];
        let c = [0, 1, 1, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0];
        let d = [0, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 0, 0, 0, 1, 0];
        let e = [1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0];
        let f = [1, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1];
        let g = [1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1];
        let h = [0, 1, 1, 0, 1, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 0];

        assert_eq!(mux8way16(a, b, c, d, e, f, g, h, [0, 0, 0]), a);
        assert_eq!(mux8way16(a, b, c, d, e, f, g, h, [0, 0, 1]), b);
        assert_eq!(mux8way16(a, b, c, d, e, f, g, h, [0, 1, 0]), c);
        assert_eq!(mux8way16(a, b, c, d, e, f, g, h, [0, 1, 1]), d);
        assert_eq!(mux8way16(a, b, c, d, e, f, g, h, [1, 0, 0]), e);
        assert_eq!(mux8way16(a, b, c, d, e, f, g, h, [1, 0, 1]), f);
        assert_eq!(mux8way16(a, b, c, d, e, f, g, h, [1, 1, 0]), g);
        assert_eq!(mux8way16(a, b, c, d, e, f, g, h, [1, 1, 1]), h);
    }
}
