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

pub fn not(a: Bit) -> Bit {
    nand(a, a)
}

pub fn and(a: Bit, b: Bit) -> Bit {
    not(nand(a, b))
}

pub fn or(a: Bit, b: Bit) -> Bit {
    let x = not(a);
    let y = not(b);

    nand(x, y)
}

pub fn xor(a: Bit, b: Bit) -> Bit {
    let nota = not(a);
    let notb = not(b);

    let x = or(nota, b);
    let y = or(a, notb);

    nand(x, y)
}

pub fn mux(a: Bit, b: Bit, sel: Bit) -> Bit {
    let x = and(a, not(sel));
    let y = and(b, sel);

    or(x, y)
}

pub fn dmux(i: Bit, sel: Bit) -> (Bit, Bit) {
    let a = and(i, not(sel));
    let b = and(i, sel);

    (a, b)
}

pub fn not16(a: [Bit; 16]) -> [Bit; 16] {
    [
        not(a[0]),
        not(a[1]),
        not(a[2]),
        not(a[3]),
        not(a[4]),
        not(a[5]),
        not(a[6]),
        not(a[7]),
        not(a[8]),
        not(a[9]),
        not(a[10]),
        not(a[11]),
        not(a[12]),
        not(a[13]),
        not(a[14]),
        not(a[15]),
    ]
}

pub fn and16(a: [Bit; 16], b: [Bit; 16]) -> [Bit; 16] {
    [
        and(a[0], b[0]),
        and(a[1], b[1]),
        and(a[2], b[2]),
        and(a[3], b[3]),
        and(a[4], b[4]),
        and(a[5], b[5]),
        and(a[6], b[6]),
        and(a[7], b[7]),
        and(a[8], b[8]),
        and(a[9], b[9]),
        and(a[10], b[10]),
        and(a[11], b[11]),
        and(a[12], b[12]),
        and(a[13], b[13]),
        and(a[14], b[14]),
        and(a[15], b[15]),
    ]
}

pub fn or16(a: [Bit; 16], b: [Bit; 16]) -> [Bit; 16] {
    [
        or(a[0], b[0]),
        or(a[1], b[1]),
        or(a[2], b[2]),
        or(a[3], b[3]),
        or(a[4], b[4]),
        or(a[5], b[5]),
        or(a[6], b[6]),
        or(a[7], b[7]),
        or(a[8], b[8]),
        or(a[9], b[9]),
        or(a[10], b[10]),
        or(a[11], b[11]),
        or(a[12], b[12]),
        or(a[13], b[13]),
        or(a[14], b[14]),
        or(a[15], b[15]),
    ]
}

pub fn mux16(a: [Bit; 16], b: [Bit; 16], sel: Bit) -> [Bit; 16] {
    [
        mux(a[0], b[0], sel),
        mux(a[1], b[1], sel),
        mux(a[2], b[2], sel),
        mux(a[3], b[3], sel),
        mux(a[4], b[4], sel),
        mux(a[5], b[5], sel),
        mux(a[6], b[6], sel),
        mux(a[7], b[7], sel),
        mux(a[8], b[8], sel),
        mux(a[9], b[9], sel),
        mux(a[10], b[10], sel),
        mux(a[11], b[11], sel),
        mux(a[12], b[12], sel),
        mux(a[13], b[13], sel),
        mux(a[14], b[14], sel),
        mux(a[15], b[15], sel),
    ]
}

pub fn or8way(a: [Bit; 8]) -> Bit {
    let o1 = or(a[0], a[1]);
    let o2 = or(o1, a[2]);
    let o3 = or(o2, a[3]);
    let o4 = or(o3, a[4]);
    let o5 = or(o4, a[5]);
    let o6 = or(o5, a[6]);

    or(o6, a[7])
}

pub fn mux4way16(
    a: [Bit; 16],
    b: [Bit; 16],
    c: [Bit; 16],
    d: [Bit; 16],
    sel: [Bit; 2],
) -> [Bit; 16] {
    let o1 = mux16(a, b, sel[1]);
    let o2 = mux16(c, d, sel[1]);

    mux16(o1, o2, sel[0])
}

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
    let o1 = mux4way16(a, b, c, d, [sel[1], sel[2]]);
    let o2 = mux4way16(e, f, g, h, [sel[1], sel[2]]);

    mux16(o1, o2, sel[0])
}

pub fn dmux4way(i: Bit, sel: [Bit; 2]) -> (Bit, Bit, Bit, Bit) {
    let (ab, cd) = dmux(i, sel[0]);

    let (a, b) = dmux(ab, sel[1]);
    let (c, d) = dmux(cd, sel[1]);

    (a, b, c, d)
}

