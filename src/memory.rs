use std::fs::File;
use std::io::Read;
use crate::utils::{join_8_to_16, split_16_to_8};

pub struct Memory {
    cart: Vec<u8>,
    stack: Vec<u8>, // stack in GMB Z80 is a part of the regular memory
}

const STACK_OFFSET: usize = 0xff80;

impl Memory {
    pub fn new(filepath: &str) -> Memory {
        Memory {
            cart: Memory::read_cartridge(filepath),
            stack: Vec::with_capacity(0x80),
        }
    }

    fn read_cartridge(filepath: &str) -> Vec<u8> {
        let mut f = File::open(filepath)
            .expect("Failed to read file");
        let mut cart = Vec::new();
        f.read_to_end(&mut cart).expect("Error reading data");
        cart.shrink_to_fit();
        cart
    }

    pub fn read_8(&self, i: usize) -> u8 {
        match i {
            0..=0x8000 => self.cart[i],
            0xff80..=0xffff => self.stack[i - STACK_OFFSET],
            _ => panic!("mem access {}", i),
        }
    }

    pub fn write_8(&mut self, i: usize, n: u8) {
        match i {
            0..=0x8000 => self.cart[i] = n,
            0xff80..=0xffff => self.stack[i - STACK_OFFSET] = n,
            _ => panic!("mem access {}", i),
        }
    }

    pub fn read_16(&self, i: usize) -> u16 {
        join_8_to_16(self.read_8(i), self.read_8(i + 1))
    }

    pub fn write_16(&mut self, i: usize, n: u16) {
        let ns = split_16_to_8(n);
        self.write_8(i, ns.0);
        self.write_8(i + 1, ns.1);
    }
}
