use super::or::or;
use crate::bit::Bit;

fn or8way(a: [Bit; 8]) -> Bit {
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
    use crate::bit::Bit::{I, O};

    use super::or8way;
    #[test]
    fn test_or8way() {
        let a = [O, O, O, O, O, O, O, O];
        let b = [O, O, O, I, O, O, O, O];

        assert_eq!(or8way(a), O);
        assert_eq!(or8way(b), I)
    }
}
