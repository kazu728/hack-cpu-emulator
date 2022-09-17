use std::convert::TryInto;

use super::Register;
use crate::boolean_gate::{mux16, mux8way16};
use crate::util::transform_from_byte_to_usize;

#[derive(Debug, Clone, Copy)]
pub struct Ram8 {
    pub registers: [Register; 8],
}

impl Ram8 {
    pub fn new() -> Self {
        Ram8 {
            registers: [Register::new(); 8],
        }
    }

    pub fn ram8(&mut self, input: [u8; 16], address: &[u8], load: u8) -> [u8; 16] {
        let words = self.registers.map(|r| r.gen_bit_arr());

        let word = mux8way16(
            words[0],
            words[1],
            words[2],
            words[3],
            words[4],
            words[5],
            words[6],
            words[7],
            address.try_into().unwrap(),
        );

        self.registers[transform_from_byte_to_usize(address)].register(input, load);

        mux16(word, input, load)
    }
}

#[cfg(test)]
mod tests {
    use super::Ram8;

    #[test]
    fn test_ram8() {
        let a = [1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 0, 1];
        let b = [1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 0];
        let c = [0, 1, 1, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0];
        let d = [0, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 0, 0, 0, 1, 0];
        let e = [1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0];
        let f = [1, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1];
        let g = [1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1];
        let h = [0, 1, 1, 0, 1, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 0];

        let input = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let input2 = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        let mut ram8 = Ram8::new();

        ram8.registers[0].register(a, 1);
        ram8.registers[1].register(b, 1);
        ram8.registers[2].register(c, 1);
        ram8.registers[3].register(d, 1);
        ram8.registers[4].register(e, 1);
        ram8.registers[5].register(f, 1);
        ram8.registers[6].register(g, 1);
        ram8.registers[7].register(h, 1);

        let out1 = ram8.ram8(input, &[0, 0, 0], 0);
        let out2 = ram8.ram8(input, &[0, 1, 1], 0);
        let out3 = ram8.ram8(input, &[1, 0, 0], 1);
        let out4 = ram8.ram8(input2, &[1, 0, 0], 0);

        assert_eq!(out1, a);
        assert_eq!(out2, d);
        assert_eq!(out3, input);
        assert_eq!(out4, input);
    }
}
