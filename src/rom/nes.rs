// iNES/NES 2.0 format ROM

#[derive(Debug)]
pub enum Mirroring {
    Horizontal,
    Vertical,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum TVSystem {
    NTSC,
    PAL,
    DualCompatible,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct NESHeader {
    pub is_nes2_format: bool,
    pub tv_system: TVSystem,
    pub prg_rom_banks: u8,
    pub chr_rom_banks: u8,
    pub prg_ram_banks: u8,
    pub is_vs_unisystem: bool,
    pub is_playchoice_10: bool,
    pub mapper: u8,
    pub mirroring: Mirroring,
    pub ignore_mirroring: bool,
    pub has_persistent_memory: bool,
    pub has_trainer: bool,
    pub has_prg_ram: bool,
    pub has_bus_conflicts: bool,
}

pub struct NESFile {
    pub header: NESHeader,
    pub prg_rom: Vec<Vec<u8>>,
    pub chr_rom: Vec<Vec<u8>>,
}

impl NESFile {
    pub fn load(data: &[u8]) -> Option<Self> {
        let header = Self::load_header(data)?;
        if header.mapper != 0 {
            panic!("Mapper {} is not implemented", header.mapper);
        }
        let mut idx = 16;
        if header.has_trainer {
            todo!()
        }
        const PRG_ROM_BANK_SIZE: usize = 0x4000;
        const CHR_ROM_BANK_SIZE: usize = 0x2000;
        if data.len()
            != idx
                + header.prg_rom_banks as usize * PRG_ROM_BANK_SIZE
                + header.chr_rom_banks as usize * CHR_ROM_BANK_SIZE
        {
            return None;
        }
        let mut prg_rom = vec![];
        for _ in 0..header.prg_rom_banks {
            prg_rom.push(data[idx..idx + PRG_ROM_BANK_SIZE].to_vec());
            idx += PRG_ROM_BANK_SIZE;
        }
        let mut chr_rom = vec![];
        for _ in 0..header.chr_rom_banks {
            chr_rom.push(data[idx..idx + CHR_ROM_BANK_SIZE].to_vec());
            idx += CHR_ROM_BANK_SIZE;
        }
        Some(NESFile {
            header,
            prg_rom,
            chr_rom,
        })
    }

    fn load_header(data: &[u8]) -> Option<NESHeader> {
        if data.len() < 16 {
            return None;
        }
        if &data[0..4] != b"NES\x1a" {
            return None;
        }
        let mut mapper = data[6] >> 4;
        let is_nes2_format = (data[7] & 0xc0) == 0x80;
        if is_nes2_format {
            unimplemented!()
        } else {
            // Ensure bytes 12..15 are zeroes
            if &data[12..=15] != b"\x00\x00\x00\x00" {
                return None;
            }
        }
        mapper |= data[7] & 0xf0;
        let mut prg_ram_size_in_units = data[8];
        if prg_ram_size_in_units == 0 {
            prg_ram_size_in_units = 1;
        }
        let mut tv_system = if (data[9] & 0x01) != 0 {
            TVSystem::PAL
        } else {
            TVSystem::NTSC
        };
        // Overwrite the TV system
        match data[10] & 0x03 {
            0 => {} // tv_system is already NTSC here, otherwise we shouldn't overwrite
            1 | 3 => tv_system = TVSystem::DualCompatible,
            2 => tv_system = TVSystem::PAL,
            _ => unreachable!(),
        }
        Some(NESHeader {
            is_nes2_format,
            tv_system,
            prg_rom_banks: data[4],
            chr_rom_banks: data[5],
            prg_ram_banks: prg_ram_size_in_units,
            is_vs_unisystem: (data[7] & 0x01) != 0,
            is_playchoice_10: (data[7] & 0x02) != 0,
            mapper,
            mirroring: if (data[6] & 0x01) != 0 {
                Mirroring::Vertical
            } else {
                Mirroring::Horizontal
            },
            ignore_mirroring: (data[6] & 0x08) != 0,
            has_persistent_memory: (data[6] & 0x02) != 0,
            has_trainer: (data[6] & 0x04) != 0,
            has_prg_ram: (data[10] & 0x10) == 0,
            has_bus_conflicts: (data[10] & 0x20) != 0,
        })
    }
}
