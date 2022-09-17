use super::{nand, not, or};

pub fn xor(a: u8, b: u8) -> u8 {
    let nota = not(a);
    let notb = not(b);

    let x = or(nota, b);
    let y = or(a, notb);

    nand(x, y)
}

#[cfg(test)]
mod tests {
    use super::xor;

    #[test]
    fn test_xor() {
        assert_eq!(xor(0, 0), 0);
        assert_eq!(xor(0, 1), 1);
        assert_eq!(xor(1, 0), 1);
        assert_eq!(xor(1, 1), 0);
    }
}
