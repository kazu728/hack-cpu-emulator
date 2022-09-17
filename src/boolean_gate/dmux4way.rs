use super::dmux;

pub fn dmux4way(i: u8, sel: [u8; 2]) -> (u8, u8, u8, u8) {
    let (ab, cd) = dmux(i, sel[0]);

    let (a, b) = dmux(ab, sel[1]);
    let (c, d) = dmux(cd, sel[1]);

    (a, b, c, d)
}

#[cfg(test)]
mod tests {
    use super::dmux4way;

    #[test]
    fn test_dmux4way() {
        assert_eq!(dmux4way(1, [0, 0]), (1, 0, 0, 0));
        assert_eq!(dmux4way(1, [0, 1]), (0, 1, 0, 0));
        assert_eq!(dmux4way(1, [1, 0]), (0, 0, 1, 0));
        assert_eq!(dmux4way(1, [1, 1]), (0, 0, 0, 1));
    }
}
