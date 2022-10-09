use std::convert::TryInto;

use super::Register;
use crate::bit::Bit;
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

    pub fn ram8(&mut self, input: [Bit; 16], address: &[Bit], load: Bit) -> [Bit; 16] {
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
    use crate::bit::Bit::{I, O};

    #[test]
    fn test_ram8() {
        let a = [I, O, I, I, I, I, I, I, I, O, O, O, I, I, O, I];
        let b = [I, O, I, I, O, I, O, I, O, I, I, O, I, I, O, O];
        let c = [O, I, I, I, O, O, O, I, I, O, I, O, I, I, I, O];
        let d = [O, I, O, O, O, I, I, I, I, O, I, O, O, O, I, O];
        let e = [I, I, O, I, O, I, I, O, I, O, I, O, O, O, I, O];
        let f = [I, I, O, I, O, I, O, O, I, O, I, I, I, O, I, I];
        let g = [I, I, I, O, O, O, O, O, I, I, I, I, I, I, I, I];
        let h = [O, I, I, O, I, O, O, O, O, I, O, I, I, I, I, O];

        let input = [O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O];
        let input2 = [I, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O];

        let mut ram8 = Ram8::new();

        ram8.registers[0].register(a, I);
        ram8.registers[1].register(b, I);
        ram8.registers[2].register(c, I);
        ram8.registers[3].register(d, I);
        ram8.registers[4].register(e, I);
        ram8.registers[5].register(f, I);
        ram8.registers[6].register(g, I);
        ram8.registers[7].register(h, I);

        let out1 = ram8.ram8(input, &[O, O, O], O);
        let out2 = ram8.ram8(input, &[O, I, I], O);
        let out3 = ram8.ram8(input, &[I, O, O], I);
        let out4 = ram8.ram8(input2, &[I, O, O], O);

        assert_eq!(out1, a);
        assert_eq!(out2, d);
        assert_eq!(out3, input);
        assert_eq!(out4, input);
    }
}
