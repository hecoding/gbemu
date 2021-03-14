use bitmatch::bitmatch;

use crate::memory::Memory;
use crate::register::{Register, Flags};
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

    fn set_register(&mut self, i: u8, n: u8) {
        match i {
            0 => self.register.b = n,
            1 => self.register.c = n,
            2 => self.register.d = n,
            3 => self.register.e = n,
            4 => self.register.h = n,
            5 => self.register.l = n,
            6 => self.memory.write_8(self.register.get_hl() as usize, n),
            7 => self.register.a = n,
            _ => panic!("Invalid register")
        }
    }

    fn is_result_zero(n: u8) -> bool {
        n == 0
    }

    fn is_carry_from_bit(bit: u8, op1: u8, op2: u8) -> bool {
        let mask = (1 << (bit - 1)) - 1;
        ((op1 & mask) + (op2 & mask)) > mask
    }

    fn is_carry_from_bit_16(bit: u8, op1: u16, op2: u16) -> bool {
        let mask = (1 << (bit - 1)) - 1;
        ((op1 & mask) + (op2 & mask)) > mask
    }

    fn is_no_borrow_from_bit(bit: u8, op1: u8, op2: u8) -> bool {
        let mask = (1 << bit) - 1;
        (op1 & mask) < (op2 & mask)
    }

    fn jump_condition_check(&self, y: u8) -> bool {
        match y {
            0 => self.register.get_bit(Flags::Zero) == 0,
            1 => self.register.get_bit(Flags::Zero) == 1,
            2 => self.register.get_bit(Flags::Carry) == 0,
            3 => self.register.get_bit(Flags::Carry) == 1,
            _ => panic!("Invalid jump condition")
        }
    }

    #[bitmatch]
    fn exec(&mut self, op: u8) {
        println!("-----------");
        println!("op {:x}", op);
        #[bitmatch]
        match op {
            "0000_0000" => {}, // no op
            "0111_0110" => panic!("unimplemented halt"), // halt (overwriting a match below)
            // 8-bit loads
            "00yy_y110" => { // ld r, n
                let n = self.read_immediate_8();
                self.set_register(y, n);
            }
            "01yy_yzzz" => self.set_register(y, self.get_register(z)), // ld r1, r2
            "1110_1010" => { // ld (nn), a
                let nn = self.read_immediate_16();
                self.memory.write_8(nn as usize, self.register.a);
            }
            "1111_1010" => { // ld a, (nn)
                let nn = self.read_immediate_16();
                self.register.a = self.memory.read_8(nn as usize);
            }
            "1111_0010" => { // ld a, (c)
                self.register.a = self.memory.read_8(0xff00 + self.register.c as usize); // todo wrapping add?
            }
            "1110_0010" => { // ld (c), a
                self.memory.write_8(0xff00 + self.register.c as usize, self.register.a); // todo wrapping add?
            }
            "00pp_0010" => { // ld nn(+/-), a
                self.memory.write_8(self.register.get_rp3(p) as usize, self.register.a);

                let hl = self.register.get_hl();
                let hl_op = if p == 2 {hl.wrapping_add(1)} else if p == 3 {hl.wrapping_sub(1)} else {hl}; // todo check if this works
                self.register.set_hl(hl_op);
            }
            "00pp_1010" => { // ld a, nn(+/-)
                self.register.a = self.memory.read_8(self.register.get_rp3(p) as usize);

                let hl = self.register.get_hl();
                let hl_op = if p == 2 {hl.wrapping_add(1)} else if p == 3 {hl.wrapping_sub(1)} else {hl}; // todo check if this works
                self.register.set_hl(hl_op);
            }
            "1110_0000" => { // ldh (n), a
                let n = self.read_immediate_8();
                self.memory.write_8(0xff00 + n as usize, self.register.a); // todo wrapping add?
            }
            "1111_0000" => { // ldh a, (n)
                let n = self.read_immediate_8();
                self.register.a = self.memory.read_8(0xff00 + n as usize); // todo wrapping add?
            }
            // 16-bit loads
            "00pp_0001" => { // ld n, nn
                let nn = self.read_immediate_16();
                self.register.set_rp(p, nn)
            }
            "1111_1001" => self.register.sp = self.register.get_hl(), // ld sp, hl
            "1111_1000" => { // ld hl, sp+d
                let sp = self.register.sp;
                let d_raw = self.read_immediate_8();
                let d = i16::from(d_raw as i8) as u16;
                self.register.set_hl(sp.wrapping_add(d));

                self.register.set_zero_flag(false);
                self.register.set_negative_flag(false);
                self.register.set_half_carry_flag(CPU::is_carry_from_bit_16(4, sp, d));
                self.register.set_carry_flag(CPU::is_carry_from_bit_16(8, sp, d));
            },
            "0000_1000" => { // ld (nn), sp
                let nn = self.read_immediate_16();
                self.memory.write_16(nn as usize, self.register.sp)
            },
            "11pp_0101" => self.stack_push(self.register.get_rp2(p)), // push nn
            "11pp_0001" => { // pop nn
                let nn = self.stack_pop();
                self.register.set_rp2(p, nn)
            },
            // 8-bit alu
            "11yy_y110" => {
                let n = self.read_immediate_8();
                self.alu(y, n);
            }
            "10yy_yzzz" => {
                let n = self.get_register(z);
                self.alu(y, n);
            }
            "00pp_p100" => { // inc n
                let n = self.get_register(p);
                let result = n.wrapping_add(1);
                self.set_register(p, result);

                self.register.set_zero_flag(CPU::is_result_zero(result));
                self.register.set_negative_flag(false);
                self.register.set_half_carry_flag(CPU::is_carry_from_bit(3, n, 1));
            }
            "00pp_p101" => { // dec n
                let n = self.get_register(p);
                let result = n.wrapping_sub(1);
                self.set_register(p, result);

                self.register.set_zero_flag(CPU::is_result_zero(result));
                self.register.set_negative_flag(true);
                self.register.set_half_carry_flag(CPU::is_no_borrow_from_bit(4, n, 1));
            }
            // 16-bit arithmetic
            "00pp_1001" => { // add hl, n
                let hl = self.register.get_hl();
                let n = self.register.get_rp(p);
                self.register.set_hl(hl.wrapping_add(n));

                self.register.set_negative_flag(false);
                self.register.set_half_carry_flag(CPU::is_carry_from_bit_16(11, hl, n));
                self.register.set_carry_flag(CPU::is_carry_from_bit_16(15, hl, n));
            }
            "1110_1000" => { // add sp, n
                let sp = self.register.sp;
                let n_raw = self.read_immediate_8();
                let n = i16::from(n_raw as i8) as u16;
                self.register.sp = sp.wrapping_add(n);

                self.register.set_zero_flag(false);
                self.register.set_negative_flag(false);
                self.register.set_half_carry_flag(CPU::is_carry_from_bit_16(4, sp, n));
                self.register.set_carry_flag(CPU::is_carry_from_bit_16(8, sp, n));
            }
            "00pp_0011" => { // inc nn
                self.register.set_rp(p, self.register.get_rp(p).wrapping_add(1));
            }
            "00pp_1011" => { // dec nn
                self.register.set_rp(p, self.register.get_rp(p).wrapping_sub(1));
            }
            // misc
            // rotations and shifts
            // bit ops
            // jumps
            "1100_0011" => self.register.pc = self.read_immediate_16(), // jp nn
            "11yy_y010" => { // jp cc, nn
                if self.jump_condition_check(y) {
                    self.register.pc = self.read_immediate_16(); // todo check if endianness is correct
                }
            }
            "1110_1001" => self.register.pc = self.register.get_hl(), // jp hl
            "0001_1000" => { // jr n
                let n = self.read_immediate_8() as i8;
                if n > 0 {
                    self.register.pc = self.register.pc.wrapping_add(n as u16)
                } else {
                    self.register.pc = self.register.pc.wrapping_sub(n.abs() as u16)
                }
            },
            "00yy_y000" => { // jr cc, n
                if self.jump_condition_check(y - 4) {
                    let n = self.read_immediate_8() as i8;
                    if n > 0 {
                        self.register.pc = self.register.pc.wrapping_add(n as u16)
                    } else {
                        self.register.pc = self.register.pc.wrapping_sub(n.abs() as u16)
                    }
                }
            }
            // calls
            // restarts
            "11yy_y111" => { // rst n
                self.stack_push(self.register.pc);
                self.register.pc = y as u16 * 8;
            }
            // returns
            _ => panic!("Unimplemented op {:x}", op)
        }
        println!("{:#x?}", self.register)
    }

    fn alu(&mut self, y: u8, n: u8) {
        let a = self.register.a;
        let carry_flag = self.register.get_carry_flag();

        match y {
            0 => { // add
                self.register.a = a.wrapping_add(n);

                self.register.set_zero_flag(CPU::is_result_zero(self.register.a));
                self.register.set_negative_flag(false);
                self.register.set_half_carry_flag(CPU::is_carry_from_bit(3, a, n));
                self.register.set_carry_flag(CPU::is_carry_from_bit(7, a, n));
            },
            1 => { // adc a
                let n = n.wrapping_add(carry_flag);
                self.register.a = a.wrapping_add(n);

                self.register.set_zero_flag(CPU::is_result_zero(self.register.a));
                self.register.set_negative_flag(false);
                self.register.set_half_carry_flag(CPU::is_carry_from_bit(3, a, n));
                self.register.set_carry_flag(CPU::is_carry_from_bit(7, a, n));
            },
            2 => { // sub
                self.register.a = a.wrapping_sub(n);

                self.register.set_zero_flag(CPU::is_result_zero(self.register.a));
                self.register.set_negative_flag(true);
                self.register.set_half_carry_flag(CPU::is_no_borrow_from_bit(4, a, n));
                self.register.set_carry_flag(CPU::is_no_borrow_from_bit(1, a, n));
            },
            3 => { // sbc a
                let n = n.wrapping_sub(carry_flag);
                self.register.a = a.wrapping_sub(n);

                self.register.set_zero_flag(CPU::is_result_zero(self.register.a));
                self.register.set_negative_flag(true);
                self.register.set_half_carry_flag(CPU::is_no_borrow_from_bit(4, a, n));
                self.register.set_carry_flag(CPU::is_no_borrow_from_bit(1, a, n));
            },
            4 => { // and
                self.register.a = a & n;

                self.register.set_zero_flag(CPU::is_result_zero(self.register.a));
                self.register.set_negative_flag(false);
                self.register.set_half_carry_flag(true);
                self.register.set_carry_flag(false);
            },
            5 => { // xor
                self.register.a = a ^ n;

                self.register.set_zero_flag(CPU::is_result_zero(self.register.a));
                self.register.set_negative_flag(false);
                self.register.set_half_carry_flag(false);
                self.register.set_carry_flag(false);
            },
            6 => { // or
                self.register.a = a | n;

                self.register.set_zero_flag(CPU::is_result_zero(self.register.a));
                self.register.set_negative_flag(false);
                self.register.set_half_carry_flag(false);
                self.register.set_carry_flag(false);
            },
            7 => { // cp
                self.alu(2, n);
                self.register.a = a
            },
            _ => panic!("Illegal alu opcode {}", y)
        }
    }
}
