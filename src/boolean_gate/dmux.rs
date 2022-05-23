use super::{and::and, not::not};
use crate::bit::Bit;

pub fn dmux(i: Bit, sel: Bit) -> (Bit, Bit) {
    let a = and(i, not(sel));
    let b = and(i, sel);

    (a, b)
}

#[cfg(test)]
mod tests {
    use super::dmux;
    use crate::bit::Bit::{I, O};

    #[test]
    fn test_dmux() {
        assert_eq!(dmux(I, O), (I, O));
        assert_eq!(dmux(I, I), (O, I))
    }
}
