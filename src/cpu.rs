use bitmatch::bitmatch;

use crate::memory::Memory;
use crate::register::{Register, Flags};
use crate::interrupt::{Interrupt, Interrupts};
use crate::utils::{join_8_to_16, join_8_to_16_lsf};

pub struct CPU {
    register: Register,
    memory: Memory,
    interrupt: Interrupt,
    halted: bool,
}

impl CPU {
    pub fn new(filepath: &str) -> CPU {
        CPU {
            register: Register::new(),
            memory: Memory::new(filepath),
            interrupt: Interrupt::new(),
            halted: false,
        }
    }

    pub fn step(&mut self) -> usize {
        match self.interrupt_step() {
            0 => {},
            n => return n,
        }

        if self.halted { 4 } else { self.exec() }
    }

    fn read_immediate_8(&mut self) -> u8 {
        let op = self.memory.read_8(self.register.pc as usize);
        self.register.pc += 1;
        op
    }

    fn read_immediate_16(&mut self) -> u16 {
        join_8_to_16_lsf(self.read_immediate_8(), self.read_immediate_8())
    }

    fn stack_push(&mut self, n: u16) {
        self.register.sp -= 2;
        self.memory.write_16(self.register.sp as usize, n); // todo is this write lsf?
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

    fn set_register(&mut self, i: u8, n: u8) -> usize {
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

        if i == 6 { 12 } else { 4 }
    }

    fn get_bit_from_register(&self, register: u8, bit: u8) -> bool {
        let b = self.get_register(register) & (1 << bit);
        b != 0
    }

    fn set_bit_from_register(&mut self, register: u8, bit: u8, value: bool) {
        let mut r = self.get_register(register);
        let mask = 1 << bit;
        if value { // same as registers.set_bit()
            r |= mask as u8
        } else {
            r &= !(mask as u8)
        }
        self.set_register(register, r);
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
            0 => self.register.get_bit(Flags::Zero) == false,
            1 => self.register.get_bit(Flags::Zero) == true,
            2 => self.register.get_bit(Flags::Carry) == false,
            3 => self.register.get_bit(Flags::Carry) == true,
            _ => panic!("Invalid jump condition")
        }
    }

