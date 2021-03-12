use crate::memory::Memory;
use crate::register::Register;

struct CPU {
    register: Register,
    memory: Memory,
}

impl CPU {
    fn get_register(&self, i: u8) -> u8 {
        match i {
            0 => self.register.b,
            1 => self.register.c,
            2 => self.register.d,
            3 => self.register.e,
            4 => self.register.h,
            5 => self.register.l,
            6 => self.memory.read_8(self.register.get_hl() as usize),
            7 => self.register.a,
            _ => panic!("Invalid register")
        }
    }
}
