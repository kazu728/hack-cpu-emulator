use super::Ram64;
use crate::util::transform_from_byte_to_usize;
use crate::{bit::Bit, boolean_gate::mux16};
use std::convert::TryInto;

#[derive(Debug, Clone, Copy)]
pub struct Ram512 {
    ram64s: [Ram64; 8],
}

impl Ram512 {
    pub fn new() -> Self {
        Ram512 {
            ram64s: [Ram64::new(); 8],
        }
    }

    pub fn io(&mut self, input: [Bit; 16], address: [Bit; 9], load: Bit) -> [Bit; 16] {
        let high_bit: [Bit; 3] = address[0..3].try_into().unwrap();
        let lower_bit: [Bit; 6] = address[3..9].try_into().unwrap();

        let index = transform_from_byte_to_usize(high_bit);

        let out = self.ram64s[index].io(input, lower_bit, load);

        mux16(out, input, load)
    }
}

#[cfg(test)]
mod tests {
    use crate::bit::Bit::{I, O};
    use crate::sequential_circuit::ram512::Ram512;

    #[test]
    fn test_ram512() {
        let input = [I, O, I, O, O, I, O, I, O, I, O, O, I, I, O, O];
        let input2 = [O, I, I, I, O, O, O, I, I, O, I, O, I, I, I, O];
        let input3 = [I, I, O, I, O, I, I, O, I, O, I, O, O, O, I, O];

        let mut ram512 = Ram512::new();

        let output1 = ram512.io(input, [O, O, O, O, O, O, O, O, O], I);
        let output2 = ram512.io(input2, [O, O, O, O, O, O, O, O, O], O);
        let output3 = ram512.io(input2, [O, I, I, O, O, O, O, O, O], I);
        let output4 = ram512.io(input3, [O, I, I, O, O, O, O, O, O], O);
        let output5 = ram512.io(input3, [O, I, I, O, O, O, O, O, O], I);

        assert_eq!(output1, input);
        assert_eq!(output2, input);
        assert_eq!(output3, input2);
        assert_eq!(output4, input2);
        assert_eq!(output4, input2);
        assert_eq!(output5, input3);
    }
}
