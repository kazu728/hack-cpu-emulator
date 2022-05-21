use super::{and::and, not::not, or::or};
use crate::bit::Bit;

pub fn mux(a: Bit, b: Bit, sel: Bit) -> Bit {
    let x = and(a, not(sel));
    let y = and(b, sel);

    or(x, y)
}

#[cfg(test)]
mod tests {
    use super::mux;
    use crate::bit::Bit::{I, O};

    #[test]
    fn test_mux() {
        assert_eq!(mux(O, O, O), O);
        assert_eq!(mux(O, I, O), O);
        assert_eq!(mux(I, O, O), I);
        assert_eq!(mux(I, I, O), I);
        assert_eq!(mux(O, O, O), O);
        assert_eq!(mux(O, I, I), I);
        assert_eq!(mux(I, O, I), O);
        assert_eq!(mux(I, I, I), I);
    }
}
