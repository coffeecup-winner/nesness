pub trait Mapper {
    fn prg_map<'a>(&'a self, addr: u16, prg_rom: &'a [Vec<u8>]) -> &'a u8;
    fn prg_map_mut<'a>(&'a mut self, addr: u16, prg_rom: &'a mut [Vec<u8>]) -> &'a mut u8;
    fn chr_map<'a>(&'a self, addr: u16, chr_rom: &'a [Vec<u8>]) -> &'a u8;
    fn chr_map_mut<'a>(&'a mut self, addr: u16, chr_rom: &'a mut [Vec<u8>]) -> &'a mut u8;
}

struct Mapper0 {
    prg_ram: Vec<u8>,
}

impl Mapper for Mapper0 {
    fn prg_map<'a>(&'a self, addr: u16, prg_rom: &'a [Vec<u8>]) -> &'a u8 {
        match addr >> 12 {
            0x6 | 0x7 => &self.prg_ram[(addr & 0x1fff) as usize],
            0x8..=0xb => &prg_rom[0][(addr & 0x3fff) as usize],
            0xc..=0xf => {
                if prg_rom.len() == 2 {
                    &prg_rom[1][(addr & 0x3fff) as usize]
                } else {
                    // Mirroring bank 0
                    &prg_rom[0][(addr & 0x3fff) as usize]
                }
            }
            _ => panic!("Unmapped space access"),
        }
    }

    fn prg_map_mut<'a>(&'a mut self, addr: u16, prg_rom: &'a mut [Vec<u8>]) -> &'a mut u8 {
        match addr >> 12 {
            0x6 | 0x7 => &mut self.prg_ram[(addr & 0x1fff) as usize],
            0x8..=0xb => &mut prg_rom[0][(addr & 0x3fff) as usize],
            0xc..=0xf => {
                if prg_rom.len() == 2 {
                    &mut prg_rom[1][(addr & 0x3fff) as usize]
                } else {
                    // Mirroring bank 0
                    &mut prg_rom[0][(addr & 0x3fff) as usize]
                }
            }
            _ => panic!("Unmapped space access"),
        }
    }

    fn chr_map<'a>(&'a self, addr: u16, chr_rom: &'a [Vec<u8>]) -> &'a u8 {
        &chr_rom[0][addr as usize]
    }

    fn chr_map_mut<'a>(&'a mut self, addr: u16, chr_rom: &'a mut [Vec<u8>]) -> &'a mut u8 {
        &mut chr_rom[0][addr as usize]
    }
}

pub fn get_mapper(mapper: u8) -> Box<dyn Mapper> {
    match mapper {
        0 => Box::new(Mapper0 {
            prg_ram: vec![0xcc; 0x2000],
        }),
        _ => unimplemented!(),
    }
}