    #[bitmatch]
    fn exec(&mut self) -> usize {
        let op = self.read_immediate_8();
        println!("{:#x?}", self.register);
        println!("interrupt master {}, enable {}, flag {}", self.interrupt.master, self.memory.interrupt_enable, self.memory.interrupt_flag);
        println!("-----------");
        println!("op {:x}", op);
        if op == 0x3e {
            println!("yeee")
        }
        #[bitmatch]
        match op {
            "0000_0000" => { 4 }, // no op
            "0111_0110" => { self.halted = true; 4 }, // halt (overwriting a match below) TODO check if overwrites
            // 8-bit loads
            "00yy_y110" => { // ld r, n
                let n = self.read_immediate_8();
                self.set_register(y, n)
            }
            "01yy_yzzz" => self.set_register(y, self.get_register(z)), // ld r1, r2
            "1110_1010" => { // ld (nn), a
                let nn = self.read_immediate_16();
                self.memory.write_8(nn as usize, self.register.a);
                0
            }
            "1111_1010" => { // ld a, (nn)
                let nn = self.read_immediate_16();
                self.register.a = self.memory.read_8(nn as usize);
                0
            }
            "1111_0010" => { // ld a, (c)
                self.register.a = self.memory.read_8(0xff00 + self.register.c as usize);
                0
            }
            "1110_0010" => { // ld (c), a
                self.memory.write_8(0xff00 + self.register.c as usize, self.register.a);
                0
            }
            "00pp_0010" => { // ld nn(+/-), a
                self.memory.write_8(self.register.get_rp3(p) as usize, self.register.a);

                let hl = self.register.get_hl();
                let hl_op = if p == 2 {hl.wrapping_add(1)} else if p == 3 {hl.wrapping_sub(1)} else {hl};
                self.register.set_hl(hl_op);
                0
            }
            "00pp_1010" => { // ld a, nn(+/-)
                self.register.a = self.memory.read_8(self.register.get_rp3(p) as usize);

                let hl = self.register.get_hl();
                let hl_op = if p == 2 {hl.wrapping_add(1)} else if p == 3 {hl.wrapping_sub(1)} else {hl};
                self.register.set_hl(hl_op);
                0
            }
            "1110_0000" => { // ldh (n), a
                let n = self.read_immediate_8();
                self.memory.write_8(0xff00 + n as usize, self.register.a);
                0
            }
            "1111_0000" => { // ldh a, (n)
                let n = self.read_immediate_8();
                self.register.a = self.memory.read_8(0xff00 + n as usize);
                0
            }
            // 16-bit loads
            "00pp_0001" => { // ld n, nn
                let nn = self.read_immediate_16();
                self.register.set_rp(p, nn);
                0
            }
            "1111_1001" => { self.register.sp = self.register.get_hl(); 0 }, // ld sp, hl
            "1111_1000" => { // ld hl, sp+d
                let sp = self.register.sp;
                let d_raw = self.read_immediate_8();
                let d = i16::from(d_raw as i8) as u16;
                self.register.set_hl(sp.wrapping_add(d));
                let cycles = 0;

                self.register.set_zero_flag(false);
                self.register.set_negative_flag(false);
                self.register.set_half_carry_flag(CPU::is_carry_from_bit_16(4, sp, d));
                self.register.set_carry_flag(CPU::is_carry_from_bit_16(8, sp, d));
                cycles
            },
            "0000_1000" => { // ld (nn), sp
                let nn = self.read_immediate_16();
                self.memory.write_16(nn as usize, self.register.sp); // todo is this write lsf?
                0
            },
            "11pp_0101" => { self.stack_push(self.register.get_rp2(p)); 0 }, // push nn
            "11pp_0001" => { // pop nn
                let nn = self.stack_pop();
                self.register.set_rp2(p, nn);
                0
            },
            // 8-bit alu
            "11yy_y110" => { // alu n
                let n = self.read_immediate_8();
                self.alu(y, n);
                0
            }
            "10yy_yzzz" => { // alu r
                let n = self.get_register(z);
                self.alu(y, n);
                0
            }
            "00pp_p100" => { // inc n
                let n = self.get_register(p);
                let result = n.wrapping_add(1);
                self.set_register(p, result);
                let cycles = 0;

                self.register.set_zero_flag(CPU::is_result_zero(result));
                self.register.set_negative_flag(false);
                self.register.set_half_carry_flag(CPU::is_carry_from_bit(3, n, 1));
                cycles
            }
            "00pp_p101" => { // dec n
                let n = self.get_register(p);
                let result = n.wrapping_sub(1);
                self.set_register(p, result);
                let cycles = 0;

                self.register.set_zero_flag(CPU::is_result_zero(result));
                self.register.set_negative_flag(true);
                self.register.set_half_carry_flag(CPU::is_no_borrow_from_bit(4, n, 1));
                cycles
            }
            // 16-bit arithmetic
            "00pp_1001" => { // add hl, n
                let hl = self.register.get_hl();
                let n = self.register.get_rp(p);
                self.register.set_hl(hl.wrapping_add(n));
                let cycles = 0;

                self.register.set_negative_flag(false);
                self.register.set_half_carry_flag(CPU::is_carry_from_bit_16(11, hl, n));
                self.register.set_carry_flag(CPU::is_carry_from_bit_16(15, hl, n));
                cycles
            }
            "1110_1000" => { // add sp, n
                let sp = self.register.sp;
                let n_raw = self.read_immediate_8();
                let n = i16::from(n_raw as i8) as u16;
                self.register.sp = sp.wrapping_add(n);
                let cycles = 0;

                self.register.set_zero_flag(false);
                self.register.set_negative_flag(false);
                self.register.set_half_carry_flag(CPU::is_carry_from_bit_16(4, sp, n));
                self.register.set_carry_flag(CPU::is_carry_from_bit_16(8, sp, n));
                cycles
            }
            "00pp_0011" => { // inc nn
                self.register.set_rp(p, self.register.get_rp(p).wrapping_add(1));0
            }
            "00pp_1011" => { // dec nn
                self.register.set_rp(p, self.register.get_rp(p).wrapping_sub(1));0
            }
            // misc (some in exec_alt)
            "0010_0111" => { // daa
                let a = self.register.a;
                let mut adjust = if self.register.get_carry_flag() { 0x60 } else { 0x00 };

                if self.register.get_bit(Flags::HalfCarry) {
                    adjust |= 0x06;
                }
                if !self.register.get_bit(Flags::Negative) {
                    if a & 0x0f > 0x09 { adjust |= 0x06; }
                    if a > 0x99 { adjust |= 0x60; }
                    self.register.a = a.wrapping_add(adjust);
                } else {
                    self.register.a = a.wrapping_sub(adjust);
                }

                self.register.set_zero_flag(CPU::is_result_zero(self.register.a));
                self.register.set_half_carry_flag(false);
                self.register.set_carry_flag(adjust >= 0x60);
                0
            }
            "0010_1111" => { // cpl
                self.register.a = !self.register.a;
                self.register.set_negative_flag(true);
                self.register.set_half_carry_flag(true);
                0
            }
            "0011_1111" => { // ccf
                self.register.set_negative_flag(false);
                self.register.set_half_carry_flag(false);
                self.register.set_carry_flag(!self.register.get_carry_flag());
                0
            }
            "0011_0111" => { // scf
                self.register.set_negative_flag(false);
                self.register.set_half_carry_flag(false);
                self.register.set_carry_flag(true);
                0
            }
            "0001_0000" => { panic!("Unimplemented stop") } // stop todo what to do?
            "1111_0011" => { self.interrupt.delayed_disable = 2; 0 } // di
            "1111_1011" => { self.interrupt.delayed_enable = 2; 0 } // ei
            // rotations and shifts (some in exec_alt)
            "000y_y111" => self.rot(y, 7),
            // bit ops (all in exec_alt)
            // jumps
            "1100_0011" => { self.register.pc = self.read_immediate_16(); 0 } // jp nn
            "11yy_y010" => { // jp cc, nn
                let nn = self.read_immediate_16(); // todo check if endianness is correct
                if self.jump_condition_check(y) {
                    self.register.pc = nn;
                }
                0
            }
            "1110_1001" => { self.register.pc = self.register.get_hl(); 0 } // jp hl
            "0001_1000" => { // jr n
                let n = self.read_immediate_8() as i8;
                if n > 0 {
                    self.register.pc = self.register.pc.wrapping_add(n as u16)
                } else {
                    self.register.pc = self.register.pc.wrapping_sub(n.abs() as u16)
                }
                0
            },
            "00yy_y000" => { // jr cc, n
                let n = self.read_immediate_8() as i8;
                if self.jump_condition_check(y - 4) {
                    if n > 0 {
                        self.register.pc = self.register.pc.wrapping_add(n as u16)
                    } else {
                        self.register.pc = self.register.pc.wrapping_sub(n.abs() as u16)
                    }
                }
                0
            }
            // calls
            "1100_1101" => { // call nn
                self.stack_push(self.register.pc);
                self.register.pc = self.read_immediate_16(); // todo check endianness
                0
            }
            "11yy_y100" => { // call cc, nn
                if self.jump_condition_check(y) {
                    self.stack_push(self.register.pc);
                    self.register.pc = self.read_immediate_16(); // todo check endianness
                }
                0
            }
            // restarts
            "11yy_y111" => { // rst n
                self.stack_push(self.register.pc);
                self.register.pc = y as u16 * 8;
                0
            }
            // returns
            "1100_1001" => { // ret
                self.register.pc = self.stack_pop(); // todo check if ok with current endianness
                0
            }
            "11yy_y000" => { // ret cc
                if self.jump_condition_check(y) {
                    self.register.pc = self.stack_pop(); // todo check if ok with current endianness
                }
                0
            }
            "1101_1001" => { // reti
                self.register.pc = self.stack_pop(); // todo check if ok with current endianness
                self.interrupt.master = true;
                0
            }
            "11001011" => { // cb prefix, using alt opcodes
                let op_next = self.read_immediate_8();
                self.exec_alt(op_next); // todo check if this reaches cb
                0
            }
            _ => panic!("Unimplemented op {:x}", op)
        }
    }