pub fn dmux8way(i: Bit, sel: [Bit; 3]) -> (Bit, Bit, Bit, Bit, Bit, Bit, Bit, Bit) {
    let (ab, cd, ef, gh) = dmux4way(i, [sel[0], sel[1]]);

    let (a, b) = dmux(ab, sel[2]);
    let (c, d) = dmux(cd, sel[2]);
    let (e, f) = dmux(ef, sel[2]);
    let (g, h) = dmux(gh, sel[2]);

    (a, b, c, d, e, f, g, h)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nand() {
        assert_eq!(nand(O, O), I);
        assert_eq!(nand(O, I), I);
        assert_eq!(nand(I, O), I);
        assert_eq!(nand(I, I), O);
    }

    #[test]
    fn test_not() {
        assert_eq!(not(I), O);
        assert_eq!(not(O), I);
    }

    #[test]
    fn test_and() {
        assert_eq!(and(O, O), O);
        assert_eq!(and(O, I), O);
        assert_eq!(and(I, O), O);
        assert_eq!(and(I, I), I);
    }

    #[test]
    fn test_or() {
        assert_eq!(or(O, O), O);
        assert_eq!(or(O, I), I);
        assert_eq!(or(I, O), I);
        assert_eq!(or(I, I), I);
    }

    #[test]
    fn test_xor() {
        assert_eq!(xor(O, O), O);
        assert_eq!(xor(O, I), I);
        assert_eq!(xor(I, O), I);
        assert_eq!(xor(I, I), O);
    }

    #[test]
    fn test_mux() {
        assert_eq!(mux(O, O, O), O);
        assert_eq!(mux(O, I, O), O);
        assert_eq!(mux(I, O, O), I);
        assert_eq!(mux(I, I, O), I);
        assert_eq!(mux(O, O, O), O);
        assert_eq!(mux(O, I, I), I);
        assert_eq!(mux(I, O, I), O);
        assert_eq!(mux(I, I, I), I);
    }

    #[test]
    fn test_dmux() {
        assert_eq!(dmux(I, O), (I, O));
        assert_eq!(dmux(I, I), (O, I))
    }

    #[test]
    fn test_and16() {
        let a = [I, O, I, I, I, I, I, I, I, O, O, O, I, I, O, I];
        let b = [O, I, O, O, I, O, I, O, O, I, I, I, O, O, I, O];
        let output = [O, O, O, O, I, O, I, O, O, O, O, O, O, O, O, O];

        assert_eq!(and16(a, b), output);
    }

    #[test]
    fn test_not16() {
        let a = [I, O, I, I, I, I, O, I, I, O, O, O, I, I, O, I];
        let b = [O, I, O, O, O, O, I, O, O, I, I, I, O, O, I, O];

        assert_eq!(not16(a), b);
    }
    #[test]
    fn test_or16() {
        let a = [I, O, I, I, I, I, I, I, I, O, O, O, I, I, O, I];
        let b = [O, I, O, O, I, O, I, O, O, O, I, I, O, O, I, O];
        let output = [I, I, I, I, I, I, I, I, I, O, I, I, I, I, I, I];

        assert_eq!(or16(a, b), output);
    }

    #[test]
    fn test_mux16() {
        let a = [I, O, I, I, I, I, I, I, I, O, O, O, I, I, O, I];
        let b = [O, I, O, O, I, O, I, O, O, O, I, I, O, O, I, O];

        assert_eq!(mux16(a, b, O), a);
        assert_eq!(mux16(a, b, I), b);
    }

    #[test]
    fn test_or8way() {
        let a = [O, O, O, O, O, O, O, O];
        let b = [O, O, O, I, O, O, O, O];

        assert_eq!(or8way(a), O);
        assert_eq!(or8way(b), I)
    }

    #[test]
    fn test_mux4way16() {
        let a = [I, O, I, I, I, I, I, I, I, O, O, O, I, I, O, I];
        let b = [I, O, I, I, O, I, O, I, O, I, I, O, I, I, O, O];
        let c = [O, I, I, I, O, O, O, I, I, O, I, O, I, I, I, O];
        let d = [O, I, O, O, O, I, I, I, I, O, I, O, O, O, I, O];

        assert_eq!(mux4way16(a, b, c, d, [O, O]), a);
        assert_eq!(mux4way16(a, b, c, d, [O, I]), b);
        assert_eq!(mux4way16(a, b, c, d, [I, O]), c);
        assert_eq!(mux4way16(a, b, c, d, [I, I]), d);
    }

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

    #[test]
    fn test_dmux4way() {
        assert_eq!(dmux4way(I, [O, O]), (I, O, O, O));
        assert_eq!(dmux4way(I, [O, I]), (O, I, O, O));
        assert_eq!(dmux4way(I, [I, O]), (O, O, I, O));
        assert_eq!(dmux4way(I, [I, I]), (O, O, O, I));
    }

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
