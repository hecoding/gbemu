struct Interrupt {
    master: bool,
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
// 0x40, 0x48, 0x50, 0x58, 0x60

impl Interrupt{
    pub fn new() -> Interrupt {
        panic!("unimplemented")
    }

    pub fn step(&self) {
        if !self.master {return}

        panic!("unimplemented")
    }
}

fn vblank() {}
fn lcd() {}
fn timer() {}
fn serial() {}
fn keypad() {}
