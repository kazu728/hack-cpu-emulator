use super::BinaryCell;

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

    pub fn gen_bit_arr(&self) -> [u8; 16] {
        self.binary_cells.map(|bc| bc.dff.current)
    }

    pub fn register(&mut self, input: [u8; 16], load: u8) -> [u8; 16] {
        let o0 = self.binary_cells[0].binary_cell(input[0], load);
        let o1 = self.binary_cells[1].binary_cell(input[1], load);
        let o2 = self.binary_cells[2].binary_cell(input[2], load);
        let o3 = self.binary_cells[3].binary_cell(input[3], load);
        let o4 = self.binary_cells[4].binary_cell(input[4], load);
        let o5 = self.binary_cells[5].binary_cell(input[5], load);
        let o6 = self.binary_cells[6].binary_cell(input[6], load);
        let o7 = self.binary_cells[7].binary_cell(input[7], load);
        let o8 = self.binary_cells[8].binary_cell(input[8], load);
        let o9 = self.binary_cells[9].binary_cell(input[9], load);
        let o10 = self.binary_cells[10].binary_cell(input[10], load);
        let o11 = self.binary_cells[11].binary_cell(input[11], load);
        let o12 = self.binary_cells[12].binary_cell(input[12], load);
        let o13 = self.binary_cells[13].binary_cell(input[13], load);
        let o14 = self.binary_cells[14].binary_cell(input[14], load);
        let o15 = self.binary_cells[15].binary_cell(input[15], load);

        [
            o0, o1, o2, o3, o4, o5, o6, o7, o8, o9, o10, o11, o12, o13, o14, o15,
        ]
    }
}

#[cfg(test)]

mod tests {
    use crate::sequential_circuit::register::Register;

    #[test]
    fn test_register() {
        let mut register = Register::new();

        let a = [0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1];
        let b = [0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1];

        let o1 = register.register(a, 1);
        let o2 = register.register(b, 0);
        let o3 = register.register(b, 1);

        assert_eq!(o1, a);
        assert_eq!(o2, a);
        assert_eq!(o3, b);
    }
}
