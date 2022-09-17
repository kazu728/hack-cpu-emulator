use super::add16;
use crate::boolean_gate::{and16, mux16, not, not16, or, or8way};
use std::convert::TryInto;

pub fn alu(
    x: [u8; 16],
    y: [u8; 16],
    zx: u8,
    nx: u8,
    zy: u8,
    ny: u8,
    f: u8,
    no: u8,
) -> ([u8; 16], u8, u8) {
    let zxout = apply_zx(x, zx);
    let nxout = apply_nx(zxout, nx);

    let zyout = apply_zy(y, zy);
    let nyout = apply_ny(zyout, ny);

    let f_out = apply_f(nxout, nyout, f);

    let out = apply_no(f_out, no);

    let o1 = or8way(out[0..8].try_into().unwrap());
    let o2 = or8way(out[8..16].try_into().unwrap());

    let zr = not(or(o1, o2));

    let ng = out[0];

    (out, zr, ng)
}

fn apply_zx(x: [u8; 16], zx: u8) -> [u8; 16] {
    let y = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    mux16(x, y, zx)
}

fn apply_nx(x: [u8; 16], nx: u8) -> [u8; 16] {
    mux16(x, not16(x), nx)
}

fn apply_zy(y: [u8; 16], zy: u8) -> [u8; 16] {
    let z = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    mux16(y, z, zy)
}

fn apply_ny(y: [u8; 16], ny: u8) -> [u8; 16] {
    mux16(y, not16(y), ny)
}

fn apply_f(x: [u8; 16], y: [u8; 16], f: u8) -> [u8; 16] {
    mux16(and16(x, y), add16(x, y), f)
}

fn apply_no(out: [u8; 16], no: u8) -> [u8; 16] {
    mux16(out, not16(out), no)
}

#[cfg(test)]
mod tests {
    use super::{alu, apply_f, apply_no, apply_nx, apply_zx, apply_zy};
    use crate::{
        boolean_arithmetic::{add16, inc16},
        boolean_gate::{and16, not16, or16},
    };

    #[test]
    fn test_zx() {
        let x = [0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1];
        let y = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        assert_eq!(apply_zx(x, 0), x);
        assert_eq!(apply_zx(x, 1), y);
    }

    #[test]
    fn test_nx() {
        let x = [0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1];
        let y = [1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1, 0, 1, 0];

        assert_eq!(apply_nx(x, 0), x);
        assert_eq!(apply_nx(x, 1), y);
    }

    #[test]
    fn test_zy() {
        let y = [0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1];
        let z = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        assert_eq!(apply_zy(y, 0), y);
        assert_eq!(apply_zy(y, 1), z);
    }

    #[test]
    fn test_ny() {
        let y = [0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1];
        let z = [1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1, 0, 1, 0];

        assert_eq!(apply_nx(y, 0), y);
        assert_eq!(apply_nx(y, 1), z);
    }

    #[test]
    fn test_f() {
        let x = [0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1];
        let y = [1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1, 0, 1, 0];

        let x_add_y = add16(x, y);
        let x_and_y = and16(x, y);

        assert_eq!(apply_f(x, y, 0), x_and_y);
        assert_eq!(apply_f(x, y, 1), x_add_y);
    }

    #[test]
    fn test_no() {
        let x = [0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1];
        let y = [1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1, 0, 1, 0];

        assert_eq!(apply_no(x, 0), x);
        assert_eq!(apply_no(x, 1), y);
    }

    #[test]
    fn test_alu() {
        let x = [0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1];
        let y = [0, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1];
        let minus_x = inc16(not16(x));
        let minus_y = inc16(not16(y));

        let zero = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let one = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
        let minus_one = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];

        assert_eq!(alu(x, y, 1, 0, 1, 0, 1, 0,), (zero, 1, 0));
        assert_eq!(alu(x, y, 1, 1, 1, 1, 1, 1,), (one, 0, 0));
        assert_eq!(alu(x, y, 1, 1, 1, 0, 1, 0,), (minus_one, 0, 1));
        assert_eq!(alu(x, y, 0, 0, 1, 1, 0, 0,), (x, 0, 0));
        assert_eq!(alu(x, y, 1, 1, 0, 0, 0, 0,), (y, 0, 0));
        assert_eq!(alu(x, y, 0, 0, 1, 1, 0, 1,), (not16(x), 0, 1));
        assert_eq!(alu(x, y, 1, 1, 0, 0, 0, 1,), (not16(y), 0, 1));
        assert_eq!(alu(x, y, 0, 0, 1, 1, 1, 1,), (minus_x, 0, 1));
        assert_eq!(alu(x, y, 1, 1, 0, 0, 1, 1,), (minus_y, 0, 1));
        assert_eq!(alu(x, y, 0, 1, 1, 1, 1, 1,), (inc16(x), 0, 0));
        assert_eq!(alu(x, y, 1, 1, 0, 1, 1, 1,), (inc16(y), 0, 0));
        assert_eq!(alu(x, y, 0, 0, 1, 1, 1, 0,), (add16(x, minus_one), 0, 0));
        assert_eq!(alu(x, y, 1, 1, 0, 0, 1, 0,), (add16(y, minus_one), 0, 0));
        assert_eq!(alu(x, y, 0, 0, 0, 0, 1, 0,), (add16(x, y), 0, 1));
        assert_eq!(alu(x, y, 0, 1, 0, 0, 1, 1,), (add16(x, minus_y), 0, 1));
        assert_eq!(alu(x, y, 0, 0, 0, 1, 1, 1,), (add16(y, minus_x), 0, 0));
        assert_eq!(alu(x, y, 0, 0, 0, 0, 0, 0,), (and16(x, y), 0, 0));
        assert_eq!(alu(x, y, 0, 1, 0, 1, 0, 1,), (or16(x, y), 0, 0));
    }
}
