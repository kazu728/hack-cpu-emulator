use super::Dff;
use crate::{bit::Bit, boolean_gate::mux};

#[derive(Debug, Clone, Copy)]
pub struct BinaryCell {
    pub dff: Dff,
}

impl BinaryCell {
    pub fn new() -> Self {
        Self {
            dff: self::Dff::new(),
        }
    }

    pub fn io(&mut self, input: Bit, load: Bit) -> Bit {
        let out = mux(self.dff.current, input, load);

        if load == Bit::I {
            self.dff.io(input);
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::BinaryCell;
    use crate::bit::Bit::{I, O};

    #[test]
    fn out_current_bit() {
        let mut binaray_cell = BinaryCell::new();

        let a = binaray_cell.io(O, I);
        let b = binaray_cell.io(I, I);
        let c = binaray_cell.io(O, O);
        let d = binaray_cell.io(O, I);
        let e = binaray_cell.io(I, O);
        let f = binaray_cell.io(O, I);
        let g = binaray_cell.io(I, I);

        assert_eq!(a, O);
        assert_eq!(b, I);
        assert_eq!(c, I);
        assert_eq!(d, O);
        assert_eq!(e, O);
        assert_eq!(f, O);
        assert_eq!(g, I);
    }
}
