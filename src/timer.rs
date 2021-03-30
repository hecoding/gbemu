pub struct Timer {
    divider: u8,
    counter: u8, // TIMA
    modulo: u8, // TMA
    control: u8, // TAC
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            divider: 0,
            counter: 0,
            modulo: 0,
            control: 0,
        }
    }

    pub fn read(&self, i: usize) -> u8 {
        match i {
            0xff04 => self.divider,
            0xff05 => self.counter,
            0xff06 => self.modulo,
            0xff07 => self.control,
            _ => panic!("Invalid timer read {}", i)
        }
    }

    pub fn write(&mut self, i: usize, n: u8) {
        match i {
            0xff04 => self.divider = 0,
            0xff05 => self.counter = n,
            0xff06 => self.modulo = n,
            0xff07 => self.control = n, // todo to have an enabled bool flag?
            _ => panic!("Invalid timer write {}", i)
        }
    }

    fn frequency(i: usize) -> usize {
        match i {
            0b00 => 4096,
            0b01 => 262144,
            0b10 => 65536,
            0b11 => 16384,
            _ => panic!("Invalid frequency value")
        }
    }

    pub fn step(&self, cycles: usize) {
        let overflow = false;
        if overflow {

        }
    }

    pub fn update_interrupt_flag(flags: &u8) {
        panic!("Unimplemented")
    }
}
