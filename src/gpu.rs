use crate::cpu::Cycles;

#[derive(Copy,Clone)]
enum PixelGrayScale {
    Zero,
    One,
    Two,
    Three,
}

impl PixelGrayScale {
    fn from_bools(b1: bool, b2: bool) -> PixelGrayScale {
        match (b1, b2) {
            (true, true) => PixelGrayScale::Three,
            (false, true) => PixelGrayScale::Two,
            (true, false) => PixelGrayScale::One,
            (false, false) => PixelGrayScale::Zero,
        }
    }
}

type Tile = Vec<Vec<PixelGrayScale>>;
fn tile_new() -> Vec<Vec<PixelGrayScale>> {
    vec![vec![PixelGrayScale::Zero; 8]; 8]
}

pub struct GPU {
    vram: Vec<u8>,
    tile_cache: Vec<Tile>,
}

impl GPU {
    pub fn new() -> GPU {
        GPU {
            vram: vec![0; 0x2000],
            tile_cache: vec![tile_new(); 128 * 3],
        }
    }

    pub fn read_vram(&self, address: usize) -> u8 {
        self.vram[address]
    }

    pub fn write_vram(&mut self, address: usize, value: u8) {
        self.vram[address] = value;

        if address < 0x1800 {
            self.update_tile_cache(address);
        }
    }

    fn update_tile_cache(&mut self, address: usize) {
        let first_byte_address = address & 0xfffe;
        let byte1 = self.vram[first_byte_address];
        let byte2 = self.vram[first_byte_address + 1];

        let tile_index = address / 16;
        let row_index = (address % 16) / 2;
        for pixel_index in 0..8 {
            let mask = 1 << (7 - pixel_index);
            self.tile_cache[tile_index][row_index][pixel_index] =
                PixelGrayScale::from_bools(byte1 & mask != 0, byte2 & mask != 0);
        }
    }

    pub fn step(&self, cycles: Cycles) {

    }
}
