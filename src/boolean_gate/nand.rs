pub fn nand(a: u8, b: u8) -> u8 {
    match (a, b) {
        (0, 0) => 1,
        (0, 1) => 1,
        (1, 0) => 1,
        (1, 1) => 0,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::nand;

    #[test]
    fn test_nand() {
        assert_eq!(nand(0, 0), 1);
        assert_eq!(nand(0, 1), 1);
        assert_eq!(nand(1, 0), 1);
        assert_eq!(nand(1, 1), 0);
    }
}
