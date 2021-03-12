use crate::memory::Memory;
use crate::register::Register;

pub struct CPU {
    register: Register,
    memory: Memory,
}

impl CPU {
    pub fn new(filepath: &str) -> CPU {
        CPU {
            register: Register::new(),
            memory: Memory::new(filepath),
        }
    }

    pub fn step(&mut self) {
        let op = self.read_instruction();
        self.exec(op);
    }

    fn read_instruction(&mut self) -> u8 {
        let op = self.memory.read_8(self.register.pc as usize);
        self.register.pc += 1;
        op
    }

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

    fn exec(&mut self, op: u8) {

        match op {
            _ => panic!("Unimplemented op {:x}", op)
        }
    }
}
