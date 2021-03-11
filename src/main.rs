use std::io::prelude::*;
use std::fs::File;

struct Memory {
    cart: Vec<u8>
}

impl Memory {
    fn new(filepath: &str) -> Memory {
        let mut f = File::open(filepath)
            .expect("Failed to read file");
        let mut cart = Vec::new();
        f.read_to_end(&mut cart).expect("Error reading data");
        cart.shrink_to_fit();

        Memory {
            cart
        }
    }
}

fn main() {
    let mem = Memory::new("roms/Tetris (World) (Rev A).gb");
    println!("{:?}", &mem.cart[0..10]);
}
