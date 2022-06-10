use super::Ram4k;
use crate::{bit::Bit, boolean_gate::mux16, util::transform_from_byte_to_usize};
use std::convert::TryInto;

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

    pub fn io(&mut self, input: [Bit; 16], address: [Bit; 14], load: Bit) -> [Bit; 16] {
        let _higher_bit: [Bit; 2] = address[0..2].try_into().unwrap();
        let lower_bit: [Bit; 12] = address[2..14].try_into().unwrap();

        let higher_bit: [Bit; 3] = [[Bit::O; 1].to_vec(), _higher_bit.to_vec()]
            .concat()
            .try_into()
            .unwrap();

        let index = transform_from_byte_to_usize(higher_bit);

        let out = self.ram4ks[index].io(input, lower_bit, load);

        mux16(out, input, load)
    }
}

#[cfg(test)]
mod tests {
    use crate::bit::Bit::{I, O};
    use crate::sequential_circuit::Ram16k;

    #[test]
    fn test_ram32k() {
        let input = [I, O, I, O, O, I, O, I, O, I, O, O, I, I, O, O];
        let input2 = [O, I, I, I, O, O, O, I, I, O, I, O, I, I, I, O];
        let input3 = [I, I, O, I, O, I, I, O, I, O, I, O, O, O, I, O];

        let mut ram16k = Ram16k::new();

        let output1 = ram16k.io(input, [O, O, O, O, O, O, O, O, O, O, O, O, O, O], I);
        let output2 = ram16k.io(input2, [O, O, O, O, O, O, O, O, O, O, O, O, O, O], O);
        let output3 = ram16k.io(input2, [O, I, I, O, O, O, O, O, O, I, O, I, O, I], I);
        let output4 = ram16k.io(input3, [O, I, I, O, O, O, O, O, O, I, O, I, O, I], O);
        let output5 = ram16k.io(input3, [O, I, I, O, O, O, O, O, O, I, O, I, O, I], I);

        assert_eq!(output1, input);
        assert_eq!(output2, input);
        assert_eq!(output3, input2);
        assert_eq!(output4, input2);
        assert_eq!(output4, input2);
        assert_eq!(output5, input3);
    }
}
