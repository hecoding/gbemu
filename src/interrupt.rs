pub struct Interrupt {
    pub master: bool,
    pub delayed_master_enable: bool,
    pub delayed_master_disable: bool,
    pub enable: u8,
    pub flag: u8,
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
            delayed_master_enable: false,
            delayed_master_disable: false,
            enable: 0,
            flag: 0,
        }
    }
}
