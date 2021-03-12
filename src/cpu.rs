use crate::memory::Memory;
use crate::register::Register;
use crate::utils::join_8_to_16;

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
        let op = self.read_immediate_8();
        self.exec(op);
    }

    fn read_immediate_8(&mut self) -> u8 {
        let op = self.memory.read_8(self.register.pc as usize);
        self.register.pc += 1;
        op
    }

    fn read_immediate_16(&mut self) -> u16 {
        join_8_to_16(self.read_immediate_8(), self.read_immediate_8())
    }

    fn stack_push(&mut self, n: u16) {
        self.register.sp -= 2;
        self.memory.write_16(self.register.sp as usize, n);
    }

    fn stack_pop(&mut self) -> u16 {
        let n = self.memory.read_16(self.register.sp as usize);
        self.register.sp += 2;
        n
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

    fn get_instruction_fields(&self, op: u8) -> (u8, u8, u8, u8, u8) {
        (op >> 6, op >> 3 & 7, op & 7, op >> 4 & 4, op >> 3 & 8)
    }

    fn exec(&mut self, op: u8) {
        let (x, y, z, p, q) = self.get_instruction_fields(op);

        match op {
            _ => panic!("Unimplemented op {:x}", op)
        }
    }
}
