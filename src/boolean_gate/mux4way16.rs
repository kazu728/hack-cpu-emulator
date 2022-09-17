use super::mux16;

pub fn mux4way16(a: [u8; 16], b: [u8; 16], c: [u8; 16], d: [u8; 16], sel: [u8; 2]) -> [u8; 16] {
    let o1 = mux16(a, b, sel[1]);
    let o2 = mux16(c, d, sel[1]);

    mux16(o1, o2, sel[0])
}

#[cfg(test)]
mod tests {
    use super::mux4way16;
    #[test]
    fn test_mux4way16() {
        let a = [1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 0, 1];
        let b = [1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 0];
        let c = [0, 1, 1, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0];
        let d = [0, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 0, 0, 0, 1, 0];

        assert_eq!(mux4way16(a, b, c, d, [0, 0]), a);
        assert_eq!(mux4way16(a, b, c, d, [0, 1]), b);
        assert_eq!(mux4way16(a, b, c, d, [1, 0]), c);
        assert_eq!(mux4way16(a, b, c, d, [1, 1]), d);
    }
}
