use super::mux::mux;
use crate::bit::Bit;

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

#[cfg(test)]
mod tests {
    use super::mux16;
    use crate::bit::Bit::{I, O};

    #[test]
    fn test_mux16() {
        let a = [I, O, I, I, I, I, I, I, I, O, O, O, I, I, O, I];
        let b = [O, I, O, O, I, O, I, O, O, O, I, I, O, O, I, O];

        assert_eq!(mux16(a, b, O), a);
        assert_eq!(mux16(a, b, I), b);
    }
}
