use super::Ram512;
use crate::util::transform_from_byte_to_usize;
use crate::{bit::Bit, boolean_gate::mux16};
use std::convert::TryInto;

#[derive(Debug, Clone, Copy)]
pub struct Ram4k {
    ram512s: [Ram512; 8],
}

impl Ram4k {
    pub fn new() -> Self {
        Ram4k {
            ram512s: [Ram512::new(); 8],
        }
    }

    pub fn io(&mut self, input: [Bit; 16], address: [Bit; 12], load: Bit) -> [Bit; 16] {
        let higher_bit: [Bit; 3] = address[0..3].try_into().unwrap();
        let lower_bit: [Bit; 9] = address[3..12].try_into().unwrap();

        let index = transform_from_byte_to_usize(higher_bit);

        let out = self.ram512s[index].io(input, lower_bit, load);

        mux16(out, input, load)
    }
}

#[cfg(test)]
mod tests {
    use crate::bit::Bit::{I, O};
    use crate::sequential_circuit::Ram4k;

    #[test]
    fn test_ram4k() {
        let input = [I, O, I, O, O, I, O, I, O, I, O, O, I, I, O, O];
        let input2 = [O, I, I, I, O, O, O, I, I, O, I, O, I, I, I, O];
        let input3 = [I, I, O, I, O, I, I, O, I, O, I, O, O, O, I, O];

        let mut ram4k = Ram4k::new();

        let output1 = ram4k.io(input, [O, O, O, O, O, O, O, O, O, O, O, O], I);
        let output2 = ram4k.io(input2, [O, O, O, O, O, O, O, O, O, O, O, O], O);
        let output3 = ram4k.io(input2, [O, I, I, O, O, O, O, O, O, I, O, I], I);
        let output4 = ram4k.io(input3, [O, I, I, O, O, O, O, O, O, I, O, I], O);
        let output5 = ram4k.io(input3, [O, I, I, O, O, O, O, O, O, I, O, I], I);

        assert_eq!(output1, input);
        assert_eq!(output2, input);
        assert_eq!(output3, input2);
        assert_eq!(output4, input2);
        assert_eq!(output4, input2);
        assert_eq!(output5, input3);
    }
}