    fn alu(&mut self, y: u8, n: u8) {
        let a = self.register.a;
        let carry_flag = self.register.get_carry_flag() as u8;

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

    fn rot(&mut self, y: u8, z: u8) -> usize {
        let n = self.get_register(z);

        let (carry, result) = match y {
            0 => { // rlc n
                let carry = (n & 0x80) != 0;
                (carry, (n << 1) | carry as u8)
            },
            1 => { // rrc n
                let carry = (n & 0x1) != 0;
                (carry, (n >> 1) | (0x80 * carry as u8))
            },
            2 => { // rl n
                ((n & 0x80) != 0, (n << 1) | self.register.get_carry_flag() as u8)
            },
            3 => { // rr n
                ((n & 0x1) != 0, (n >> 1) | (0x80 * self.register.get_carry_flag() as u8))
            },
            4 => { // sla n
                ((n & 0x80) != 0, n << 1)
            },
            5 => { // sra n
                ((n & 0x1) != 0, (n >> 1) | (0x80 & n))
            },
            6 => { // swap n
                (false, (n << 4) | (n >> 4))
            },
            7 => { // srl n
                ((n & 0x1) != 0, (n >> 1) & !0x80)
            },
            _ => panic!("Illegal rot opcode {}", y)
        };

        let cycles = self.set_register(z, result);

        self.register.set_zero_flag(CPU::is_result_zero(result));
        self.register.set_negative_flag(false);
        self.register.set_half_carry_flag(false);
        self.register.set_carry_flag(carry);
        cycles
    }

    #[bitmatch]
    fn exec_alt(&mut self, op: u8) -> usize {
        (#[bitmatch]
        match op {
            // misc

            // rotates shifts
            "00yy_yzzz" => self.rot(y, z),

            // bit ops
            "01yy_yzzz" => { // bit b, r
                let bit = self.get_bit_from_register(z, y);

                self.register.set_zero_flag(bit == false);
                self.register.set_negative_flag(false);
                self.register.set_half_carry_flag(true);
                0
            }
            "11yy_yzzz" => { // set b, r
                self.set_bit_from_register(z, y, true);
                0
            }
            "10yy_yzzz" => { // res b, r
                self.set_bit_from_register(z, y, false);
                0
            }
            _ => panic!("Unimplemented 0xcb prefixed op {:x}", op)
        }) + 4
    }

    fn interrupt_step(&mut self) -> usize {
        self.interrupt.update_delays();

        if self.interrupt.master && self.memory.interrupt_enable != 0 && self.memory.interrupt_flag != 0 {
            let fired = self.memory.interrupt_enable & self.memory.interrupt_flag;

            if fired & (Interrupts::VBlank as u8) != 0 {
                self.memory.interrupt_flag &= !(Interrupts::VBlank as u8);
                self.interrupt.master = false;
                self.stack_push(self.register.pc);
                self.register.pc = 0x40;
                return 16 // todo are all 16 or 12?
            }

            if fired & (Interrupts::LCD as u8) != 0 {
                self.memory.interrupt_flag &= !(Interrupts::LCD as u8);
                self.interrupt.master = false;
                self.stack_push(self.register.pc);
                self.register.pc = 0x48;
                return 16
            }

            if fired & (Interrupts::Timer as u8) != 0 {
                self.memory.interrupt_flag &= !(Interrupts::Timer as u8);
                self.interrupt.master = false;
                self.stack_push(self.register.pc);
                self.register.pc = 0x50;
                return 16
            }

            if fired & (Interrupts::Transfer as u8) != 0 {
                self.memory.interrupt_flag &= !(Interrupts::Transfer as u8);
                self.interrupt.master = false;
                self.stack_push(self.register.pc);
                self.register.pc = 0x58;
                return 16
            }

            if fired & (Interrupts::Keypad as u8) != 0 {
                self.memory.interrupt_flag &= !(Interrupts::Keypad as u8);
                self.interrupt.master = false;
                self.stack_push(self.register.pc);
                self.register.pc = 0x60;
                return 16
            }
        }
        0
    }
}
