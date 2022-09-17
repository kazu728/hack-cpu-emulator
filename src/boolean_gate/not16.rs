use super::not;

pub fn not16(a: [u8; 16]) -> [u8; 16] {
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

#[cfg(test)]
mod tests {
    use crate::boolean_gate::not16::not16;

    #[test]

    fn test_not16() {
        let input = [1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 0, 1];
        let output = [0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 1, 0];

        assert_eq!(not16(input), output);
    }
}
