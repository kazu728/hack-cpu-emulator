use crate::bit::{Bit, Bit::*, BitSliceToUsize};
use crate::gate::{mux, mux16, mux4way16, mux8way16};

use super::clock::Clock;
use super::clock::ClockKind::*;

#[derive(Debug, Clone, Copy)]
pub struct Dff {
    pub prev: Bit,
    pub current: Bit,
    pub clock: Clock,
}

impl Dff {
    pub fn new() -> Self {
        Dff {
            prev: O,
            current: O,
            clock: Clock::new(),
        }
    }

    pub fn next(&mut self, a: Bit) {
        self.prev = self.current;
        self.current = a;
    }

    pub fn out(&mut self) -> Bit {
        match self.clock.state {
            Tick => self.current,
            Tock => self.prev,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BitRegister {
    pub dff: Dff,
}

impl BitRegister {
    pub fn new() -> Self {
        Self {
            dff: self::Dff::new(),
        }
    }

    pub fn next(&mut self, input: Bit, load: Bit) {
        let next_bit = match load {
            O => self.dff.current,
            I => input,
        };

        self.dff.next(mux(next_bit, input, load))
    }

    pub fn out(&mut self) -> Bit {
        self.dff.out()
    }

    pub fn next_tick(&mut self) {
        self.dff.clock.next();
        self.dff.clock.next();
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Register {
    pub bit_registers: [BitRegister; 16],
}

impl Register {
    pub fn new() -> Self {
        Self {
            bit_registers: [BitRegister::new(); 16],
        }
    }

    pub fn next(&mut self, input: [Bit; 16], load: Bit) {
        self.bit_registers
            .iter_mut()
            .enumerate()
            .for_each(|(i, bit_register)| bit_register.next(input[i], load));
    }

    pub fn out(&mut self) -> [Bit; 16] {
        [
            self.bit_registers[0].out(),
            self.bit_registers[1].out(),
            self.bit_registers[2].out(),
            self.bit_registers[3].out(),
            self.bit_registers[4].out(),
            self.bit_registers[5].out(),
            self.bit_registers[6].out(),
            self.bit_registers[7].out(),
            self.bit_registers[8].out(),
            self.bit_registers[9].out(),
            self.bit_registers[10].out(),
            self.bit_registers[11].out(),
            self.bit_registers[12].out(),
            self.bit_registers[13].out(),
            self.bit_registers[14].out(),
            self.bit_registers[15].out(),
        ]
    }

    pub fn clcok(&mut self) {
        self.bit_registers.iter_mut().for_each(|bit_register| {
            bit_register.dff.clock.next();
        });
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ram8 {
    pub registers: [Register; 8],
}

impl Ram8 {
    pub fn new() -> Self {
        Ram8 {
            registers: [Register::new(); 8],
        }
    }

    pub fn next(&mut self, input: [Bit; 16], address: [Bit; 3], load: Bit) {
        let words = self.registers.map(|mut r| r.out());

        let current_register = mux8way16(
            words[0], words[1], words[2], words[3], words[4], words[5], words[6], words[7], address,
        );

        self.registers[address.to_usize()].next(mux16(current_register, input, load), load);
    }

    pub fn out(&mut self, address: [Bit; 3]) -> [Bit; 16] {
        self.registers[address.to_usize()].out()
    }

    pub fn clock(&mut self) {
        self.registers.iter_mut().for_each(|register| {
            register.clcok();
        });
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ram64 {
    pub ram8s: [Ram8; 8],
}

impl Ram64 {
    pub fn new() -> Self {
        Ram64 {
            ram8s: [Ram8::new(); 8],
        }
    }

    pub fn next(&mut self, input: [Bit; 16], address: [Bit; 6], load: Bit) {
        let lower_bit = [address[0], address[1], address[2]];
        let upper_bit = [address[3], address[4], address[5]];

        let current_ram8 = mux8way16(
            self.ram8s[0].out(lower_bit),
            self.ram8s[1].out(lower_bit),
            self.ram8s[2].out(lower_bit),
            self.ram8s[3].out(lower_bit),
            self.ram8s[4].out(lower_bit),
            self.ram8s[5].out(lower_bit),
            self.ram8s[6].out(lower_bit),
            self.ram8s[7].out(lower_bit),
            upper_bit,
        );

        self.ram8s[upper_bit.to_usize()].next(mux16(current_ram8, input, load), lower_bit, load);
    }

    pub fn out(&mut self, address: [Bit; 6]) -> [Bit; 16] {
        let lower_bit = [address[0], address[1], address[2]];
        let upper_bit = [address[3], address[4], address[5]];

        self.ram8s[upper_bit.to_usize()].out(lower_bit)
    }

    pub fn clock(&mut self) {
        self.ram8s.iter_mut().for_each(|ram8| {
            ram8.clock();
        });
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ram512 {
    pub ram64s: [Ram64; 8],
}

impl Ram512 {
    pub fn new() -> Self {
        Ram512 {
            ram64s: [Ram64::new(); 8],
        }
    }

    pub fn next(&mut self, input: [Bit; 16], address: [Bit; 9], load: Bit) {
        let lower_bit = [
            address[0], address[1], address[2], address[3], address[4], address[5],
        ];
        let upper_bit = [address[6], address[7], address[8]];

        let current_ram64 = mux8way16(
            self.ram64s[0].out(lower_bit),
            self.ram64s[1].out(lower_bit),
            self.ram64s[2].out(lower_bit),
            self.ram64s[3].out(lower_bit),
            self.ram64s[4].out(lower_bit),
            self.ram64s[5].out(lower_bit),
            self.ram64s[6].out(lower_bit),
            self.ram64s[7].out(lower_bit),
            upper_bit,
        );

        self.ram64s[upper_bit.to_usize()].next(mux16(current_ram64, input, load), lower_bit, load);
    }

    pub fn out(&mut self, address: [Bit; 9]) -> [Bit; 16] {
        let lower_bit = [
            address[0], address[1], address[2], address[3], address[4], address[5],
        ];
        let upper_bit = [address[6], address[7], address[8]];

        self.ram64s[upper_bit.to_usize()].out(lower_bit)
    }

    pub fn clock(&mut self) {
        self.ram64s.iter_mut().for_each(|ram64| {
            ram64.clock();
        });
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ram4K {
    pub ram512s: [Ram512; 8],
}

impl Ram4K {
    pub fn new() -> Self {
        Ram4K {
            ram512s: [Ram512::new(); 8],
        }
    }

    pub fn next(&mut self, input: [Bit; 16], address: [Bit; 12], load: Bit) {
        let lower_bit = [
            address[0], address[1], address[2], address[3], address[4], address[5], address[6],
            address[7], address[8],
        ];
        let upper_bit = [address[9], address[10], address[11]];

        let current_ram512 = mux8way16(
            self.ram512s[0].out(lower_bit),
            self.ram512s[1].out(lower_bit),
            self.ram512s[2].out(lower_bit),
            self.ram512s[3].out(lower_bit),
            self.ram512s[4].out(lower_bit),
            self.ram512s[5].out(lower_bit),
            self.ram512s[6].out(lower_bit),
            self.ram512s[7].out(lower_bit),
            upper_bit,
        );

        self.ram512s[upper_bit.to_usize()].next(
            mux16(current_ram512, input, load),
            lower_bit,
            load,
        );
    }

    pub fn out(&mut self, address: [Bit; 12]) -> [Bit; 16] {
        let lower_bit = [
            address[0], address[1], address[2], address[3], address[4], address[5], address[6],
            address[7], address[8],
        ];
        let upper_bit = [address[9], address[10], address[11]];

        self.ram512s[upper_bit.to_usize()].out(lower_bit)
    }

    pub fn clock(&mut self) {
        self.ram512s.iter_mut().for_each(|ram512| {
            ram512.clock();
        });
    }
}

pub struct Ram16K {
    pub ram4ks: [Ram4K; 4],
}

impl Ram16K {
    pub fn new() -> Self {
        Ram16K {
            ram4ks: [Ram4K::new(); 4],
        }
    }

    pub fn next(&mut self, input: [Bit; 16], address: [Bit; 14], load: Bit) {
        let lower_bit = [
            address[0],
            address[1],
            address[2],
            address[3],
            address[4],
            address[5],
            address[6],
            address[7],
            address[8],
            address[9],
            address[10],
            address[11],
        ];

        let upper_bit = [address[12], address[13]];

        let current_ram4k = mux4way16(
            self.ram4ks[0].out(lower_bit),
            self.ram4ks[1].out(lower_bit),
            self.ram4ks[2].out(lower_bit),
            self.ram4ks[3].out(lower_bit),
            upper_bit,
        );

        self.ram4ks[upper_bit.to_usize()].next(mux16(current_ram4k, input, load), lower_bit, load);
    }

    pub fn out(&mut self, address: [Bit; 14]) -> [Bit; 16] {
        let lower_bit = [
            address[0],
            address[1],
            address[2],
            address[3],
            address[4],
            address[5],
            address[6],
            address[7],
            address[8],
            address[9],
            address[10],
            address[11],
        ];
        let upper_bit = [address[12], address[13]];

        self.ram4ks[upper_bit.to_usize()].out(lower_bit)
    }

    pub fn clock(&mut self) {
        self.ram4ks.iter_mut().for_each(|ram4k| {
            ram4k.clock();
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dff() {
        let mut dff = Dff::new();

        dff.clock.next(); // Tock
        dff.next(I);

        assert_eq!(dff.out(), O);
        dff.clock.next(); // Tick
        assert_eq!(dff.out(), I);
    }

    #[test]
    fn test_bit_register() {
        let mut bit_register = BitRegister::new();
        bit_register.dff.clock.next(); // Tock

        assert_eq!(bit_register.out(), O);

        bit_register.next(I, I);
        assert_eq!(bit_register.out(), O);

        bit_register.dff.clock.next(); // Tick
        assert_eq!(bit_register.out(), I);

        bit_register.dff.clock.next(); // Tock

        bit_register.next(O, I);
        assert_eq!(bit_register.out(), I);

        bit_register.dff.clock.next(); // Tick
        assert_eq!(bit_register.out(), O);
    }

    #[test]
    fn test_register() {
        let mut register = Register::new();

        let a = [O, I, I, O, I, O, I, O, O, O, I, I, O, I, O, I];
        let b = [O, I, O, O, O, O, O, I, I, O, O, I, I, O, O, I];

        register.clcok(); // Tock

        register.next(a, I);
        assert_eq!(register.out(), [O; 16]);

        register.clcok(); // Tick
        assert_eq!(register.out(), a);

        register.clcok(); // Tock

        register.next(b, O);
        register.clcok(); // Tick

        assert_eq!(register.out(), a);

        register.clcok(); // Tock

        register.next(b, I);
        register.clcok(); // Tick

        assert_eq!(register.out(), b);
    }

    #[test]
    fn test_ram8() {
        let a = [I, O, I, I, I, I, I, I, I, O, O, O, I, I, O, I];
        let b = [I, O, I, I, O, I, O, I, O, I, I, O, I, I, O, O];
        let c = [O, I, I, I, O, O, O, I, I, O, I, O, I, I, I, O];
        let d = [O, I, O, O, O, I, I, I, I, O, I, O, O, O, I, O];
        let e = [I, I, O, I, O, I, I, O, I, O, I, O, O, O, I, O];
        let f = [I, I, O, I, O, I, O, O, I, O, I, I, I, O, I, I];
        let g = [I, I, I, O, O, O, O, O, I, I, I, I, I, I, I, I];
        let h = [O, I, I, O, I, O, O, O, O, I, O, I, I, I, I, O];
        let i = [O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O];

        let mut ram8 = Ram8::new();

        ram8.clock(); // Tock

        ram8.next(a, [O, O, O], I);
        ram8.next(b, [O, O, I], I);
        ram8.next(c, [O, I, O], I);
        ram8.next(d, [O, I, I], I);
        ram8.next(e, [I, O, O], I);
        ram8.next(f, [I, O, I], I);
        ram8.next(g, [I, I, O], I);
        ram8.next(h, [I, I, I], I);

        ram8.clock(); // Tick

        assert_eq!(ram8.out([O, O, O]), a);
        assert_eq!(ram8.out([O, O, I]), b);
        assert_eq!(ram8.out([O, I, O]), c);
        assert_eq!(ram8.out([O, I, I]), d);
        assert_eq!(ram8.out([I, O, O]), e);
        assert_eq!(ram8.out([I, O, I]), f);
        assert_eq!(ram8.out([I, I, O]), g);
        assert_eq!(ram8.out([I, I, I]), h);

        ram8.clock(); // Tock

        ram8.next(i, [O, O, O], O);
        ram8.clock(); // Tick
        assert_eq!(ram8.out([O, O, O]), a);

        ram8.clock(); // Tock

        ram8.next(i, [O, O, O], I);
        ram8.clock(); // Tick
        assert_eq!(ram8.out([O, O, O]), i);
    }

    #[test]
    fn test_ram64() {
        let a = [I, O, I, I, I, I, I, I, I, O, O, O, I, I, O, I];
        let b = [I, O, I, I, O, I, O, I, O, I, I, O, I, I, O, O];
        let c = [O, I, I, I, O, O, O, I, I, O, I, O, I, I, I, O];
        let d = [O, I, O, O, O, I, I, I, I, O, I, O, O, O, I, O];
        let e = [I, I, O, I, O, I, I, O, I, O, I, O, O, O, I, O];
        let f = [I, I, O, I, O, I, O, O, I, O, I, I, I, O, I, I];
        let g = [I, I, I, O, O, O, O, O, I, I, I, I, I, I, I, I];
        let h = [O, I, I, O, I, O, O, O, O, I, O, I, I, I, I, O];

        let mut ram64 = Ram64::new();

        ram64.clock(); // Tock

        ram64.next(a, [I, O, O, O, O, O], I);
        ram64.next(b, [I, O, O, O, O, I], I);
        ram64.next(c, [I, O, O, O, I, O], I);
        ram64.next(d, [I, O, O, O, I, I], I);
        ram64.next(e, [I, O, O, I, O, O], I);
        ram64.next(f, [I, O, O, I, O, I], I);
        ram64.next(g, [I, O, I, O, I, O], I);
        ram64.next(h, [I, O, I, O, I, I], I);

        ram64.clock(); // Tick

        assert_eq!(ram64.out([I, O, O, O, O, O]), a);
        assert_eq!(ram64.out([I, O, O, O, O, I]), b);
        assert_eq!(ram64.out([I, O, O, O, I, O]), c);
        assert_eq!(ram64.out([I, O, O, O, I, I]), d);
        assert_eq!(ram64.out([I, O, O, I, O, O]), e);
        assert_eq!(ram64.out([I, O, O, I, O, I]), f);
        assert_eq!(ram64.out([I, O, I, O, I, O]), g);
        assert_eq!(ram64.out([I, O, I, O, I, I]), h);

        ram64.clock(); // Tock

        ram64.next(a, [I, O, O, O, O, O], O);
        ram64.clock(); // Tick
        assert_eq!(ram64.out([I, O, O, O, O, O]), a);

        ram64.clock(); // Tock

        ram64.next(a, [I, O, O, O, O, O], I);
        ram64.clock(); // Tick
        assert_eq!(ram64.out([I, O, O, O, O, O]), a);
    }

    #[test]
    fn test_ram512() {
        let a = [I, O, I, I, I, I, I, I, I, O, O, O, I, I, O, I];
        let b = [I, O, I, I, O, I, O, I, O, I, I, O, I, I, O, O];
        let c = [O, I, I, I, O, O, O, I, I, O, I, O, I, I, I, O];
        let d = [O, I, O, O, O, I, I, I, I, O, I, O, O, O, I, O];
        let e = [I, I, O, I, O, I, I, O, I, O, I, O, O, O, I, O];
        let f = [I, I, O, I, O, I, O, O, I, O, I, I, I, O, I, I];
        let g = [I, I, I, O, O, O, O, O, I, I, I, I, I, I, I, I];
        let h = [O, I, I, O, I, O, O, O, O, I, O, I, I, I, I, O];

        let mut ram512 = Ram512::new();

        ram512.clock(); // Tock

        ram512.next(a, [O, O, I, O, O, O, O, O, O], I);
        ram512.next(b, [O, I, O, O, O, O, O, O, I], I);
        ram512.next(c, [I, O, O, O, O, O, O, I, O], I);
        ram512.next(d, [O, O, O, O, O, O, O, I, I], I);
        ram512.next(e, [O, O, O, O, I, O, I, O, O], I);
        ram512.next(f, [O, O, O, O, O, O, I, O, I], I);
        ram512.next(g, [O, O, I, O, O, O, I, I, O], I);
        ram512.next(h, [O, O, O, O, O, O, I, I, I], I);

        ram512.clock(); // Tick

        assert_eq!(ram512.out([O, O, I, O, O, O, O, O, O]), a);
        assert_eq!(ram512.out([O, I, O, O, O, O, O, O, I]), b);
        assert_eq!(ram512.out([I, O, O, O, O, O, O, I, O]), c);
        assert_eq!(ram512.out([O, O, O, O, O, O, O, I, I]), d);
        assert_eq!(ram512.out([O, O, O, O, I, O, I, O, O]), e);
        assert_eq!(ram512.out([O, O, O, O, O, O, I, O, I]), f);
        assert_eq!(ram512.out([O, O, I, O, O, O, I, I, O]), g);
        assert_eq!(ram512.out([O, O, O, O, O, O, I, I, I]), h);

        ram512.clock(); // Tock

        ram512.next(b, [O, O, I, O, O, O, O, O, O], O);
        ram512.clock(); // Tick
        assert_eq!(ram512.out([O, O, I, O, O, O, O, O, O]), a);

        ram512.clock(); // Tock

        ram512.next(b, [O, O, I, O, O, O, O, O, O], I);
        ram512.clock(); // Tick

        assert_eq!(ram512.out([O, O, I, O, O, O, O, O, O]), b);
    }

    #[test]
    fn test_ram4k() {
        let a = [I, O, I, I, I, I, I, I, I, O, O, O, I, I, O, I];
        let b = [I, O, I, I, O, I, O, I, O, I, I, O, I, I, O, O];
        let c = [O, I, I, I, O, O, O, I, I, O, I, O, I, I, I, O];
        let d = [O, I, O, O, O, I, I, I, I, O, I, O, O, O, I, O];
        let e = [I, I, O, I, O, I, I, O, I, O, I, O, O, O, I, O];
        let f = [I, I, O, I, O, I, O, O, I, O, I, I, I, O, I, I];
        let g = [I, I, I, O, O, O, O, O, I, I, I, I, I, I, I, I];
        let h = [O, I, I, O, I, O, O, O, O, I, O, I, I, I, I, O];

        let mut ram4k = Ram4K::new();

        ram4k.clock(); // Tock

        ram4k.next(a, [O, I, O, O, O, O, O, O, O, O, O, O], I);
        ram4k.next(b, [O, O, O, I, O, I, O, I, O, O, O, I], I);
        ram4k.next(c, [I, O, O, O, O, O, O, O, O, O, I, O], I);
        ram4k.next(d, [O, O, O, O, I, O, O, I, O, O, I, I], I);
        ram4k.next(e, [O, O, I, O, O, O, I, O, O, I, O, O], I);
        ram4k.next(f, [I, I, O, O, O, O, O, O, O, I, O, I], I);
        ram4k.next(g, [O, I, O, O, O, I, I, I, O, I, I, O], I);
        ram4k.next(h, [O, O, I, I, I, O, O, O, O, I, I, I], I);

        ram4k.clock(); // Tick

        assert_eq!(ram4k.out([O, I, O, O, O, O, O, O, O, O, O, O]), a);
        assert_eq!(ram4k.out([O, O, O, I, O, I, O, I, O, O, O, I]), b);
        assert_eq!(ram4k.out([I, O, O, O, O, O, O, O, O, O, I, O]), c);
        assert_eq!(ram4k.out([O, O, O, O, I, O, O, I, O, O, I, I]), d);
        assert_eq!(ram4k.out([O, O, I, O, O, O, I, O, O, I, O, O]), e);
        assert_eq!(ram4k.out([I, I, O, O, O, O, O, O, O, I, O, I]), f);
        assert_eq!(ram4k.out([O, I, O, O, O, I, I, I, O, I, I, O]), g);
        assert_eq!(ram4k.out([O, O, I, I, I, O, O, O, O, I, I, I]), h);

        ram4k.clock(); // Tock

        ram4k.next(b, [O, I, O, O, O, O, O, O, O, O, O, O], O);
        ram4k.clock(); // Tick
        assert_eq!(ram4k.out([O, I, O, O, O, O, O, O, O, O, O, O]), a);

        ram4k.clock(); // Tock

        ram4k.next(b, [O, I, O, O, O, O, O, O, O, O, O, O], I);
        ram4k.clock(); // Tick

        assert_eq!(ram4k.out([O, I, O, O, O, O, O, O, O, O, O, O]), b);
    }

    #[test]
    fn test_ram16k() {
        let a = [I, O, I, I, I, I, I, I, I, O, O, O, I, I, O, I];
        let b = [I, O, I, I, O, I, O, I, O, I, I, O, I, I, O, O];
        let c = [O, I, I, I, O, O, O, I, I, O, I, O, I, I, I, O];
        let d = [O, I, O, O, O, I, I, I, I, O, I, O, O, O, I, O];
        let e = [I, I, O, I, O, I, I, O, I, O, I, O, O, O, I, O];
        let f = [I, I, O, I, O, I, O, O, I, O, I, I, I, O, I, I];
        let g = [I, I, I, O, O, O, O, O, I, I, I, I, I, I, I, I];
        let h = [O, I, I, O, I, O, O, O, O, I, O, I, I, I, I, O];

        let mut ram16k = Ram16K::new();

        ram16k.clock(); // Tock

        ram16k.next(a, [O, I, I, O, I, O, O, I, O, O, I, I, O, O], I);
        ram16k.next(b, [I, O, O, I, I, O, O, O, I, O, I, O, O, I], I);
        ram16k.next(c, [O, I, I, O, O, I, O, I, O, O, O, O, I, O], I);
        ram16k.next(d, [I, I, O, O, O, O, O, O, O, O, O, O, I, I], I);
        ram16k.next(e, [I, O, O, O, O, I, I, I, O, O, O, I, O, O], I);
        ram16k.next(f, [O, I, I, I, O, O, O, O, O, O, O, I, O, I], I);
        ram16k.next(g, [O, I, O, O, I, I, O, O, O, I, O, I, I, O], I);
        ram16k.next(h, [I, O, I, O, O, O, I, O, I, O, O, I, I, I], I);

        ram16k.clock(); // Tick

        assert_eq!(ram16k.out([O, I, I, O, I, O, O, I, O, O, I, I, O, O]), a);
        assert_eq!(ram16k.out([I, O, O, I, I, O, O, O, I, O, I, O, O, I]), b);
        assert_eq!(ram16k.out([O, I, I, O, O, I, O, I, O, O, O, O, I, O]), c);
        assert_eq!(ram16k.out([I, I, O, O, O, O, O, O, O, O, O, O, I, I]), d);
        assert_eq!(ram16k.out([I, O, O, O, O, I, I, I, O, O, O, I, O, O]), e);
        assert_eq!(ram16k.out([O, I, I, I, O, O, O, O, O, O, O, I, O, I]), f);
        assert_eq!(ram16k.out([O, I, O, O, I, I, O, O, O, I, O, I, I, O]), g);
        assert_eq!(ram16k.out([I, O, I, O, O, O, I, O, I, O, O, I, I, I]), h);

        ram16k.clock(); // Tock

        ram16k.next(b, [O, I, I, O, I, O, O, I, O, O, I, I, O, O], O);
        ram16k.clock(); // Tick

        assert_eq!(ram16k.out([O, I, I, O, I, O, O, I, O, O, I, I, O, O]), a);

        ram16k.clock(); // Tock

        ram16k.next(b, [O, I, I, O, I, O, O, I, O, O, I, I, O, O], I);
        ram16k.clock(); // Tick

        assert_eq!(ram16k.out([O, I, I, O, I, O, O, I, O, O, I, I, O, O]), b);
    }
}
