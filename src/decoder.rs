use crate::bit::Bit::{I, O};

#[derive(Debug, PartialEq, Eq)]
pub enum Computation {
    Zero,            // 0
    One,             // 1
    MinusOne,        // -1
    D,               // D
    X(bool),         // A or M
    NotD,            // !D
    NotX(bool),      // !A or !M
    MinusD,          // -D
    MinusX(bool),    // -A or -M
    DPlusOne,        // D + 1
    XPlusOne(bool),  // A + 1 or M + 1
    DMinusOne,       // D - 1
    XMinusOne(bool), // A - 1 or M - 1
    DPlusX(bool),    // D + A or D + M
    DMinusX(bool),   // D - A or D - M
    XMinusD(bool),   // A - D or M - D
    DAndX(bool),     // D & A or D & M
    DOrX(bool),      // D | A or D | M
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Jump {
    Null, // No jump
    JGT,  // if out >  0 jump
    JEQ,  // if out == 0 jump
    JGE,  // if out >= 0 jump
    JLT,  // if out <  0 jump
    JNE,  // if out != 0 jump
    JLE,  // if out <= 0 jump
    JMP,  // jump
}

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    AInstruction([Bit; 15]),
    CInstruction(Computation, u8, Jump),
}

use Computation::*;
use Instruction::*;

fn extract_most_significant_bit(instruction: u16) -> Bit {
    if instruction >> 15 & 1 == 1 {
        I
    } else {
        O
    }
}

use crate::bit::{to_address, Bit};

impl Instruction {
    pub fn decode(instruction: u16) -> Self {
        dbg!(instruction);
        match extract_most_significant_bit(instruction) {
            O => AInstruction(to_address(instruction)),
            I => {
                let a = instruction >> 12 & 1 == 1;
                let comp = match (instruction >> 6) & 0b111111 {
                    0b101010 => Zero,
                    0b111111 => One,
                    0b111010 => MinusOne,
                    0b001100 => D,
                    0b110000 => X(a),
                    0b001101 => NotD,
                    0b110001 => NotX(a),
                    0b001111 => MinusD,
                    0b110011 => MinusX(a),
                    0b011111 => DPlusOne,
                    0b110111 => XPlusOne(a),
                    0b001110 => DMinusOne,
                    0b110010 => XMinusOne(a),
                    0b000010 => DPlusX(a),
                    0b010011 => DMinusX(a),
                    0b000111 => XMinusD(a),
                    0b000000 => DAndX(a),
                    0b010101 => DOrX(a),
                    _ => unreachable!(),
                };
                let jump = match instruction & 0b111 {
                    0b000 => Jump::Null,
                    0b001 => Jump::JGT,
                    0b010 => Jump::JEQ,
                    0b011 => Jump::JGE,
                    0b100 => Jump::JLT,
                    0b101 => Jump::JNE,
                    0b110 => Jump::JLE,
                    0b111 => Jump::JMP,
                    _ => unreachable!(),
                };
                let dest = (instruction >> 3) & 0b111;
                CInstruction(comp, dest as u8, jump)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: u16,
        expected: Instruction,
    }
    impl TestCase {
        fn new(input: u16, expected: Instruction) -> Self {
            Self { input, expected }
        }
    }

    #[test]

    fn test_decode() {
        let case = [
            TestCase::new(
                0b0011000000111001,
                AInstruction([O, I, I, O, O, O, O, O, O, I, I, I, O, O, I]),
            ),
            TestCase::new(
                0b1110110000010000,
                CInstruction(Computation::X(false), 0b010, Jump::Null),
            ),
            TestCase::new(
                0b0101101110100000,
                AInstruction([I, O, I, I, O, I, I, I, O, I, O, O, O, O, O]),
            ),
            TestCase::new(
                0b1110000111010000,
                CInstruction(Computation::XMinusD(false), 0b010, Jump::Null),
            ),
            TestCase::new(
                0b1110001100000100,
                CInstruction(Computation::D, 0b000, Jump::JLT),
            ),
        ];

        for c in case.iter() {
            assert_eq!(c.expected, Instruction::decode(c.input));
        }
    }
}
