use super::mux16;
use crate::bit::Bit;

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

#[cfg(test)]
mod tests {
    use super::mux4way16;
    use crate::bit::Bit::{I, O};
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
}
