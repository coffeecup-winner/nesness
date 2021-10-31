use crate::{
    mem::{
        mappers::{self, Mapper},
        Memory,
    },
    ppu::PPU,
};

pub struct CpuMemoryMap {
    // Main RAM - 0x0000..0x1fff
    pub ram: [u8; 0x0800],

    // PPU (registers) - 0x2000..0x3fff
    // TODO: This type should be renamed/moved
    pub ppu: PPU,

    // PPU memory map
    // TODO: This should be moved
    pub ppu_mmap: PpuMemoryMap,

    // APU and I/O registers - 0x4000..0x4020
    apu: [u8; 32],

    // Cartridge space - 0x4020..0xffff
    mapper: Box<dyn Mapper>,
    prg_rom: Vec<Vec<u8>>,
}

pub struct PpuMemoryMap {
    // PPU RAM (VRAM) - 0x2000..0x3fff
    // TODO: mirror and split the attribute memory out
    vram: [u8; 0x2000],

    mapper: Box<dyn Mapper>,
    chr_rom: Vec<Vec<u8>>,
}

impl CpuMemoryMap {
    pub fn new(mapper: u8, prg_rom: Vec<Vec<u8>>, chr_rom: Vec<Vec<u8>>) -> Self {
        CpuMemoryMap {
            ram: [0x00; 0x0800],
            ppu: PPU::new(),
            ppu_mmap: PpuMemoryMap::new(mapper, chr_rom),
            apu: [0x00; 32],
            mapper: mappers::get_mapper(mapper),
            prg_rom,
        }
    }
}

impl PpuMemoryMap {
    pub fn new(mapper: u8, chr_rom: Vec<Vec<u8>>) -> Self {
        PpuMemoryMap {
            vram: [0x00; 0x2000],
            mapper: mappers::get_mapper(mapper),
            chr_rom,
        }
    }
}

impl Memory for CpuMemoryMap {
    fn read_u8(&self, addr: u16) -> u8 {
        match addr >> 12 {
            0 | 1 => self.ram[(addr & 0x7ff) as usize],
            2 | 3 => match addr & 0x7 {
                0 => self.ppu.read_ppuctrl(),
                1 => self.ppu.read_ppumask(),
                2 => self.ppu.read_ppustatus(),
                3 => self.ppu.read_oamaddr(),
                4 => self.ppu.read_oamdata(),
                5 => self.ppu.read_ppuscroll(),
                6 => self.ppu.read_ppuaddr(),
                7 => self.ppu.read_ppudata(&self.ppu_mmap),
                _ => unreachable!(),
            },
            4 if (addr & 0xfff == 0x14) => self.ppu.read_oamdma(),
            4 if (addr as u8) < 0x20 => self.apu[addr as u8 as usize],
            _ => *self.mapper.prg_map(addr, &self.prg_rom),
        }
    }

    fn write_u8(&mut self, addr: u16, value: u8) {
        match addr >> 12 {
            0 | 1 => self.ram[(addr & 0x7ff) as usize] = value,
            2 | 3 => match addr & 0x7 {
                0 => self.ppu.write_ppuctrl(value),
                1 => self.ppu.write_ppumask(value),
                2 => self.ppu.write_ppustatus(value),
                3 => self.ppu.write_oamaddr(value),
                4 => self.ppu.write_oamdata(value),
                5 => self.ppu.write_ppuscroll(value),
                6 => self.ppu.write_ppuaddr(value),
                7 => self.ppu.write_ppudata(&mut self.ppu_mmap, value),
                _ => unreachable!(),
            },
            4 if (addr & 0xfff == 0x14) => self.ppu.write_oamdma(value),
            4 if (addr as u8) < 0x20 => self.apu[addr as u8 as usize] = value,
            _ => *self.mapper.prg_map_mut(addr, &mut self.prg_rom) = value,
        }
    }
}

impl Memory for PpuMemoryMap {
    fn read_u8(&self, mut addr: u16) -> u8 {
        addr &= 0x3fff;
        match addr {
            0x0000..=0x1fff => *self.mapper.chr_map(addr, &self.chr_rom),
            0x2000..=0x2fff => self.vram[(addr - 0x2000) as usize],
            0x3000..=0x3eff => self.vram[(addr - 0x3000) as usize],
            0x3f00..=0x3fff => self.vram[0x1f00 | (addr as usize & 0x1f)],
            _ => unreachable!(),
        }
    }

    fn write_u8(&mut self, mut addr: u16, value: u8) {
        addr &= 0x3fff;
        match addr {
            0x0000..=0x1fff => *self.mapper.chr_map_mut(addr, &mut self.chr_rom) = value,
            0x2000..=0x2fff => self.vram[(addr - 0x2000) as usize] = value,
            0x3000..=0x3eff => self.vram[(addr - 0x3000) as usize] = value,
            0x3f00..=0x3fff => self.vram[0x1f00 | (addr as usize & 0x1f)] = value,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Range;

    #[test]
    fn test_ram_mirroring() {
        let test = |range: Range<u16>| {
            let prg_rom = vec![];
            let mut mmap = CpuMemoryMap::new(0, prg_rom, vec![]);
            for i in range {
                mmap.write_u8(i, i as u8);
            }
            for i in 0..0x2000u16 {
                assert_eq!(i as u8, mmap.read_u8(i));
            }
        };
        test(0x0000..0x0800);
        test(0x0800..0x1000);
        test(0x1000..0x1800);
        test(0x1800..0x2000);
    }
}
