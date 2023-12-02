use crate::alu::*;
use crate::bit::Bit;
use crate::gate::{and, mux16, not, or};
use crate::ram::{Counter, Register};

struct CpuInput {
    in_m: [Bit; 16],        // M入力値(RAM[A]の値) // データメモリからの入力
    instruction: [Bit; 16], // 実行する命令 // 命令メモリからの入力
    reset: Bit,             // 現在のプログラムを再実行するか
}

pub struct CpuOutput {
    out_m: [Bit; 16],     // M出力値
    write_m: Bit,         // Mに書き込みを行うか
    address_m: [Bit; 16], // データメモリ中のMのアドレス
    pc: [Bit; 15],        // 次の命令のアドレス
}

impl CpuOutput {
    pub fn new(out_m: [Bit; 16], write_m: Bit, address_m: [Bit; 16], pc: [Bit; 15]) -> Self {
        CpuOutput {
            out_m,     // M出力値
            write_m,   // Mに書き込みを行うか
            address_m, // データメモリ中のMのアドレス
            pc,        // 次の命令のアドレス
        }
    }
}

pub struct ControlBits {
    in_a: [Bit; 16],  // input to A register
    in_d: [Bit; 16],  // input to D register
    in_pc: [Bit; 16], // input to PC register
    load_a: Bit,      // laod bit for A register
    load_d: Bit,      // load bit for D register
    jump: Bit,        // jump bit for PC register
}

