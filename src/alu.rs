use crate::adder::*;
use crate::bit::Bit;
use crate::bit::Bit::*;
use crate::gate::*;

pub fn alu(
    x: [Bit; 16], // input
    y: [Bit; 16], // input
    zx: Bit,      // 入力xをゼロにする
    nx: Bit,      // 入力xを反転(negate)する
    zy: Bit,      // 入力yをゼロにする
    ny: Bit,      // 入力yを反転する
    f: Bit,       // 関数コード:1 は「加算」、0 は「And 演算」に対応する
    no: Bit,      // 出力 out を反転する
) -> ([Bit; 16], Bit, Bit) {
    let out = apply_no(
        apply_f(
            apply_nx(apply_zx(x, zx), nx),
            apply_ny(apply_zy(y, zy), ny),
            f,
        ),
        no,
    );

    let zr = not(or(
        or8way([
            out[0], out[1], out[2], out[3], out[4], out[5], out[6], out[7],
        ]),
        or8way([
            out[8], out[9], out[10], out[11], out[12], out[13], out[14], out[15],
        ]),
    ));

    let ng = out[0];

    (out, zr, ng)
}

fn apply_zx(x: [Bit; 16], zx: Bit) -> [Bit; 16] {
    let y = [O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O];
    mux16(x, y, zx)
}

fn apply_nx(x: [Bit; 16], nx: Bit) -> [Bit; 16] {
    mux16(x, not16(x), nx)
}

fn apply_zy(y: [Bit; 16], zy: Bit) -> [Bit; 16] {
    let z = [O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O];
    mux16(y, z, zy)
}

fn apply_ny(y: [Bit; 16], ny: Bit) -> [Bit; 16] {
    mux16(y, not16(y), ny)
}

fn apply_f(x: [Bit; 16], y: [Bit; 16], f: Bit) -> [Bit; 16] {
    mux16(and16(x, y), add16(x, y), f)
}

