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

impl Register {
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
}
