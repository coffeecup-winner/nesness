pub struct LengthCounter {
    halt: bool,
    value: u8,
}

impl LengthCounter {
    pub fn new() -> Self {
        LengthCounter {
            halt: true,
            value: 0,
        }
    }

    pub fn value_is_zero(&self) -> bool {
        self.value == 0
    }

    pub fn set_halt(&mut self, value: bool) {
        self.halt = value;
    }

    pub fn set_index(&mut self, index: u8) {
        self.value = match index {
            0x00 => 0x0a,
            0x01 => 0xfe,
            0x02 => 0x14,
            0x03 => 0x02,
            0x04 => 0x28,
            0x05 => 0x04,
            0x06 => 0x50,
            0x07 => 0x06,
            0x08 => 0xa0,
            0x09 => 0x08,
            0x0a => 0x3c,
            0x0b => 0x0a,
            0x0c => 0x0e,
            0x0d => 0x0c,
            0x0e => 0x1a,
            0x0f => 0x0e,
            0x10 => 0x0c,
            0x11 => 0x10,
            0x12 => 0x18,
            0x13 => 0x12,
            0x14 => 0x30,
            0x15 => 0x14,
            0x16 => 0x60,
            0x17 => 0x16,
            0x18 => 0xc0,
            0x19 => 0x18,
            0x1a => 0x48,
            0x1b => 0x1a,
            0x1c => 0x10,
            0x1d => 0x1c,
            0x1e => 0x20,
            0x1f => 0x1e,
            _ => unreachable!(),
        }
    }

    pub fn tick(&mut self) {
        if !self.halt && self.value > 0 {
            self.value -= 1;
        }
    }
}
