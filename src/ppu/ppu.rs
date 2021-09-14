use std::cell::Cell;

#[derive(Debug)]
pub enum SpriteSize {
    _8x8,
    _8x16,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub struct PPU {
    pub current_scanline: u16,
    pub current_cycle: u16, // within a scanline

    // Internal registers
    reg_v: Cell<u16>,  // Current VRAM address
    reg_t: u16,        // Temp VRAM address
    reg_x: u8,         // Fine X scroll
    reg_w: Cell<bool>, // Write toggle

    latch: u8,

    // PPU control
    sprite_pattern_table_addr: u16,
    background_pattern_table_addr: u16,
    vram_incr: u8,
    sprite_size: SpriteSize,
    is_primary: bool,
    generate_nmi_on_vblank: bool,

    // PPU mask
    is_greyscale: bool,
    show_background_leftmost_8pix: bool,
    show_sprites_leftmost_8pix: bool,
    show_background: bool,
    show_sprites: bool,
    emphasize_red: bool,
    emphasize_green: bool,
    emphasize_blue: bool,

    // PPU status
    is_sprite_overflow: bool,
    is_sprite0_hit: bool,
    is_in_vblank: Cell<bool>,

    // OAM address
    oam_addr: Cell<u8>,

    // OAM data
    oam_data: [u8; 256],

    // PPU data (VRAM)
    ppu_vram: Vec<u8>,

    // OAM DMA page
    oam_dma_page: u8,
}

impl PPU {
    pub fn new() -> Self {
        PPU {
            current_scanline: 0,
            current_cycle: 0,
            reg_v: Cell::new(0),
            reg_t: 0,
            reg_x: 0,
            reg_w: Cell::new(false),
            latch: 0,
            sprite_pattern_table_addr: 0x0000,
            background_pattern_table_addr: 0x0000,
            vram_incr: 1,
            sprite_size: SpriteSize::_8x8,
            is_primary: false,
            generate_nmi_on_vblank: false,
            is_greyscale: false,
            show_background_leftmost_8pix: false,
            show_sprites_leftmost_8pix: false,
            show_background: false,
            show_sprites: false,
            emphasize_red: false,
            emphasize_green: false,
            emphasize_blue: false,
            is_sprite_overflow: false,
            is_sprite0_hit: false,
            is_in_vblank: Cell::new(false),
            oam_addr: Cell::new(0),
            oam_data: [0; 256],
            ppu_vram: vec![0; 0x4000],
            oam_dma_page: 0,
        }
    }

    pub fn run_one(&mut self) {
        self.current_cycle += 1;
        if self.current_cycle > 340 {
            self.current_scanline += 1;
            self.current_cycle = 0;
        }
        if self.current_scanline > 261 {
            self.current_scanline = 0;
        }
    }

    pub fn read_ppuctrl(&self) -> u8 {
        // PPUCTRL is write-only
        self.latch
    }

    pub fn write_ppuctrl(&mut self, value: u8) {
        self.latch = value;
        // TODO: ignore writes for 30k cycles (?)
        self.reg_t = (self.reg_t & !0x0c00) | (((value & 0x03) as u16) << 10);
        self.vram_incr = if (value & 0x04) == 0 { 1 } else { 32 };
        self.sprite_pattern_table_addr = if (value & 0x08) == 0 { 0x0000 } else { 0x1000 };
        self.background_pattern_table_addr = if (value & 0x10) == 0 { 0x0000 } else { 0x1000 };
        self.sprite_size = if (value & 0x20) == 0 {
            SpriteSize::_8x8
        } else {
            SpriteSize::_8x16
        };
        self.is_primary = (value & 0x40) != 0;
        self.generate_nmi_on_vblank = (value & 0x80) != 0;
        // TODO: generate an NMI if in vblank
    }

    pub fn read_ppumask(&self) -> u8 {
        // PPUMASK is write-only
        self.latch
    }

