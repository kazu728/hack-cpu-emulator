use super::or;
use crate::bit::Bit;

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

#[cfg(test)]
mod tests {
    use super::or16;
    use crate::bit::Bit::{I, O};

    #[test]
    fn test_or16() {
        let a = [I, O, I, I, I, I, I, I, I, O, O, O, I, I, O, I];
        let b = [O, I, O, O, I, O, I, O, O, O, I, I, O, O, I, O];
        let output = [I, I, I, I, I, I, I, I, I, O, I, I, I, I, I, I];

        assert_eq!(or16(a, b), output);
    }
}
