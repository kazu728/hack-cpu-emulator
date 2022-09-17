use super::Ram64;
use crate::boolean_gate::mux16;
use crate::util::parse_address;

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

    pub fn ram512(&mut self, input: [u8; 16], address: &[u8], load: u8) -> [u8; 16] {
        let (lower_bit, index) = parse_address(address);

        let out = self.ram64s[index].ram64(input, lower_bit, load);

        mux16(out, input, load)
    }
}

#[cfg(test)]
mod tests {
    use crate::sequential_circuit::ram512::Ram512;

    #[test]
    fn test_ram512() {
        let input = [1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0];
        let input2 = [0, 1, 1, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0];
        let input3 = [1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0];

        let mut ram512 = Ram512::new();

        let output1 = ram512.ram512(input, &[0, 0, 0, 0, 0, 0, 0, 0, 0], 1);
        let output2 = ram512.ram512(input2, &[0, 0, 0, 0, 0, 0, 0, 0, 0], 0);
        let output3 = ram512.ram512(input2, &[0, 1, 1, 0, 0, 0, 0, 0, 0], 1);
        let output4 = ram512.ram512(input3, &[0, 1, 1, 0, 0, 0, 0, 0, 0], 0);
        let output5 = ram512.ram512(input3, &[0, 1, 1, 0, 0, 0, 0, 0, 0], 1);

        assert_eq!(output1, input);
        assert_eq!(output2, input);
        assert_eq!(output3, input2);
        assert_eq!(output4, input2);
        assert_eq!(output4, input2);
        assert_eq!(output5, input3);
    }
}
