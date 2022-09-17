use super::{nand, not};

pub fn or(a: u8, b: u8) -> u8 {
    let x = not(a);
    let y = not(b);

    nand(x, y)
}

#[cfg(test)]
mod tests {
    use super::or;

    #[test]
    fn test_or() {
        assert_eq!(or(0, 0), 0);
        assert_eq!(or(0, 1), 1);
        assert_eq!(or(1, 0), 1);
        assert_eq!(or(1, 1), 1);
    }
}
