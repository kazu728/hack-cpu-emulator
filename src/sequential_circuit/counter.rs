use super::Register;
use crate::{boolean_arithmetic::inc16, boolean_gate::mux16};

#[derive(Debug, Clone, Copy)]
pub struct PC {
    counter: Register,
}

impl PC {
    pub fn new() -> Self {
        PC {
            counter: Register::new(),
        }
    }

    pub fn count(&mut self, input: [u8; 16], inc: u8, load: u8, reset: u8) -> [u8; 16] {
        let current: [u8; 16] = self.counter.gen_bit_arr();

        let next = inc16(current);

        let inc_outout = mux16(current, next, inc);
        let load_out = mux16(inc_outout, input, load);

        let out = mux16(
            load_out,
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            reset,
        );

        self.counter.register(out, 1);

        out
    }
}

#[cfg(test)]
mod tests {
    use crate::boolean_arithmetic::inc16;

    use super::PC;

    #[test]
    fn test_counter() {
        let zero: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let initial_counter: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1];
        let input: [u8; 16] = [0, 0, 1, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1];

        let mut pc = PC::new();
        pc.counter.register(initial_counter, 1);

        let output1 = pc.count(input, 1, 0, 0);
        let output2 = pc.count(input, 0, 1, 0);
        let output3 = pc.count(input, 0, 0, 1);
        let output4 = pc.count(input, 1, 1, 1);

        assert_eq!(output1, inc16(initial_counter));
        assert_eq!(output2, input);
        assert_eq!(output3, zero);
        assert_eq!(output4, zero);
    }
}
