mod memory;
mod register;
mod cpu;
mod interrupt;
mod utils;

use cpu::CPU;

fn main() {
    let mut cpu = CPU::new("roms/Tetris (World) (Rev A).gb");
    loop {
        cpu.step()
    }
}
