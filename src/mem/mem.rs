use super::mappers::{self, Mapper};

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
}

pub struct MemoryMap<'a> {
    // Main RAM - 0x0000..0x1fff
    ram: [u8; 0x0800],

    // PPU registers - 0x2000..0x3fff
    ppuctrl: u8,
    ppumask: u8,
    ppustatus: u8,
    oamaddr: u8,
    oamdata: u8,
    ppuscroll: u8,
    ppuaddr: u8,
    ppudata: u8,

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
            ppuctrl: 0,
            ppumask: 0,
            ppustatus: 0,
            oamaddr: 0,
            oamdata: 0,
            ppuscroll: 0,
            ppuaddr: 0,
            ppudata: 0,
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
                0 => self.ppuctrl,
                1 => self.ppumask,
                2 => self.ppustatus,
                3 => self.oamaddr,
                4 => self.oamdata,
                5 => self.ppuscroll,
                6 => self.ppuaddr,
                7 => self.ppudata,
                _ => unreachable!(),
            },
            4 if (addr as u8) < 0x20 => self.apu[addr as u8 as usize],
            _ => *self.mapper.map(addr, &self.prg_rom),
        }
    }

    fn write_u8(&mut self, addr: u16, value: u8) {
        match addr >> 12 {
            0 | 1 => self.ram[(addr & 0x7ff) as usize] = value,
            2 | 3 => match addr & 0x7 {
                0 => self.ppuctrl = value,
                1 => self.ppumask = value,
                2 => self.ppustatus = value,
                3 => self.oamaddr = value,
                4 => self.oamdata = value,
                5 => self.ppuscroll = value,
                6 => self.ppuaddr = value,
                7 => self.ppudata = value,
                _ => unreachable!(),
            },
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
            let mem = &mut mmap as &mut dyn Memory;
            for i in range {
                mem.write_u8(i, i as u8);
            }
            let mem = &mmap as &dyn Memory;
            for i in 0..0x2000u16 {
                assert_eq!(i as u8, mem.read_u8(i));
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
            let mem = &mut mmap as &mut dyn Memory;
            for i in offset..offset + 8 {
                mem.write_u8(i, 1 << (i & 0x7));
            }
            let mem = &mmap as &dyn Memory;
            for i in 0x2000..0x4000u16 {
                assert_eq!(1 << (i & 0x7), mem.read_u8(i));
            }
        };

        for offset in (0x2000..0x4000).step_by(8) {
            test(offset);
        }
    }
}
