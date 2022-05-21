use super::not::not;
use crate::bit::Bit;

fn not16(a: [Bit; 16]) -> [Bit; 16] {
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
    use crate::bit::Bit::{I, O};
    use crate::boolean_gate::not16::not16;

    #[test]

    fn test_not16() {
        let input = [I, O, I, I, I, I, O, I, I, O, O, O, I, I, O, I];
        let output = [O, I, O, O, O, O, I, O, O, I, I, I, O, O, I, O];

        assert_eq!(not16(input), output);
    }
}
