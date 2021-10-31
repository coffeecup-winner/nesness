#[derive(Debug, Clone, Copy)]
pub struct ShiftRegister8 {
    data: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct ShiftRegister16 {
    data: u16,
}

#[allow(dead_code)]
impl ShiftRegister8 {
    pub fn new() -> Self {
        ShiftRegister8 { data: 0 }
    }

    pub fn get_u1(&self) -> bool {
        (self.data & 0x80) != 0
    }

    pub fn shift(&mut self) {
        self.data <<= 1;
    }

    pub fn load(&mut self, v: u8) {
        self.data = v;
    }
}

impl ShiftRegister16 {
    pub fn new() -> Self {
        ShiftRegister16 { data: 0 }
    }

    pub fn shift(&mut self) {
        self.data <<= 1;
    }

    pub fn hi(&self) -> u8 {
        (self.data >> 8) as u8
    }

    pub fn feed(&mut self, lo: u8) {
        self.data = (self.data & 0xff00) | (lo as u16);
    }
}
