use super::add16;

pub fn inc16(a: [u8; 16]) -> [u8; 16] {
    let b = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];

    add16(a, b)
}

#[cfg(test)]
mod tests {

    use super::inc16;
    #[test]
    fn test_inc16() {
        let a = [0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1];
        let b = [0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 1, 0];

        assert_eq!(inc16(a), b);
    }
}