fn apply_no(out: [Bit; 16], no: Bit) -> [Bit; 16] {
    mux16(out, not16(out), no)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_half_adder() {
        assert_eq!(half_adder(O, O), (O, O));
        assert_eq!(half_adder(O, I), (O, I));
        assert_eq!(half_adder(I, O), (O, I));
        assert_eq!(half_adder(I, I), (I, O));
    }

    #[test]
    fn test_full_adder() {
        assert_eq!(full_adder(O, O, O), (O, O));
        assert_eq!(full_adder(O, O, I), (O, I));
        assert_eq!(full_adder(O, I, O), (O, I));
        assert_eq!(full_adder(O, I, I), (I, O));
        assert_eq!(full_adder(I, O, O), (O, I));
        assert_eq!(full_adder(I, O, I), (I, O));
        assert_eq!(full_adder(I, I, O), (I, O));
        assert_eq!(full_adder(I, I, I), (I, I));
    }

    #[test]
    fn test_add16() {
        let a = [O, I, I, O, I, O, I, O, O, O, I, I, O, I, O, I];
        let b = [O, I, I, I, O, O, I, O, I, I, I, I, O, I, I, I];

        let output = [I, I, O, I, I, I, O, I, O, O, I, O, I, I, O, O];

        assert_eq!(add16(a, b), output);
    }

    #[test]
    fn test_inc16() {
        let a = [O, I, I, O, I, O, I, O, O, O, I, I, O, I, O, I];
        let b = [O, I, I, O, I, O, I, O, O, O, I, I, O, I, I, O];

        assert_eq!(inc16(a), b);
    }

    #[test]
    fn test_zx() {
        let x = [O, I, I, O, I, O, I, O, O, O, I, I, O, I, O, I];
        let y = [O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O];

        assert_eq!(apply_zx(x, O), x);
        assert_eq!(apply_zx(x, I), y);
    }

    #[test]
    fn test_nx() {
        let x = [O, I, I, O, I, O, I, O, O, O, I, I, O, I, O, I];
        let y = [I, O, O, I, O, I, O, I, I, I, O, O, I, O, I, O];

        assert_eq!(apply_nx(x, O), x);
        assert_eq!(apply_nx(x, I), y);
    }

    #[test]
    fn test_zy() {
        let y = [O, I, I, O, I, O, I, O, O, O, I, I, O, I, O, I];
        let z = [O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O];

        assert_eq!(apply_zy(y, O), y);
        assert_eq!(apply_zy(y, I), z);
    }

    #[test]
    fn test_ny() {
        let y = [O, I, I, O, I, O, I, O, O, O, I, I, O, I, O, I];
        let z = [I, O, O, I, O, I, O, I, I, I, O, O, I, O, I, O];

        assert_eq!(apply_nx(y, O), y);
        assert_eq!(apply_nx(y, I), z);
    }

    #[test]
    fn test_f() {
        let x = [O, I, I, O, I, O, I, O, O, O, I, I, O, I, O, I];
        let y = [I, O, O, I, O, I, O, I, I, I, O, O, I, O, I, O];

        let x_add_y = add16(x, y);
        let x_and_y = and16(x, y);

        assert_eq!(apply_f(x, y, O), x_and_y);
        assert_eq!(apply_f(x, y, I), x_add_y);
    }

    #[test]
    fn test_no() {
        let x = [O, I, I, O, I, O, I, O, O, O, I, I, O, I, O, I];
        let y = [I, O, O, I, O, I, O, I, I, I, O, O, I, O, I, O];

        assert_eq!(apply_no(x, O), x);
        assert_eq!(apply_no(x, I), y);
    }

    #[test]
    fn test_alu() {
        let x = [O, I, I, O, I, O, I, O, O, O, I, I, O, I, O, I];
        let y = [O, I, I, I, O, O, I, O, I, I, I, I, O, I, I, I];
        let minus_x = inc16(not16(x));
        let minus_y = inc16(not16(y));

        let zero = [O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O];
        let one = [O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, I];
        let minus_one = [I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I];

        assert_eq!(alu(x, y, I, O, I, O, I, O,), (zero, I, O));
        assert_eq!(alu(x, y, I, I, I, I, I, I,), (one, O, O));
        assert_eq!(alu(x, y, I, I, I, O, I, O,), (minus_one, O, I));
        assert_eq!(alu(x, y, O, O, I, I, O, O,), (x, O, O));
        assert_eq!(alu(x, y, I, I, O, O, O, O,), (y, O, O));
        assert_eq!(alu(x, y, O, O, I, I, O, I,), (not16(x), O, I));
        assert_eq!(alu(x, y, I, I, O, O, O, I,), (not16(y), O, I));
        assert_eq!(alu(x, y, O, O, I, I, I, I,), (minus_x, O, I));
        assert_eq!(alu(x, y, I, I, O, O, I, I,), (minus_y, O, I));
        assert_eq!(alu(x, y, O, I, I, I, I, I,), (inc16(x), O, O));
        assert_eq!(alu(x, y, I, I, O, I, I, I,), (inc16(y), O, O));
        assert_eq!(alu(x, y, O, O, I, I, I, O,), (add16(x, minus_one), O, O));
        assert_eq!(alu(x, y, I, I, O, O, I, O,), (add16(y, minus_one), O, O));
        assert_eq!(alu(x, y, O, O, O, O, I, O,), (add16(x, y), O, I));
        assert_eq!(alu(x, y, O, I, O, O, I, I,), (add16(x, minus_y), O, I));
        assert_eq!(alu(x, y, O, O, O, I, I, I,), (add16(y, minus_x), O, O));
        assert_eq!(alu(x, y, O, O, O, O, O, O,), (and16(x, y), O, O));
        assert_eq!(alu(x, y, O, I, O, I, O, I,), (or16(x, y), O, O));
    }
}
