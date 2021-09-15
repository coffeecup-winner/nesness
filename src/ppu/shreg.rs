#[derive(Debug)]
pub struct ShiftRegister8 {
    data: u8,
}

#[derive(Debug)]
pub struct ShiftRegister16 {
    data: u16,
}

#[allow(dead_code)]
impl ShiftRegister8 {
    pub fn new() -> Self {
        ShiftRegister8 { data: 0 }
    }

    pub fn shift(&mut self) -> bool {
        let result = (self.data & 0x01) == 0x01;
        self.data >>= 1;
        result
    }

    pub fn feed(&mut self, v: bool) {
        self.data &= 0x7f;
        if v {
            self.data |= 0x80;
        }
    }
}

impl ShiftRegister16 {
    pub fn new() -> Self {
        ShiftRegister16 { data: 0 }
    }

    pub fn shift(&mut self) -> bool {
        let result = (self.data & 0x0001) == 0x0001;
        self.data >>= 1;
        result
    }

    pub fn lo(&self) -> u8 {
        self.data as u8
    }

    pub fn feed(&mut self, hi: u8) {
        self.data = (self.data & 0x00ff) | ((hi as u16) << 8);
    }
}
