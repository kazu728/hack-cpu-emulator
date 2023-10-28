use Bit::{I, O};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bit {
    O = 0,
    I = 1,
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