pub struct CPU {
    pub a: Register,
    pub d: Register,
    pub pc: Counter,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            a: Register::new(),
            d: Register::new(),
            pc: Counter::new(),
        }
    }

    fn alu(&mut self, in_m: [Bit; 16], instruction: [Bit; 16]) -> AluOutput {
        let x = self.d.out();
        let y = mux16(in_m, self.a.out(), instruction[3]); // y = A or M

        alu(
            x,
            y,
            instruction[4], // zx
            instruction[5], // nx
            instruction[6], // zy
            instruction[7], // ny
            instruction[8], // f
            instruction[9], // no
        )
    }

    pub fn decode(&mut self, in_m: [Bit; 16], instruction: [Bit; 16]) -> ControlBits {
        let is_c_instruction = instruction[0];
        let alu_out = self.alu(in_m, instruction);

        ControlBits {
            in_a: mux16(instruction, alu_out.out, is_c_instruction),
            in_d: alu_out.out,
            in_pc: self.pc.out(),
            load_a: or(instruction[10], not(is_c_instruction)),
            load_d: and(instruction[11], is_c_instruction),
            jump: and(
                is_c_instruction,
                or(
                    or(
                        and(instruction[15], not(or(alu_out.zr, alu_out.ng))),
                        and(instruction[14], alu_out.zr),
                    ),
                    and(instruction[13], alu_out.ng),
                ),
            ),
        }
    }

    fn next(&mut self, input: CpuInput) {
        let c = self.decode(input.in_m, input.instruction);
        self.a.next(c.in_a, c.load_a);
        self.d.next(c.in_d, c.load_d);
        self.pc.next(c.in_pc, not(c.jump), c.jump, input.reset);
    }

    fn output(&mut self) -> CpuOutput {
        let pc_out = self.pc.out();
        let cpu_pc_out = [
            pc_out[0], pc_out[1], pc_out[2], pc_out[3], pc_out[4], pc_out[5], pc_out[6], pc_out[7],
            pc_out[8], pc_out[9], pc_out[10], pc_out[11], pc_out[12], pc_out[13], pc_out[14],
        ];

        CpuOutput::new(self.a.out(), self.d.out()[0], self.a.out(), cpu_pc_out)
    }

    fn clock(&mut self) {
        self.a.clcok();
        self.d.clcok();
        self.pc.clock();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bit::Bit::*;

    #[test]
    fn test_alu_adder() {
        struct TestCase {
            in_m: [Bit; 16],
            instruction: [Bit; 16],
            exepct: [Bit; 16],
        }
        let mut cpu = CPU::new();

        let case = TestCase {
            in_m: [I, O, I, I, O, I, I, I, I, I, O, O, O, O, O, O],
            instruction: [I, O, I, I, O, I, I, I, I, O, O, O, O, O, O, O],
            exepct: [I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, O],
        };

        let alu_out = cpu.alu(case.in_m, case.instruction);
        assert_eq!(alu_out.out, case.exepct);

        cpu.d
            .next([O, I, I, O, O, O, O, O, O, I, I, I, O, O, I, O], I);
        cpu.clock(); // Tock
        cpu.clock(); // Tick

        let case = TestCase {
            in_m: [I, O, I, I, O, I, I, I, I, I, O, O, O, O, O, O],
            instruction: [I, O, I, I, O, I, I, I, I, O, O, O, O, O, O, O],
            exepct: [I, O, O, I, I, I, I, I, I, O, O, O, I, I, O, O],
        };

        let alu_out = cpu.alu(case.in_m, case.instruction);
        assert_eq!(alu_out.out, case.exepct);
    }

    #[test]
    fn test_alu_and() {
        struct TestCase {
            in_m: [Bit; 16],
            instruction: [Bit; 16],
            exepct: [Bit; 16],
        }
        let mut cpu = CPU::new();

        let case = TestCase {
            in_m: [I, O, I, I, O, I, I, I, I, I, O, O, O, O, O, O],
            instruction: [I, O, I, O, I, I, O, I, I, I, O, O, O, O, O, O],
            exepct: [I, O, I, I, O, I, I, I, I, I, O, O, O, O, O, I],
        };

        let alu_out = cpu.alu(case.in_m, case.instruction);
        assert_eq!(alu_out.out, case.exepct);

        cpu.d
            .next([O, I, I, O, O, I, O, I, O, I, I, I, O, O, I, O], I);
        cpu.clock(); // Tock
        cpu.clock(); // Tick

        let case = TestCase {
            in_m: [I, O, I, O, O, O, O, O, O, O, O, O, O, O, O, O],
            instruction: [I, O, I, O, O, O, O, O, O, O, O, O, O, O, O, O],
            exepct: [O, O, I, O, O, O, O, O, O, O, O, O, O, O, O, O],
        };

        let alu_out = cpu.alu(case.in_m, case.instruction);
        assert_eq!(alu_out.out, case.exepct);
    }

    #[test]
    fn test_decode() {
        struct TestCase {
            in_m: [Bit; 16],
            instruction: [Bit; 16],
        }
        let mut cpu = CPU::new();

        let case = TestCase {
            in_m: [I, O, I, I, O, I, I, I, I, I, O, O, O, O, O, O],
            instruction: [I, O, I, O, I, I, O, I, I, I, O, O, O, O, O, O],
        };

        let c = cpu.decode(case.in_m, case.instruction);
        assert_eq!(c.in_a, [I, O, I, I, O, I, I, I, I, I, O, O, O, O, O, I]);
        assert_eq!(c.in_d, [I, O, I, I, O, I, I, I, I, I, O, O, O, O, O, I]);
        assert_eq!(c.in_pc, [O; 16]);
        assert_eq!(c.load_a, O);
        assert_eq!(c.load_d, O);
        assert_eq!(c.jump, O);

        cpu.d
            .next([O, I, I, O, O, I, O, I, O, I, I, I, O, O, I, O], I);
        cpu.clock(); // Tock
        cpu.clock(); // Tick

        let case = TestCase {
            in_m: [I, O, I, O, O, O, O, O, O, O, O, O, O, O, O, O],
            instruction: [I, O, I, O, O, O, O, O, O, O, I, I, I, I, I, I],
        };

        let c = cpu.decode(case.in_m, case.instruction);

        assert_eq!(c.in_a, [O, O, I, O, O, O, O, O, O, O, O, O, O, O, O, O]);
        assert_eq!(c.in_d, [O, O, I, O, O, O, O, O, O, O, O, O, O, O, O, O]);
        assert_eq!(c.in_pc, [O; 16]);
        assert_eq!(c.load_a, I);
        assert_eq!(c.load_d, I);
        assert_eq!(c.jump, I);
    }
}
