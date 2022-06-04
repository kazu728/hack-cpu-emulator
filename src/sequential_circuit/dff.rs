use crate::bit::Bit;
use crate::bit::Bit::{I, O};

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
    pub fn io(&mut self, a: Bit) -> Bit {
        self.prev = self.current;
        self.current = a;

        self.prev
    }
}

#[cfg(test)]
mod tests {
    use super::Dff;
    use crate::bit::Bit::{self, I, O};
    use std::convert::TryInto;

    #[test]
    fn test_dff() {
        let mut dff = Dff::new();

        let i = [O, I, O, I];
        let expected = [O, I, O];

        let actual: [Bit; 3] = i
            .iter()
            .map(|j| dff.io(*j))
            .enumerate()
            .filter(|&(index, _)| index != 0)
            .map(|(_, val)| val)
            .collect::<Vec<Bit>>()
            .try_into()
            .unwrap();

        assert_eq!(actual, expected);
    }
}
