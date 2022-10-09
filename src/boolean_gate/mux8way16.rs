use super::{mux16, mux4way16};
use crate::bit::Bit;
use std::convert::TryInto;

pub fn mux8way16(
    a: [Bit; 16],
    b: [Bit; 16],
    c: [Bit; 16],
    d: [Bit; 16],
    e: [Bit; 16],
    f: [Bit; 16],
    g: [Bit; 16],
    h: [Bit; 16],
    sel: [Bit; 3],
) -> [Bit; 16] {
    let o1 = mux4way16(a, b, c, d, sel[1..3].try_into().unwrap());
    let o2 = mux4way16(e, f, g, h, sel[1..3].try_into().unwrap());

    mux16(o1, o2, sel[0])
}

#[cfg(test)]
mod tests {
    use super::mux8way16;
    use crate::bit::Bit::{I, O};

    #[test]
    fn test_mux8way16() {
        let a = [I, O, I, I, I, I, I, I, I, O, O, O, I, I, O, I];
        let b = [I, O, I, I, O, I, O, I, O, I, I, O, I, I, O, O];
        let c = [O, I, I, I, O, O, O, I, I, O, I, O, I, I, I, O];
        let d = [O, I, O, O, O, I, I, I, I, O, I, O, O, O, I, O];
        let e = [I, I, O, I, O, I, I, O, I, O, I, O, O, O, I, O];
        let f = [I, I, O, I, O, I, O, O, I, O, I, I, I, O, I, I];
        let g = [I, I, I, O, O, O, O, O, I, I, I, I, I, I, I, I];
        let h = [O, I, I, O, I, O, O, O, O, I, O, I, I, I, I, O];

        assert_eq!(mux8way16(a, b, c, d, e, f, g, h, [O, O, O]), a);
        assert_eq!(mux8way16(a, b, c, d, e, f, g, h, [O, O, I]), b);
        assert_eq!(mux8way16(a, b, c, d, e, f, g, h, [O, I, O]), c);
        assert_eq!(mux8way16(a, b, c, d, e, f, g, h, [O, I, I]), d);
        assert_eq!(mux8way16(a, b, c, d, e, f, g, h, [I, O, O]), e);
        assert_eq!(mux8way16(a, b, c, d, e, f, g, h, [I, O, I]), f);
        assert_eq!(mux8way16(a, b, c, d, e, f, g, h, [I, I, O]), g);
        assert_eq!(mux8way16(a, b, c, d, e, f, g, h, [I, I, I]), h);
    }
}
