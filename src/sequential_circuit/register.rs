use super::BinaryCell;
use crate::bit::Bit;

#[derive(Debug, Clone, Copy)]
pub struct Register {
    pub binary_cells: [BinaryCell; 16],
}

impl Register {
    pub fn new() -> Self {
        Register {
            binary_cells: [BinaryCell::new(); 16],
        }
    }

    pub fn io(&mut self, input: [Bit; 16], load: Bit) -> [Bit; 16] {
        let o0 = self.binary_cells[0].io(input[0], load);
        let o1 = self.binary_cells[1].io(input[1], load);
        let o2 = self.binary_cells[2].io(input[2], load);
        let o3 = self.binary_cells[3].io(input[3], load);
        let o4 = self.binary_cells[4].io(input[4], load);
        let o5 = self.binary_cells[5].io(input[5], load);
        let o6 = self.binary_cells[6].io(input[6], load);
        let o7 = self.binary_cells[7].io(input[7], load);
        let o8 = self.binary_cells[8].io(input[8], load);
        let o9 = self.binary_cells[9].io(input[9], load);
        let o10 = self.binary_cells[10].io(input[10], load);
        let o11 = self.binary_cells[11].io(input[11], load);
        let o12 = self.binary_cells[12].io(input[12], load);
        let o13 = self.binary_cells[13].io(input[13], load);
        let o14 = self.binary_cells[14].io(input[14], load);
        let o15 = self.binary_cells[15].io(input[15], load);

        [
            o0, o1, o2, o3, o4, o5, o6, o7, o8, o9, o10, o11, o12, o13, o14, o15,
        ]
    }
}

#[cfg(test)]

mod tests {
    use crate::bit::Bit::{I, O};
    use crate::sequential_circuit::register::Register;

    #[test]
    fn test_register() {
        let mut register = Register::new();

        let a = [O, I, I, O, I, O, I, O, O, O, I, I, O, I, O, I];
        let b = [O, I, O, O, O, O, O, I, I, O, O, I, I, O, O, I];

        let o1 = register.io(a, I);
        let o2 = register.io(b, O);
        let o3 = register.io(b, I);

        assert_eq!(o1, a);
        assert_eq!(o2, a);
        assert_eq!(o3, b);
    }
}
