#[derive(Debug, Clone, Copy)]
pub struct Dff {
    pub prev: u8,
    pub current: u8,
}

impl Dff {
    pub fn new() -> Self {
        Dff {
            prev: 0,
            current: 0,
        }
    }
    pub fn dff(&mut self, a: u8) -> u8 {
        self.prev = self.current;
        self.current = a;

        self.prev
    }
}

#[cfg(test)]
mod tests {
    use super::Dff;

    #[test]
    fn test_dff() {
        let mut dff = Dff::new();

        dff.dff(1);

        assert_eq!(dff.dff(0), 1);
        assert_eq!(dff.dff(1), 0);
    }
}
