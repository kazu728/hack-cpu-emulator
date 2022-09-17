use super::or;

pub fn or8way(a: [u8; 8]) -> u8 {
    let o1 = or(a[0], a[1]);
    let o2 = or(o1, a[2]);
    let o3 = or(o2, a[3]);
    let o4 = or(o3, a[4]);
    let o5 = or(o4, a[5]);
    let o6 = or(o5, a[6]);

    or(o6, a[7])
}

#[cfg(test)]
mod tests {
    use super::or8way;

    #[test]
    fn test_or8way() {
        let a = [0, 0, 0, 0, 0, 0, 0, 0];
        let b = [0, 0, 0, 1, 0, 0, 0, 0];

        assert_eq!(or8way(a), 0);
        assert_eq!(or8way(b), 1)
    }
}
