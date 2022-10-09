use super::and;
use crate::bit::Bit;

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

#[cfg(test)]
mod tests {
    use super::and16;
    use crate::bit::Bit::{I, O};

    #[test]
    fn test_and16() {
        let a = [I, O, I, I, I, I, I, I, I, O, O, O, I, I, O, I];
        let b = [O, I, O, O, I, O, I, O, O, I, I, I, O, O, I, O];
        let output = [O, O, O, O, I, O, I, O, O, O, O, O, O, O, O, O];

        assert_eq!(and16(a, b), output);
    }
}
