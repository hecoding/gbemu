mod memory;
mod register;
mod cpu;
mod utils;

use memory::Memory;

fn main() {
    let mem = Memory::new("roms/Tetris (World) (Rev A).gb");
    println!("{:?}", mem.read_16(0));
}
