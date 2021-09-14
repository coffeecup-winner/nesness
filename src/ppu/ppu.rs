use std::cell::Cell;

#[derive(Debug)]
pub enum SpriteSize {
    _8x8,
    _8x16,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub struct PPU {
    latch: u8,

    // PPU control
    nametable_base_addr: u16,
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

    // PPU scroll
    scroll_x: Cell<u8>,
    scroll_y: Cell<u8>,
    wrote_scroll_x: Cell<bool>,

    // PPU address
    ppu_addr: Cell<u16>,
    wrote_ppu_addr_hi: Cell<bool>,

    // PPU data (VRAM)
    ppu_vram: Vec<u8>,

    // OAM DMA page
    oam_dma_page: u8,
}

impl PPU {
    pub fn new() -> Self {
        PPU {
            latch: 0,
            nametable_base_addr: 0x2000,
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
            scroll_x: Cell::new(0),
            scroll_y: Cell::new(0),
            wrote_scroll_x: Cell::new(false),
            ppu_addr: Cell::new(0),
            wrote_ppu_addr_hi: Cell::new(false),
            ppu_vram: vec![0; 0x4000],
            oam_dma_page: 0,
        }
    }

    pub fn read_ppuctrl(&self) -> u8 {
        // PPUCTRL is write-only
        self.latch
    }

    pub fn write_ppuctrl(&mut self, value: u8) {
        self.latch = value;
        // TODO: ignore writes for 30k cycles (?)
        self.nametable_base_addr = 0x2000 + 0x400 * (value & 0x03) as u16;
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
        self.scroll_x.set(0);
        self.scroll_y.set(0);
        self.wrote_scroll_x.set(false); // is this correct?
        self.ppu_addr.set(0);
        self.wrote_ppu_addr_hi.set(false); // is this correct?
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
        if self.wrote_scroll_x.get() {
            self.scroll_y.set(value);
        } else {
            self.scroll_x.set(value);
        }
        self.wrote_scroll_x.set(!self.wrote_scroll_x.get());
    }

    pub fn read_ppuaddr(&self) -> u8 {
        // PPUADDR is write-only
        self.latch
    }

    pub fn write_ppuaddr(&mut self, value: u8) {
        self.latch = value;
        if self.wrote_ppu_addr_hi.get() {
            self.ppu_addr
                .set(self.ppu_addr.get() & 0xff00 | (value as u16));
        } else {
            self.ppu_addr.set((value as u16) << 8);
        }
        self.wrote_ppu_addr_hi.set(!self.wrote_ppu_addr_hi.get());
    }

    pub fn read_ppudata(&self) -> u8 {
        // TODO: use internal buffer instead of the direct read
        let result = self.ppu_vram[self.get_ppu_addr()];
        self.ppu_addr
            .set(self.ppu_addr.get().wrapping_add(self.vram_incr as u16));
        result
    }

    pub fn write_ppudata(&mut self, value: u8) {
        self.latch = value;
        let addr = self.get_ppu_addr();
        self.ppu_vram[addr] = value;
        self.ppu_addr
            .set(self.ppu_addr.get().wrapping_add(self.vram_incr as u16));
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
        let addr = self.ppu_addr.get();
        (match addr & 0x3fff {
            0x0000..=0x2fff => addr,
            0x3000..=0x3eff => addr - 0x1000,
            0x3f00..=0x3fff => 0x3f00 | (addr & 0x1f),
            _ => unreachable!(),
        }) as usize
    }
}
