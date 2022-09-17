use super::{and, not, or};

pub fn mux(a: u8, b: u8, sel: u8) -> u8 {
    let x = and(a, not(sel));
    let y = and(b, sel);

    or(x, y)
}

#[cfg(test)]
mod tests {
    use super::mux;

    #[test]
    fn test_mux() {
        assert_eq!(mux(0, 0, 0), 0);
        assert_eq!(mux(0, 1, 0), 0);
        assert_eq!(mux(1, 0, 0), 1);
        assert_eq!(mux(1, 1, 0), 1);
        assert_eq!(mux(0, 0, 0), 0);
        assert_eq!(mux(0, 1, 1), 1);
        assert_eq!(mux(1, 0, 1), 0);
        assert_eq!(mux(1, 1, 1), 1);
    }
}
