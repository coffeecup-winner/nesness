pub trait Memory {
    fn read_u8(&self, addr: u16) -> u8;
    fn write_u8(&mut self, addr: u16, value: u8);

    fn read_u16(&self, addr: u16) -> u16 {
        let mut result = self.read_u8(addr) as u16;
        result |= (self.read_u8(addr + 1) as u16) << 8;
        result
    }

    #[allow(dead_code)]
    fn write_u16(&mut self, addr: u16, value: u16) {
        self.write_u8(addr, value as u8);
        self.write_u8(addr + 1, (value >> 8) as u8);
    }

    fn update_u8<F: FnOnce(u8) -> u8>(&mut self, addr: u16, f: F) -> (u8, u8) {
        let prev = self.read_u8(addr);
        let result = f(prev);
        self.write_u8(addr, result);
        (prev, result)
    }
}
