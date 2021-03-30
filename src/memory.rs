use std::fs::File;
use std::io::Read;
use crate::gpu::GPU;
use crate::timer::Timer;
use crate::utils::{join_8_to_16, split_16_to_8};

pub struct Memory {
    cart: Vec<u8>,
    gpu: GPU,
    switchable_ram: Vec<u8>,
    ram: Vec<u8>, // in cgb mode this is split in bank 0 and switchable bank 1
    oam: Vec<u8>, // sprites stuff
    io_port: Vec<u8>,
    stack: Vec<u8>, // stack in GMB Z80 is a part of the regular memory, simply called zero-page ram

    pub interrupt_enable: u8,
    pub interrupt_flag: u8,
    timer: Timer,
}

const STACK_OFFSET: usize = 0xff80;

impl Memory {
    pub fn new(filepath: &str) -> Memory {
        Memory {
            cart: Memory::read_cartridge(filepath),
            gpu: GPU::new(),
            switchable_ram: vec![0; 0x2000],
            ram: vec![0; 0x2000],
            oam: vec![0; 0x100],
            io_port: vec![0; 0x100],
            stack: vec![0; 0x80],

            interrupt_enable: 0,
            interrupt_flag: 0,
            timer: Timer::new(),
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
        match i { // TODO implement the "read 0s" and so from invalid regions
            0..=0x7fff => self.cart[i],
            0x8000..=0x9fff => self.gpu.read_vram(i - 0x8000),
            0xa000..=0xbfff => self.switchable_ram[i - 0xa000],
            0xc000..=0xdfff => self.ram[i - 0xc000],
            0xe000..=0xfdff => self.ram[i - 0xe000], // ram echo
            0xff04..=0xff07 => self.timer.read(i),
            0xff0f => self.interrupt_flag,
            0xff80..=0xfffe => self.stack[i - STACK_OFFSET],
            0xffff => self.interrupt_enable,
            _ => panic!("mem read {}", i),
        }
    }

    pub fn write_8(&mut self, i: usize, n: u8) {
        match i { // TODO implement the "do nothing" and so from invalid regions
            0..=0x7fff => self.cart[i] = n,
            0x8000..=0x9fff => self.gpu.write_vram(i - 0x8000, n),
            0xa000..=0xbfff => self.switchable_ram[i - 0xa000] = n,
            0xc000..=0xdfff => self.ram[i - 0xc000] = n,
            0xe000..=0xfdff => self.ram[i - 0xe000] = n, // ram echo
            0xff04..=0xff07 => self.timer.write(i, n),
            0xff0f => self.interrupt_flag = n,
            0xff80..=0xfffe => self.stack[i - STACK_OFFSET] = n,
            0xffff => self.interrupt_enable = n,
            _ => panic!("mem write {}", i),
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

    pub fn step(&self, cycles: usize) {

    }
}
