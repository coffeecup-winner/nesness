use crate::{cpu::CPU, ppu::PPU, rom::nes::NESFile};

use super::mmap::MemoryMap;

#[allow(clippy::upper_case_acronyms)]
pub struct NES {
    pub cpu: CPU,
    pub ppu: PPU,
    pub mmap: MemoryMap,
}

impl NES {
    pub fn new(rom: NESFile) -> Self {
        let mut nes = NES {
            cpu: CPU::new(),
            ppu: PPU::new(),
            mmap: MemoryMap::new(rom.header.mapper, rom.prg_rom),
        };
        nes.reset();
        nes
    }

    pub fn reset(&mut self) {
        self.cpu.reset(&self.mmap);
    }
}
