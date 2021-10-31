use std::cell::Cell;

use crate::mem::Memory;

use super::shreg::{ShiftRegister16, ShiftRegister8};

#[derive(Debug)]
pub enum SpriteSize {
    _8x8,
    _8x16,
}

#[derive(Debug, Clone, Copy)]
struct EvaluatedSprite {
    is_valid: bool,
    is_zero_sprite: bool,
    x: u8,
    y: u8,
    tile_index: u8,
    attributes: u8,
    tile_lo: ShiftRegister8,
    tile_hi: ShiftRegister8,
}

impl EvaluatedSprite {
    pub fn new() -> Self {
        EvaluatedSprite {
            is_valid: false,
            is_zero_sprite: false,
            x: 0xff,
            y: 0xff,
            tile_index: 0xff,
            attributes: 0xff,
            tile_lo: ShiftRegister8::new(),
            tile_hi: ShiftRegister8::new(),
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub struct PPU {
    pub current_scanline: u16,
    pub current_cycle: u16, // within a scanline
    pub frame_buffer: Vec<u8>,
    pub is_cpu_interrupt_requested: bool,
    pub oam_dma_page: Option<u8>,
    is_odd_frame: bool,

    // Internal registers
    reg_v: Cell<u16>,  // Current VRAM address
    reg_t: u16,        // Temp VRAM address
    reg_x: u8,         // Fine X scroll
    reg_w: Cell<bool>, // Write toggle

    // Internal temp storage for data fetching
    current_tile_idx: u8,
    current_tile_attr: u8,
    current_tile_pattern_lo: u8,
    current_tile_pattern_hi: u8,
    shreg_bg_tile_lo: ShiftRegister16,
    shreg_bg_tile_hi: ShiftRegister16,

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
    oam_evaluated: [EvaluatedSprite; 8],
}

impl PPU {
    pub fn new() -> Self {
        PPU {
            current_scanline: 0,
            current_cycle: 0,
            frame_buffer: vec![0; 256 * 240],
            is_cpu_interrupt_requested: false,
            oam_dma_page: None,
            is_odd_frame: false,
            reg_v: Cell::new(0),
            reg_t: 0,
            reg_x: 0,
            reg_w: Cell::new(false),
            current_tile_idx: 0,
            current_tile_attr: 0,
            current_tile_pattern_lo: 0,
            current_tile_pattern_hi: 0,
            shreg_bg_tile_lo: ShiftRegister16::new(),
            shreg_bg_tile_hi: ShiftRegister16::new(),
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
            oam_evaluated: [EvaluatedSprite::new(); 8],
        }
    }

    pub fn run_one<M: Memory>(&mut self, mem: &M) {
        if self.show_background || self.show_sprites {
            // Data fetches and address increments
            if self.current_scanline < 240 || self.current_scanline == 261 {
                // Tile fetch
                // NOTE: unused nametable fetches are not implemented
                let c = self.current_cycle;
                if (c > 0 && c <= 256) || (c > 320 && c <= 336) {
                    match c % 8 {
                        1 => {
                            // Do nothing, the fetch happens on the next cycle
                            // This might have to be changed to be more precise
                        }
                        2 => {
                            let tile_addr = 0x2000 | (self.reg_v.get() & 0x0fff);
                            self.current_tile_idx = mem.read_u8(tile_addr);
                        }
                        3 => {
                            // Do nothing, the fetch happens on the next cycle
                            // This might have to be changed to be more precise
                        }
                        4 => {
                            let v = self.reg_v.get();
                            let attr_addr =
                                0x23c0 | (v & 0x0c00) | ((v >> 4) & 0x38) | ((v >> 2) & 0x07);
                            self.current_tile_attr = mem.read_u8(attr_addr);
                        }
                        5 => {
                            // Do nothing, the fetch happens on the next cycle
                            // This might have to be changed to be more precise
                        }
                        6 => {
                            let addr = self.background_pattern_table_addr
                                + self.current_tile_idx as u16 * 16
                                + self.current_scanline % 8;
                            self.current_tile_pattern_lo = mem.read_u8(addr);
                            self.shreg_bg_tile_lo.feed(self.current_tile_pattern_lo);
                        }
                        7 => {
                            // Do nothing, the fetch happens on the next cycle
                            // This might have to be changed to be more precise
                        }
                        0 => {
                            let addr = self.background_pattern_table_addr
                                + self.current_tile_idx as u16 * 16
                                + self.current_scanline % 8
                                + 8;
                            self.current_tile_pattern_hi = mem.read_u8(addr);
                            self.shreg_bg_tile_hi.feed(self.current_tile_pattern_hi);
                        }
                        _ => unreachable!(),
                    }
                }

                // Coordinate increment
                if ((c > 0 && c <= 248) || (c > 320 && c <= 336)) && c % 8 == 0 {
                    // Increment coarse X
                    let v = self.reg_v.get();
                    if (v & 0x001f) == 0x001f {
                        // Switch the nametable
                        self.reg_v.set((v & !0x001f) ^ 0x0400);
                    } else {
                        self.reg_v.set(v + 1);
                    }
                } else if c == 256 {
                    // Increment fine Y
                    let mut v = self.reg_v.get();
                    if (v & 0x7000) != 0x7000 {
                        self.reg_v.set(v + 0x1000);
                    } else {
                        // Increment coarse Y
                        v &= !0x7000;
                        let mut y = (v & 0x03e0) >> 5;
                        if y == 29 {
                            // Switch the nametable
                            y = 0;
                            v ^= 0x0800;
                        } else if y == 31 {
                            // Do not switch the nametable
                            y = 0;
                        } else {
                            y += 1;
                        }
                        self.reg_v.set((v & !0x03e0) | (y << 5));
                    }
                } else if c == 257 {
                    // Horizontal position copy from t to v
                    self.reg_v
                        .set((self.reg_v.get() & !0x041f) | (self.reg_t & 0x041f));
                } else if (c >= 280 && c <= 304) && self.current_scanline == 261 {
                    // Vertical position copy from t to v
                    self.reg_v
                        .set((self.reg_v.get() & !0x7be0) | (self.reg_t & 0x7be0));
                }

                // Sprite evaluation
                let next_y = if self.current_scanline == 239 { 0 } else { self.current_scanline + 1} as u8;
                if c == 1 {
                    if self.current_scanline != 261 {
                        for i in 0..self.oam_evaluated.len() {
                            self.oam_evaluated[i] = EvaluatedSprite::new();
                        }
                    }
                } else if c == 65 {
                    if self.current_scanline != 261 {
                        // Not cycle accurate
                        let mut idx_free = 0;
                        for i in 0..64 {
                            let y = self.oam_data[i * 4];
                            self.oam_evaluated[idx_free].y = y;
                            if (y..y.wrapping_add(8)).contains(&next_y) {
                                self.oam_evaluated[idx_free].tile_index = self.oam_data[i * 4 + 1];
                                self.oam_evaluated[idx_free].attributes = self.oam_data[i * 4 + 2];
                                self.oam_evaluated[idx_free].x = self.oam_data[i * 4 + 3];
                                self.oam_evaluated[idx_free].is_valid = true;
                                self.oam_evaluated[idx_free].is_zero_sprite = i == 0;
                                idx_free += 1;
                                if idx_free == self.oam_evaluated.len() {
                                    break;
                                }
                            }
                        }
                        // TODO: set sprite overflow flag
                    }
                } else if c == 257 {
                    // Not cycle accurate
                    for i in 0..self.oam_evaluated.len() {
                        if !self.oam_evaluated[i].is_valid {
                            break;
                        }
                        let idx = self.oam_evaluated[i].tile_index;
                        let addr = self.sprite_pattern_table_addr + idx as u16 * 8 + (next_y - self.oam_evaluated[i].y) as u16;
                        self.oam_evaluated[i].tile_lo.load(mem.read_u8(addr));
                        self.oam_evaluated[i].tile_hi.load(mem.read_u8(addr + 8));
                    }
                }
            }

            // Rendering
            if self.current_scanline < 240 && (4..260).contains(&self.current_cycle) {
                // PPU starts rendering a scanline from cycle 4
                let x = self.current_cycle - 4;
                let y = self.current_scanline;

                let bg_idx = if !self.show_background || (!self.show_background_leftmost_8pix && x < 8) {
                    None
                } else {
                    let bit0 = (self.shreg_bg_tile_lo.hi() >> (7 - self.reg_x)) & 0x01;
                    let bit1 = (self.shreg_bg_tile_hi.hi() >> (7 - self.reg_x)) & 0x01;
                    if bit0 == 0 && bit1 == 0 {
                        None
                    } else {
                        // TODO: shift register
                        let attr = self.current_tile_attr;
                        let shift = match (x / 32 > 15, y / 32 > 15) {
                            (true, true) => 6,
                            (true, false) => 2,
                            (false, true) => 4,
                            (false, false) => 0,
                        };
                        Some((((attr >> shift) & 0x03) << 2) | (bit1 << 1) | bit0)
                    }
                };

                let sprite_idx = if !self.show_sprites || (!self.show_sprites_leftmost_8pix && x < 8) {
                    None
                } else {
                    let mut result = None;
                    for s in &self.oam_evaluated {
                        if !s.is_valid || s.x != 0 {
                            break;
                        }

                        let bit0 = if s.tile_lo.get_u1() { 1 } else { 0 };
                        let bit1 = if s.tile_hi.get_u1() { 1 } else { 0 };
                        if bit0 == 0 && bit1 == 0 {
                            continue;
                        }

                        let attr = s.attributes;
                        let idx = ((attr & 0x03) << 2) | (bit1 << 1) | bit0;

                        result = Some((idx, (s.attributes & 0x20) != 0));
                        break;
                        // TODO: priority
                    }
                    result
                };

                let idx = match (bg_idx, sprite_idx) {
                    (None, None) => 0,
                    (None, Some((idx, _))) => idx,
                    (_, Some((idx, false))) => idx,
                    (Some(idx), None) => idx,
                    (Some(idx), Some((_, true))) => idx,
                };
                self.frame_buffer[(y * 256 + x) as usize] = mem.read_u8(0x3f00 + idx as u16);

                // Sprite 0 hit
                if bg_idx.is_some() && sprite_idx.is_some() && x < 255 {
                    self.is_sprite0_hit = true;
                }
            }
        }

        // vblank state change
        if self.current_scanline == 241 && self.current_cycle == 1 {
            self.is_in_vblank.set(true);
            if self.generate_nmi_on_vblank {
                self.is_cpu_interrupt_requested = true;
            }
        } else if self.current_scanline == 261 && self.current_cycle == 1 {
            self.is_in_vblank.set(false);
            self.is_sprite0_hit = false;
            self.is_sprite_overflow = false;
        }

        // Shift all shift registers
        if (2..258).contains(&self.current_cycle) {
            self.shreg_bg_tile_lo.shift();
            self.shreg_bg_tile_hi.shift();
            for s in &mut self.oam_evaluated {
                if !s.is_valid {
                    break;
                }
                if s.x != 0 {
                    s.x -= 1;
                    continue;
                }
                s.tile_lo.shift();
                s.tile_hi.shift();
            }
        }

        // Increment the cycle/scanline counters
        self.current_cycle += 1;
        // Skip cycle 340 on the last scanline of the odd frame
        if self.is_odd_frame && self.current_cycle == 340 && self.current_scanline == 261 {
            self.current_cycle += 1;
        }
        if self.current_cycle > 340 {
            self.current_scanline += 1;
            self.current_cycle = 0;
        }
        if self.current_scanline > 261 {
            self.current_scanline = 0;
        }
        self.is_odd_frame = !self.is_odd_frame;
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
        if self.generate_nmi_on_vblank && self.is_in_vblank.get() {
            self.is_cpu_interrupt_requested = true;
        }
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
        // During secondary OAM clear, OAM data is wired to be 0xff
        if (0..240).contains(&self.current_scanline) && (1..=64).contains(&self.current_cycle) {
            return 0xff;
        }
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

    pub fn read_ppudata<M: Memory>(&self, mem: &M) -> u8 {
        // TODO: use internal buffer instead of the direct read
        let result = mem.read_u8(self.reg_v.get());
        self.reg_v
            .set(self.reg_v.get().wrapping_add(self.vram_incr as u16));
        result
    }

    pub fn write_ppudata<M: Memory>(&mut self, mem: &mut M, value: u8) {
        self.latch = value;
        mem.write_u8(self.reg_v.get(), value);
        self.reg_v
            .set(self.reg_v.get().wrapping_add(self.vram_incr as u16));
    }

    pub fn read_oamdma(&self) -> u8 {
        // OAMDMA is write-only
        self.latch
    }

    pub fn write_oamdma(&mut self, value: u8) {
        self.latch = value;
        self.oam_dma_page = Some(value);
    }

    pub fn write_oamdata_raw(&mut self, index: u8, value: u8) {
        self.oam_data[self.oam_addr.get().wrapping_add(index) as usize] = value;
    }

    #[cfg(debug_assertions)]
    pub fn dump<M: Memory>(&self, mem: &M) {
        let mut vram = vec![0; 0x4000];
        for addr in 0..vram.len() {
            vram[addr] = mem.read_u8(addr as u16);
        }
        std::fs::write("vram_dump.bin", &vram).expect("Failed to dump PPU VRAM");
        let mut img = bmp::Image::new(256, 128);
        for tile_idx in 0..512 {
            let tile_base_addr = tile_idx * 16;
            let img_tile_x = if tile_idx < 256 {
                (tile_idx % 16) * 8
            } else {
                128 + (tile_idx % 16) * 8
            };
            let img_tile_y = if tile_idx < 256 {
                (tile_idx / 16) * 8
            } else {
                ((tile_idx / 16) - 16) * 8
            };
            for y in 0..8 {
                let plane0 = mem.read_u8(tile_base_addr + y);
                let plane1 = mem.read_u8(tile_base_addr + y + 8);
                for x in 0..8 {
                    let mut idx = 0;
                    if plane0 & (1 << (7 - x)) != 0 {
                        idx += 1;
                    }
                    if plane1 & (1 << (7 - x)) != 0 {
                        idx += 2;
                    }
                    let pixel = match idx {
                        0 => bmp::Pixel::new(0, 0, 0),
                        1 => bmp::Pixel::new(255, 0, 0),
                        2 => bmp::Pixel::new(0, 255, 0),
                        3 => bmp::Pixel::new(0, 0, 255),
                        _ => unreachable!(),
                    };
                    img.set_pixel((img_tile_x + x) as u32, (img_tile_y + y) as u32, pixel);
                }
            }
        }
        img.save("palette.bmp").expect("Failed to dump the palette");
    }
}

#[cfg(test)]
mod tests {
    use crate::{cpu::CPU, nes::mmap::CpuMemoryMap, cpu::rp2a03::opcodes::*};

    #[test]
    fn test_ppu_vram_access() {
        let mut cpu = CPU::new();
        let mut mmap = CpuMemoryMap::new(0, vec![vec![0; 0x10000]], vec![vec![0; 0x2000]]);
        cpu.reset(&mut mmap);

        for addr in 0..0x4000u16 {
            let code = vec![
                LDX_IMM, ((addr >> 8) as u8),
                LDY_IMM, (addr as u8),

                STX_ABS, 0x06, 0x20,
                STY_ABS, 0x06, 0x20,

                // Write 0xcc
                LDA_IMM, 0xcc,
                STA_ABS, 0x07, 0x20,

                STX_ABS, 0x06, 0x20,
                STY_ABS, 0x06, 0x20,

                LDA_ABS, 0x07, 0x20,
                STA_ZPG, 0x00,
            ];
            for i in 0..code.len() {
                mmap.ram[0x200 + i] = code[i];
            }

            cpu.pc = 0x0200;
            mmap.ram[0x0000] = 0x00;
            for _ in 0..code.len() {
                cpu.run_one(&mut mmap);
            }
            assert_eq!(0xcc, mmap.ram[0x0000]);
            assert_eq!(addr.wrapping_add(1), mmap.ppu.reg_v.get());
        }
    }
}
