use crate::bit::Bit;
use crate::bit::Bit::{I, O};

use super::add16;

pub fn inc16(a: [Bit; 16]) -> [Bit; 16] {
    let b = [O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, I];

    add16(a, b)
}

#[cfg(test)]
mod tests {
    use crate::bit::Bit::{I, O};

    use super::inc16;
    #[test]
    fn test_inc16() {
        let a = [O, I, I, O, I, O, I, O, O, O, I, I, O, I, O, I];
        let b = [O, I, I, O, I, O, I, O, O, O, I, I, O, I, I, O];

        assert_eq!(inc16(a), b);
    }
}
