use super::mappers::{self, Mapper};
use crate::ppu::PPU;

pub trait Memory {
    fn read_u8(&self, addr: u16) -> u8;
    fn write_u8(&mut self, addr: u16, value: u8);

    fn read_u16(&self, addr: u16) -> u16 {
        let mut result = self.read_u8(addr) as u16;
        result |= (self.read_u8(addr + 1) as u16) << 8;
        result
    }

    fn write_u16(&mut self, addr: u16, value: u16) {
        self.write_u8(addr, value as u8);
        self.write_u8(addr + 1, (value >> 8) as u8);
    }

    fn update_u8<F: FnOnce(u8) -> u8>(&mut self, addr: u16, f: F) -> (u8, u8) {
        let prev = self.read_u8(addr);
        let result = f(prev);
        self.write_u8(addr, result);
        (prev, result)
    }
}

pub struct MemoryMap<'a> {
    // Main RAM - 0x0000..0x1fff
    ram: [u8; 0x0800],

    // PPU (registers) - 0x2000..0x3fff
    // TODO: This type should be renamed/moved
    ppu: PPU,

    // APU and I/O registers - 0x4000..0x4020
    apu: [u8; 32],

    // Cartridge space - 0x4020..0xffff
    mapper: Box<dyn Mapper>,
    prg_rom: &'a mut [Vec<u8>],
}

impl<'a> MemoryMap<'a> {
    #[allow(dead_code)]
    pub fn new(mapper: u8, prg_rom: &'a mut [Vec<u8>]) -> Self {
        MemoryMap {
            ram: [0x00; 0x0800],
            ppu: PPU::new(),
            apu: [0x00; 32],
            mapper: mappers::get_mapper(mapper),
            prg_rom,
        }
    }
}

impl<'a> Memory for MemoryMap<'a> {
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
                7 => self.ppu.read_ppudata(),
                _ => unreachable!(),
            },
            4 if (addr & 0xfff == 0x14) => self.ppu.read_oamdma(),
            4 if (addr as u8) < 0x20 => self.apu[addr as u8 as usize],
            _ => *self.mapper.map(addr, &self.prg_rom),
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
                7 => self.ppu.write_ppudata(value),
                _ => unreachable!(),
            },
            4 if (addr & 0xfff == 0x14) => self.ppu.write_oamdma(value),
            4 if (addr as u8) < 0x20 => self.apu[addr as u8 as usize] = value,
            _ => *self.mapper.map_mut(addr, &mut self.prg_rom) = value,
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
            let mut prg_rom = vec![];
            let mut mmap = MemoryMap::new(0, &mut prg_rom);
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

    #[test]
    fn test_ppu_registers_mirroring() {
        let test = |offset: u16| {
            let mut prg_rom = vec![];
            let mut mmap = MemoryMap::new(0, &mut prg_rom);
            for i in offset..offset + 8 {
                mmap.write_u8(i, 1 << (i & 0x7));
            }
            for i in 0x2000..0x4000u16 {
                assert_eq!(1 << (i & 0x7), mmap.read_u8(i));
            }
        };

        for offset in (0x2000..0x4000).step_by(8) {
            test(offset);
        }
    }
}
