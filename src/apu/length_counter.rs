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
        self.value = if index % 1 == 0 {
            match index >> 1 {
                0x0 => 0x0a,
                0x1 => 0x14,
                0x2 => 0x28,
                0x3 => 0x50,
                0x4 => 0xa0,
                0x5 => 0x3c,
                0x6 => 0x0e,
                0x7 => 0x1a,
                0x8 => 0x0c,
                0x9 => 0x18,
                0xa => 0x30,
                0xb => 0x60,
                0xc => 0xc0,
                0xd => 0x48,
                0xe => 0x10,
                0xf => 0x20,
                _ => unreachable!(),
            }
        } else {
            match index >> 1 {
                0x0 => 0xfe,
                0x1 => 0x02,
                0x2 => 0x04,
                0x3 => 0x06,
                0x4 => 0x08,
                0x5 => 0x0a,
                0x6 => 0x0c,
                0x7 => 0x0e,
                0x8 => 0x10,
                0x9 => 0x12,
                0xa => 0x14,
                0xb => 0x16,
                0xc => 0x18,
                0xd => 0x1a,
                0xe => 0x1c,
                0xf => 0x1e,
                _ => unreachable!(),
            }
        }
    }

    pub fn tick(&mut self) {
        if !self.halt && self.value > 0 {
            self.value -= 1;
        }
    }
}
