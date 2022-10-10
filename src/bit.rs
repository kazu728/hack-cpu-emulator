use Bit::{I, O};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bit {
    O = 0,
    I = 1,
}

pub fn to_decimal(bits: &[Bit]) -> usize {
    let mut pow = bits.len() as u32;

    bits.iter().fold(0, |acc, x| {
        pow -= 1;

        match x {
            O => acc,
            I => acc + 2_i32.pow(pow),
        }
    }) as usize
}

#[cfg(test)]
mod tests {
    use crate::bit::Bit::{I, O};

    use super::to_decimal;

    #[test]
    fn test_to_decimal() {
        let a = &[O, O, O];
        let b = &[O, O, I];
        let c = &[O, I, O];
        let d = &[O, I, I];
        let e = &[I, O, O];
        let f = &[I, O, I];
        let g = &[I, I, O];
        let h = &[I, I, I];
        let i = &[I, I, I, I, I, I, I, I];

        assert_eq!(to_decimal(a), 0);
        assert_eq!(to_decimal(b), 1);
        assert_eq!(to_decimal(c), 2);
        assert_eq!(to_decimal(d), 3);
        assert_eq!(to_decimal(e), 4);
        assert_eq!(to_decimal(f), 5);
        assert_eq!(to_decimal(g), 6);
        assert_eq!(to_decimal(h), 7);
        assert_eq!(to_decimal(i), 255);
    }
}
