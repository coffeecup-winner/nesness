use std::ops::{Index, IndexMut};

pub trait Memory {
    fn index(&self, addr: u16) -> &u8;
    fn index_mut(&mut self, addr: u16) -> &mut u8;

    fn read_u8(&self, addr: u16) -> u8 {
        *self.index(addr)
    }

    fn write_u8(&mut self, addr: u16, value: u8) {
        *self.index_mut(addr) = value;
    }

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

// Indices are implemented for memory and not the other way around
// to allow implementing this for std types such as Vec<u8>

impl<'a> Index<u16> for dyn Memory + 'a {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        Memory::index(self, index)
    }
}

impl<'a> IndexMut<u16> for dyn Memory + 'a {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        Memory::index_mut(self, index)
    }
}

pub struct MemoryMap {
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

    // APU and I/O registers - 0x4000..0x4017
    // TODO

    // Unused/disabled - 0x4018..0x401f

    // Cartridge space - 0x4020..0xffff
    cartridge: Vec<u8>,
}

impl MemoryMap {
    #[allow(dead_code)]
    pub fn new() -> Self {
        MemoryMap {
            ram: [0; 0x0800],
            ppuctrl: 0,
            ppumask: 0,
            ppustatus: 0,
            oamaddr: 0,
            oamdata: 0,
            ppuscroll: 0,
            ppuaddr: 0,
            ppudata: 0,
            cartridge: vec![0; 0xbfe0],
        }
    }
}

impl Memory for MemoryMap {
    fn index(&self, addr: u16) -> &u8 {
        match addr >> 12 {
            0 | 1 => &self.ram[(addr & 0x7ff) as usize],
            2 | 3 => match addr & 0x7 {
                0 => &self.ppuctrl,
                1 => &self.ppumask,
                2 => &self.ppustatus,
                3 => &self.oamaddr,
                4 => &self.oamdata,
                5 => &self.ppuscroll,
                6 => &self.ppuaddr,
                7 => &self.ppudata,
                _ => unreachable!(),
            },
            4 if (addr as u8) < 0x20 => {
                todo!()
            }
            _ => &self.cartridge[(addr - 0x4020) as usize],
        }
    }

    fn index_mut(&mut self, addr: u16) -> &mut u8 {
        match addr >> 12 {
            0 | 1 => &mut self.ram[(addr & 0x7ff) as usize],
            2 | 3 => match addr & 0x7 {
                0 => &mut self.ppuctrl,
                1 => &mut self.ppumask,
                2 => &mut self.ppustatus,
                3 => &mut self.oamaddr,
                4 => &mut self.oamdata,
                5 => &mut self.ppuscroll,
                6 => &mut self.ppuaddr,
                7 => &mut self.ppudata,
                _ => unreachable!(),
            },
            4 if (addr as u8) < 0x20 => {
                todo!()
            }
            _ => &mut self.cartridge[(addr - 0x4020) as usize],
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
            let mut mmap = MemoryMap::new();
            let mem = &mut mmap as &mut dyn Memory;
            for i in range {
                mem[i] = i as u8;
            }
            let mem = &mmap as &dyn Memory;
            for i in 0..0x2000u16 {
                assert_eq!(i as u8, mem[i]);
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
            let mut mmap = MemoryMap::new();
            let mem = &mut mmap as &mut dyn Memory;
            for i in offset..offset + 8 {
                mem[i] = 1 << (i & 0x7);
            }
            let mem = &mmap as &dyn Memory;
            for i in 0x2000..0x4000u16 {
                assert_eq!(1 << (i & 0x7), mem[i]);
            }
        };

        for offset in (0x2000..0x4000).step_by(8) {
            test(offset);
        }
    }
}
