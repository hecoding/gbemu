mod memory;
mod register;
mod cpu;
mod interrupt;
mod gpu;
mod timer;
mod utils;

use cpu::CPU;

fn main() {
    let clock_frequency: usize = 4194304; // Hertz
    let frame_rate: f64 = 59.63;
    let cycles_per_frame: usize = (clock_frequency as f64 / frame_rate).round() as usize;

    let mut cpu = CPU::new("roms/Tetris (World) (Rev A).gb");
    let mut cycles: usize = 0; // TODO usize or u32?
    loop {
        cycles = 0;
        while cycles < cycles_per_frame {
            cycles += cpu.step();
        }

        cpu.memory.step(cycles);
    }
}
