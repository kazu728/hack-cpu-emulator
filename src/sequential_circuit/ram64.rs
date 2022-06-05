use super::Ram8;
use crate::{bit::Bit, boolean_gate::mux16, util::transform_from_byte_to_usize};
use std::convert::TryInto;

#[derive(Debug, Clone, Copy)]
pub struct Ram64 {
    rams8s: [Ram8; 8],
}

impl Ram64 {
    pub fn new() -> Self {
        Ram64 {
            rams8s: [Ram8::new(); 8],
        }
    }
    pub fn io(&mut self, input: [Bit; 16], address: [Bit; 6], load: Bit) -> [Bit; 16] {
        let higher_bit: [Bit; 3] = address[0..3].try_into().unwrap();
        let lower_bit: [Bit; 3] = address[3..6].try_into().unwrap();

        let higher_bit_index = transform_from_byte_to_usize(higher_bit);

        let out = self.rams8s[higher_bit_index].io(input, lower_bit, load);

        mux16(out, input, load)
    }
}

#[cfg(test)]
mod tests {
    use crate::bit::Bit::{I, O};

    use super::Ram64;
    #[test]
    fn test_ram64() {
        let initial_value = [O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O];

        let input = [I, O, I, O, O, I, O, I, O, I, O, O, I, I, O, O];
        let input2 = [O, I, I, I, O, O, O, I, I, O, I, O, I, I, I, O];
        let input3 = [I, I, O, I, O, I, I, O, I, O, I, O, O, O, I, O];

        let ram64 = &mut Ram64::new();

        let output1 = ram64.io(input, [I, O, I, O, I, O], I);
        let output2 = ram64.io(input2, [I, O, I, O, I, O], O);
        let output3 = ram64.io(input2, [O, O, O, O, O, O], O);
        let output4 = ram64.io(input3, [O, O, O, O, O, O], I);

        assert_eq!(output1, input);
        assert_eq!(output1, output2);
        assert_eq!(output3, initial_value);
        assert_eq!(output4, input3);
    }
}
