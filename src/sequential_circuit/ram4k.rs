use super::Ram512;
use crate::util::parse_address;
use crate::{bit::Bit, boolean_gate::mux16};

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

    pub fn ram4k(&mut self, input: [Bit; 16], address: &[Bit], load: Bit) -> [Bit; 16] {
        let (lower_bit, index) = parse_address(address);
        let out = self.ram512s[index].ram512(input, lower_bit, load);

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

        let output1 = ram4k.ram4k(input, &[O, O, O, O, O, O, O, O, O, O, O, O], I);
        let output2 = ram4k.ram4k(input2, &[O, O, O, O, O, O, O, O, O, O, O, O], O);
        let output3 = ram4k.ram4k(input2, &[O, I, I, O, O, O, O, O, O, I, O, I], I);
        let output4 = ram4k.ram4k(input3, &[O, I, I, O, O, O, O, O, O, I, O, I], O);
        let output5 = ram4k.ram4k(input3, &[O, I, I, O, O, O, O, O, O, I, O, I], I);

        assert_eq!(output1, input);
        assert_eq!(output2, input);
        assert_eq!(output3, input2);
        assert_eq!(output4, input2);
        assert_eq!(output4, input2);
        assert_eq!(output5, input3);
    }
}
