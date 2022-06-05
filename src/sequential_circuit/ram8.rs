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

    pub fn io(&mut self, input: [Bit; 16], address: [Bit; 3], load: Bit) -> [Bit; 16] {
        let words = self
            .registers
            .map(|r| r.binary_cells.map(|bc| bc.dff.current));

        let word = mux8way16(
            words[0], words[1], words[2], words[3], words[4], words[5], words[6], words[7], address,
        );

        if load == Bit::I {
            self.registers[transform_from_byte_to_usize(address)]
                .binary_cells
                .iter_mut()
                .enumerate()
                .for_each(|(index, binary_cell)| {
                    binary_cell.dff.io(input[index]);
                });
        }

        mux16(word, input, load)
    }
}

#[cfg(test)]
mod tests {
    use crate::bit::Bit::{I, O};

    use super::Ram8;
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

        ram8.registers[0].io(a, I);
        ram8.registers[1].io(b, I);
        ram8.registers[2].io(c, I);
        ram8.registers[3].io(d, I);
        ram8.registers[4].io(e, I);
        ram8.registers[5].io(f, I);
        ram8.registers[6].io(g, I);
        ram8.registers[7].io(h, I);

        let out1 = ram8.io(input, [O, O, O], O);
        let out2 = ram8.io(input, [O, I, I], O);
        let out3 = ram8.io(input, [I, O, O], I);
        let out4 = ram8.io(input2, [I, O, O], O);

        assert_eq!(out1, a);
        assert_eq!(out2, d);
        assert_eq!(out3, input);
        assert_eq!(out4, input);
    }
}
