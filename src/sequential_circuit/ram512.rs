use super::Ram64;
use crate::util::parse_address;
use crate::{bit::Bit, boolean_gate::mux16};

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

    pub fn ram512(&mut self, input: [Bit; 16], address: &[Bit], load: Bit) -> [Bit; 16] {
        let (lower_bit, index) = parse_address(address);

        let out = self.ram64s[index].ram64(input, lower_bit, load);

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

        let output1 = ram512.ram512(input, &[O, O, O, O, O, O, O, O, O], I);
        let output2 = ram512.ram512(input2, &[O, O, O, O, O, O, O, O, O], O);
        let output3 = ram512.ram512(input2, &[O, I, I, O, O, O, O, O, O], I);
        let output4 = ram512.ram512(input3, &[O, I, I, O, O, O, O, O, O], O);
        let output5 = ram512.ram512(input3, &[O, I, I, O, O, O, O, O, O], I);

        assert_eq!(output1, input);
        assert_eq!(output2, input);
        assert_eq!(output3, input2);
        assert_eq!(output4, input2);
        assert_eq!(output4, input2);
        assert_eq!(output5, input3);
    }
}
