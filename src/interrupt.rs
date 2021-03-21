pub struct Interrupt {
    pub master: bool,
    pub delayed_enable: usize,
    pub delayed_disable: usize,
}

pub enum Interrupts {
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
            delayed_enable: 0,
            delayed_disable: 0,
        }
    }

    pub fn update_delays(&mut self) {
        match self.delayed_enable {
            2 => self.delayed_enable = 1,
            1 => { self.delayed_enable = 0; self.master = true }
            _ => {},
        }
        match self.delayed_disable {
            2 => self.delayed_disable = 1,
            1 => { self.delayed_disable = 0; self.master = false }
            _ => {},
        }
    }
}
