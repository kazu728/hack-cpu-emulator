use std::convert::TryInto;

use Bit::{I, O};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bit {
    O = 0,
    I = 1,
}

pub fn from_usize(n: u16) -> [Bit; 16] {
    format!("{:0>16b}", n)
        .chars()
        .map(|c| match c {
            '0' => O,
            '1' => I,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

pub fn to_address(n: u16) -> [Bit; 15] {
    let bits = from_usize(n);

    [
        bits[1], bits[2], bits[3], bits[4], bits[5], bits[6], bits[7], bits[8], bits[9], bits[10],
        bits[11], bits[12], bits[13], bits[14], bits[15],
    ]
}

pub trait BitSliceToUsize {
    fn to_usize(&self) -> usize;
}

impl BitSliceToUsize for [Bit] {
    fn to_usize(&self) -> usize {
        self.iter().fold(0, |acc, &bit| {
            acc * 2
                + match bit {
                    O => 0,
                    I => 1,
                }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::bit::{
        Bit::{I, O},
        BitSliceToUsize,
    };

    #[test]
    fn test_to_usize() {
        let a = &[O, O, O];
        let b = &[O, O, I];
        let c = &[O, I, O];
        let d = &[O, I, I];
        let e = &[I, O, O];
        let f = &[I, O, I];
        let g = &[I, I, O];
        let h = &[I, I, I];
        let i = &[I, I, I, I, I, I, I, I];

        assert_eq!(a.to_usize(), 0);
        assert_eq!(b.to_usize(), 1);
        assert_eq!(c.to_usize(), 2);
        assert_eq!(d.to_usize(), 3);
        assert_eq!(e.to_usize(), 4);
        assert_eq!(f.to_usize(), 5);
        assert_eq!(g.to_usize(), 6);
        assert_eq!(h.to_usize(), 7);
        assert_eq!(i.to_usize(), 255);
    }
}
