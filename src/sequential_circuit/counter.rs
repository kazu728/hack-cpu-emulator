use super::Register;
use crate::bit::Bit::{I, O};
use crate::{bit::Bit, boolean_arithmetic::inc16, boolean_gate::mux16};

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

    pub fn count(&mut self, input: [Bit; 16], inc: Bit, load: Bit, reset: Bit) -> [Bit; 16] {
        let current: [Bit; 16] = self.counter.binary_cells.map(|bc| bc.dff.current);
        let next = inc16(current);

        let inc_outout = mux16(current, next, inc);
        let load_out = mux16(inc_outout, input, load);

        let out = mux16(
            load_out,
            [O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O],
            reset,
        );

        self.counter.io(out, I);

        out
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        bit::Bit::{self, I, O},
        boolean_arithmetic::inc16,
    };

    use super::PC;

    #[test]
    fn test_counter() {
        let zero: [Bit; 16] = [O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O];
        let initial_counter: [Bit; 16] = [O, O, O, O, O, O, O, O, O, O, O, O, O, O, I, I];
        let input: [Bit; 16] = [O, O, I, I, O, I, O, O, O, I, O, O, O, O, I, I];

        let mut pc = PC::new();
        pc.counter.io(initial_counter, I);

        let output1 = pc.count(input, I, O, O);
        let output2 = pc.count(input, O, I, O);
        let output3 = pc.count(input, O, O, I);
        let output4 = pc.count(input, I, I, I);

        assert_eq!(output1, inc16(initial_counter));
        assert_eq!(output2, input);
        assert_eq!(output3, zero);
        assert_eq!(output4, zero);
    }
}
