use super::Ram4k;
use crate::{boolean_gate::mux16, util::parse_address};

#[derive(Debug, Clone, Copy)]
pub struct Ram16k {
    ram4ks: [Ram4k; 4],
}

impl Ram16k {
    pub fn new() -> Self {
        Ram16k {
            ram4ks: [Ram4k::new(); 4],
        }
    }

    pub fn ram16k(&mut self, input: [u8; 16], address: &[u8], load: u8) -> [u8; 16] {
        let _address = &[&[0], address].concat();

        let (lower_bit, index) = parse_address(_address);

        let out = self.ram4ks[index].ram4k(input, lower_bit, load);

        mux16(out, input, load)
    }
}

#[cfg(test)]
mod tests {
    use crate::sequential_circuit::Ram16k;

    #[test]
    fn test_ram32k() {
        let input = [1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0];
        let input2 = [0, 1, 1, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0];
        let input3 = [1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0];

        let mut ram16k = Ram16k::new();

        let output1 = ram16k.ram16k(input, &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 1);
        let output2 = ram16k.ram16k(input2, &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 0);
        let output3 = ram16k.ram16k(input2, &[0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1], 1);
        let output4 = ram16k.ram16k(input3, &[0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1], 0);
        let output5 = ram16k.ram16k(input3, &[0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1], 1);

        assert_eq!(output1, input);
        assert_eq!(output2, input);
        assert_eq!(output3, input2);
        assert_eq!(output4, input2);
        assert_eq!(output4, input2);
        assert_eq!(output5, input3);
    }
}
