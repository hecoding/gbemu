use crate::utils::{join_8_to_16, split_16_to_8};

pub struct Register {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

enum Flags {
    Zero = 1 << 7,
    Negative = 1 << 6,
    HalfCarry = 1 << 5,
    Carry = 1 << 4,
}

impl Register {
    pub fn new() -> Register {
        Register{ a: 0x01, f: 0xB0, b: 0, c: 0x13, d: 0, e: 0xD8, h: 0x01, l: 0x4D, sp: 0xfffe, pc: 0x100 }
    }

    pub fn get_hl(&self) -> u16 {
        join_8_to_16(self.h, self.l)
    }

    pub fn set_hl(&mut self, n: u16) {
        let ns = split_16_to_8(n);
        self.h = ns.0;
        self.l = ns.1;
    }

    pub fn get_rp(&self, i: u8) -> u16 {
        match i {
            0 => join_8_to_16(self.b, self.c),
            1 => join_8_to_16(self.d, self.e),
            2 => join_8_to_16(self.h, self.l),
            3 => self.sp,
            _ => panic!("Invalid double register")
        }
    }

    pub fn set_rp(&mut self, i: u8, n: u16) {
        let ns = split_16_to_8(n);
        match i {
            0 => {self.b = ns.0; self.c = ns.1},
            1 => {self.d = ns.0; self.e = ns.1},
            2 => {self.h = ns.0; self.l = ns.1},
            3 => self.sp = n,
            _ => panic!("Invalid double register")
        }
    }

    pub fn get_rp2(&self, i: u8) -> u16 {
        match i {
            3 => join_8_to_16(self.a, self.f),
            _ => self.get_rp(i)
        }
    }

    pub fn set_rp2(&mut self, i: u8, n: u16) {
        let ns = split_16_to_8(n);
        match i {
            3 => {self.a = ns.0; self.f = ns.1},
            _ => self.set_rp(i, n)
        }
    }

    fn set_bit(&mut self, bit: Flags, b: bool) {
        if b {
            self.f |= bit as u8
        } else {
            self.f &= !(bit as u8)
        }
    }

    pub fn set_zero_flag(&mut self, b: bool) {
        self.set_bit(Flags::Zero, b);
    }

    pub fn set_negative_flag(&mut self, b: bool) {
        self.set_bit(Flags::Negative, b);
    }

    pub fn set_half_carry_flag(&mut self, b: bool) {
        self.set_bit(Flags::HalfCarry, b);
    }

    pub fn set_carry_flag(&mut self, b: bool) {
        self.set_bit(Flags::Carry, b);
    }
}