    pub fn write_ppumask(&mut self, value: u8) {
        self.latch = value;
        self.is_greyscale = (value & 0x01) != 0;
        self.show_background_leftmost_8pix = (value & 0x02) != 0;
        self.show_sprites_leftmost_8pix = (value & 0x04) != 0;
        self.show_background = (value & 0x08) != 0;
        self.show_sprites = (value & 0x10) != 0;
        self.emphasize_red = (value & 0x20) != 0;
        self.emphasize_green = (value & 0x40) != 0;
        self.emphasize_blue = (value & 0x80) != 0;
    }

    pub fn read_ppustatus(&self) -> u8 {
        let mut result = self.latch & 0x1f;
        if self.is_sprite_overflow {
            result |= 0x20;
        }
        if self.is_sprite0_hit {
            result |= 0x40;
        }
        if self.is_in_vblank.get() {
            result |= 0x80;
        }
        self.is_in_vblank.set(false);
        self.reg_w.set(false);
        result
    }

    pub fn write_ppustatus(&mut self, value: u8) {
        // PPUSTATUS is read-only
        self.latch = value;
    }

    pub fn read_oamaddr(&self) -> u8 {
        // OAMADDR is write-only
        self.latch
    }

    pub fn write_oamaddr(&mut self, value: u8) {
        self.latch = value;
        self.oam_addr.set(value);
    }

    pub fn read_oamdata(&self) -> u8 {
        let result = self.oam_data[self.oam_addr.get() as usize];
        if !self.is_in_vblank.get() {
            self.oam_addr.set(self.oam_addr.get().wrapping_add(1));
        }
        result
    }

    pub fn write_oamdata(&mut self, value: u8) {
        self.latch = value;
        self.oam_data[self.oam_addr.get() as usize] = value;
        self.oam_addr.set(self.oam_addr.get().wrapping_add(1));
    }

    pub fn read_ppuscroll(&self) -> u8 {
        // PPUSCROLL is write-only
        self.latch
    }

    pub fn write_ppuscroll(&mut self, value: u8) {
        self.latch = value;
        if !self.reg_w.get() {
            self.reg_t = (self.reg_t & !0x001f) | ((value as u16) >> 3);
            self.reg_x = value & 0x7;
        } else {
            self.reg_t = (self.reg_t & !0x73e0)
                | (((value as u16) << 13) >> 1)
                | (((value as u16) >> 3) << 5);
        }
        self.reg_w.set(!self.reg_w.get());
    }

    pub fn read_ppuaddr(&self) -> u8 {
        // PPUADDR is write-only
        self.latch
    }

    pub fn write_ppuaddr(&mut self, value: u8) {
        self.latch = value;
        if !self.reg_w.get() {
            self.reg_t = (self.reg_t & 0x00ff) | (((value as u16) & 0x3f) << 8);
        } else {
            self.reg_t = (self.reg_t & 0xff00) | (value as u16);
            self.reg_v.set(self.reg_t);
        }
        self.reg_w.set(!self.reg_w.get());
    }

    pub fn read_ppudata(&self) -> u8 {
        // TODO: use internal buffer instead of the direct read
        let result = self.ppu_vram[self.get_ppu_addr()];
        self.reg_v
            .set(self.reg_v.get().wrapping_add(self.vram_incr as u16));
        result
    }

    pub fn write_ppudata(&mut self, value: u8) {
        self.latch = value;
        let addr = self.get_ppu_addr();
        self.ppu_vram[addr] = value;
        self.reg_v
            .set(self.reg_v.get().wrapping_add(self.vram_incr as u16));
    }

    pub fn read_oamdma(&self) -> u8 {
        // OAMDMA is write-only
        self.latch
    }

    pub fn write_oamdma(&mut self, value: u8) {
        self.latch = value;
        self.oam_dma_page = value;
        // TODO: start the data copy
    }

    fn get_ppu_addr(&self) -> usize {
        let addr = self.reg_v.get();
        (match addr {
            0x0000..=0x2fff => addr,
            0x3000..=0x3eff => addr - 0x1000,
            0x3f00..=0x3fff => 0x3f00 | (addr & 0x1f),
            _ => panic!("Wrong register v setting somewhere"),
        }) as usize
    }
}
