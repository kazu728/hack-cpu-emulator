use super::nand;

pub fn not(a: u8) -> u8 {
    nand(a, a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not() {
        assert_eq!(not(1), 0);
        assert_eq!(not(0), 1);
    }
}
