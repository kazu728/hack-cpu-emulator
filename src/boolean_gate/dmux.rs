use super::{and, not};

pub fn dmux(i: u8, sel: u8) -> (u8, u8) {
    let a = and(i, not(sel));
    let b = and(i, sel);

    (a, b)
}

#[cfg(test)]
mod tests {
    use super::dmux;

    #[test]
    fn test_dmux() {
        assert_eq!(dmux(1, 0), (1, 0));
        assert_eq!(dmux(1, 1), (0, 1))
    }
}
