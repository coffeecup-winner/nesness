use nes::NES;
use rom::nes::NESFile;

mod cpu;
mod mem;
mod nes;
mod ppu;
mod rom;

fn main() {
    let path = std::env::args().nth(1).expect("Expected an argument");
    let data = std::fs::read(path).expect("Failed to read the ROM file");
    let rom = NESFile::load(&data).expect("Failed to load the ROM");
    let mut nes = NES::new(rom);
    loop {
        nes.tick();
        nes.wait_until_cpu_ready();
    }
}
