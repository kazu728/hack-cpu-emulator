use super::{nand, not};

pub fn and(a: u8, b: u8) -> u8 {
    let x = nand(a, b);
    not(x)
}

#[cfg(test)]
mod tests {
    use super::and;
    #[test]
    fn test_and() {
        assert_eq!(and(0, 0), 0);
        assert_eq!(and(0, 1), 0);
        assert_eq!(and(1, 0), 0);
        assert_eq!(and(1, 1), 1);
    }
}
