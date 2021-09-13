#[derive(Default)]
pub struct PPU {
    ppuctrl: u8,
    ppumask: u8,
    ppustatus: u8,
    oamaddr: u8,
    oamdata: u8,
    ppuscroll: u8,
    ppuaddr: u8,
    ppudata: u8,
    oamdma: u8,
}

impl PPU {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn read_ppuctrl(&self) -> u8 {
        self.ppuctrl
    }

    pub fn write_ppuctrl(&mut self, value: u8) {
        self.ppuctrl = value;
    }

    pub fn read_ppumask(&self) -> u8 {
        self.ppumask
    }

    pub fn write_ppumask(&mut self, value: u8) {
        self.ppumask = value;
    }

    pub fn read_ppustatus(&self) -> u8 {
        self.ppustatus
    }

    pub fn write_ppustatus(&mut self, value: u8) {
        self.ppustatus = value;
    }

    pub fn read_oamaddr(&self) -> u8 {
        self.oamaddr
    }

    pub fn write_oamaddr(&mut self, value: u8) {
        self.oamaddr = value;
    }

    pub fn read_oamdata(&self) -> u8 {
        self.oamdata
    }

    pub fn write_oamdata(&mut self, value: u8) {
        self.oamdata = value;
    }

    pub fn read_ppuscroll(&self) -> u8 {
        self.ppuscroll
    }

    pub fn write_ppuscroll(&mut self, value: u8) {
        self.ppuscroll = value;
    }

    pub fn read_ppuaddr(&self) -> u8 {
        self.ppuaddr
    }

    pub fn write_ppuaddr(&mut self, value: u8) {
        self.ppuaddr = value;
    }

    pub fn read_ppudata(&self) -> u8 {
        self.ppudata
    }

    pub fn write_ppudata(&mut self, value: u8) {
        self.ppudata = value;
    }

    pub fn read_oamdma(&self) -> u8 {
        self.oamdma
    }

    pub fn write_oamdma(&mut self, value: u8) {
        self.oamdma = value;
    }
}
