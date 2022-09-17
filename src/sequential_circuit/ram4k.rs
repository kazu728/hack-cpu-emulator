use super::Ram512;
use crate::boolean_gate::mux16;
use crate::util::parse_address;

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

    pub fn ram4k(&mut self, input: [u8; 16], address: &[u8], load: u8) -> [u8; 16] {
        let (lower_bit, index) = parse_address(address);
        let out = self.ram512s[index].ram512(input, lower_bit, load);

        mux16(out, input, load)
    }
}

#[cfg(test)]
mod tests {
    use crate::sequential_circuit::Ram4k;

    #[test]
    fn test_ram4k() {
        let input = [1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0];
        let input2 = [0, 1, 1, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0];
        let input3 = [1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0];

        let mut ram4k = Ram4k::new();

        let output1 = ram4k.ram4k(input, &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 1);
        let output2 = ram4k.ram4k(input2, &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 0);
        let output3 = ram4k.ram4k(input2, &[0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1], 1);
        let output4 = ram4k.ram4k(input3, &[0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1], 0);
        let output5 = ram4k.ram4k(input3, &[0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1], 1);

        assert_eq!(output1, input);
        assert_eq!(output2, input);
        assert_eq!(output3, input2);
        assert_eq!(output4, input2);
        assert_eq!(output4, input2);
        assert_eq!(output5, input3);
    }
}
