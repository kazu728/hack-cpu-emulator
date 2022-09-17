use super::Ram8;
use crate::{boolean_gate::mux16, util::parse_address};

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
    pub fn ram64(&mut self, input: [u8; 16], address: &[u8], load: u8) -> [u8; 16] {
        let (lower_bit, index) = parse_address(address);

        let out = self.rams8s[index].ram8(input, lower_bit, load);

        mux16(out, input, load)
    }
}

#[cfg(test)]
mod tests {

    use super::Ram64;
    #[test]
    fn test_ram64() {
        let initial_value = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        let input = [1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0];
        let input2 = [0, 1, 1, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0];
        let input3 = [1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0];

        let ram64 = &mut Ram64::new();

        let output1 = ram64.ram64(input, &[1, 0, 1, 0, 1, 0], 1);
        let output2 = ram64.ram64(input2, &[1, 0, 1, 0, 1, 0], 0);
        let output3 = ram64.ram64(input2, &[0, 0, 0, 0, 0, 0], 0);
        let output4 = ram64.ram64(input3, &[0, 0, 0, 0, 0, 0], 1);

        assert_eq!(output1, input);
        assert_eq!(output1, output2);
        assert_eq!(output3, initial_value);
        assert_eq!(output4, input3);
    }
}
