use crate::bit::{Bit, Bit::O};

#[derive(Debug, Clone, Copy)]
pub struct Dff {
    pub prev: Bit,
    pub current: Bit,
}

impl Dff {
    pub fn new() -> Self {
        Dff {
            prev: O,
            current: O,
        }
    }
    pub fn dff(&mut self, a: Bit) -> Bit {
        self.prev = self.current;
        self.current = a;

        self.prev
    }
}

#[cfg(test)]
mod tests {
    use super::Dff;
    use crate::bit::Bit::{I, O};

    #[test]
    fn test_dff() {
        let mut dff = Dff::new();

        dff.dff(I);

        assert_eq!(dff.dff(O), I);
        assert_eq!(dff.dff(I), O);
    }
}
