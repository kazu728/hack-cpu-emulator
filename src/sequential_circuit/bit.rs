use super::Dff;
use crate::boolean_gate::mux;

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

    pub fn binary_cell(&mut self, input: u8, load: u8) -> u8 {
        if load == 1 {
            self.dff.dff(input);
        }

        mux(self.dff.current, input, load)
    }
}

#[cfg(test)]
mod tests {
    use super::BinaryCell;

    #[test]
    fn out_current_bit() {
        let mut binaray_cell = BinaryCell::new();

        let a = binaray_cell.binary_cell(0, 1);
        let b = binaray_cell.binary_cell(1, 1);
        let c = binaray_cell.binary_cell(0, 0);
        let d = binaray_cell.binary_cell(0, 1);
        let e = binaray_cell.binary_cell(1, 0);
        let f = binaray_cell.binary_cell(0, 1);
        let g = binaray_cell.binary_cell(1, 1);

        assert_eq!(a, 0);
        assert_eq!(b, 1);
        assert_eq!(c, 1);
        assert_eq!(d, 0);
        assert_eq!(e, 0);
        assert_eq!(f, 0);
        assert_eq!(g, 1);
    }
}
