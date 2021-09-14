use crate::{cpu::CPU, ppu::PPU, rom::nes::NESFile};

use super::mmap::MemoryMap;

#[allow(clippy::upper_case_acronyms)]
pub struct NES {
    pub cpu: CPU,
    pub ppu: PPU,
    pub mmap: MemoryMap,

    total_ticks: u64, // Enough for ~25k years
    next_cpu_tick: u64,
    next_ppu_tick: u64,
}

impl NES {
    pub fn new(rom: NESFile) -> Self {
        let mut nes = NES {
            cpu: CPU::new(),
            ppu: PPU::new(),
            mmap: MemoryMap::new(rom.header.mapper, rom.prg_rom),
            total_ticks: 0,
            next_cpu_tick: 0,
            next_ppu_tick: 0,
        };
        nes.reset();
        nes
    }

    #[cfg(test)]
    pub fn get_total_cycles(&self) -> u64 {
        self.total_ticks / 12
    }

    pub fn reset(&mut self) {
        self.cpu.reset(&self.mmap);
        self.total_ticks = 7 * 12; // CPU reset takes 7 cycles
        // Meanwhile PPU progresses through the first 7 * 3 dots
        for _ in 0..21 {
            self.ppu.run_one();
        }
        self.next_cpu_tick = self.total_ticks;
        self.next_ppu_tick = self.total_ticks;
    }

    #[inline]
    pub fn tick(&mut self) {
        // NTSC timings
        if self.total_ticks == self.next_cpu_tick {
            let cycles = self.cpu.run_one(&mut self.mmap);
            self.next_cpu_tick += cycles as u64 * 12;
        }
        if self.total_ticks == self.next_ppu_tick {
            self.ppu.run_one();
            self.next_ppu_tick += 4;
        }
        self.total_ticks += 1;
    }

    #[inline]
    pub fn wait_until_cpu_ready(&mut self) {
        while self.next_cpu_tick > self.total_ticks {
            self.tick();
        }
    }
}