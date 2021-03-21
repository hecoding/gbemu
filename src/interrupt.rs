use crate::cpu::CPU;

pub struct Interrupt {
    pub master: bool,
    pub delayed_master_enable: bool,
    pub delayed_master_disable: bool,
    enable: u8,
    flag: u8,
}

enum Interrupts {
    VBlank = 1 << 0,
    LCD = 1 << 1,
    Timer = 1 << 2,
    Transfer = 1 << 3,
    Keypad = 1 << 4,
}

impl Interrupt{
    pub fn new() -> Interrupt {
        Interrupt{
            master: true,
            delayed_master_enable: false,
            delayed_master_disable: false,
            enable: 0,
            flag: 0,
        }
    }

    pub fn step(&mut self, cpu: &mut CPU) {
        if self.master && self.enable != 0 && self.flag != 0 {
            let fired = self.enable & self.flag;

            if fired & (Interrupts::VBlank as u8) != 0 {
                self.flag &= !(Interrupts::VBlank as u8);
                self.master = false;
                cpu.jump_to(0x40);
            }

            if fired & (Interrupts::LCD as u8) != 0 {
                self.flag &= !(Interrupts::LCD as u8);
                self.master = false;
                cpu.jump_to(0x48);
            }

            if fired & (Interrupts::Timer as u8) != 0 {
                self.flag &= !(Interrupts::Timer as u8);
                self.master = false;
                cpu.jump_to(0x50);
            }

            if fired & (Interrupts::Transfer as u8) != 0 {
                self.flag &= !(Interrupts::Transfer as u8);
                self.master = false;
                cpu.jump_to(0x58);
            }

            if fired & (Interrupts::Keypad as u8) != 0 {
                self.flag &= !(Interrupts::Keypad as u8);
                self.master = false;
                cpu.jump_to(0x60);
            }
        }
    }
}
