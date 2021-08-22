use std::ops::{Index, IndexMut};

pub trait Memory {
    fn index(&self, addr: u16) -> &u8;
    fn index_mut(&mut self, addr: u16) -> &mut u8;

    fn read_u8(&self, addr: u16) -> u8 {
        *self.index(addr)
    }

    fn write_u8(&mut self, addr: u16, value: u8) {
        *self.index_mut(addr) = value;
    }

    fn read_u16(&self, addr: u16) -> u16 {
        let mut result = self.read_u8(addr) as u16;
        result |= (self.read_u8(addr + 1) as u16) << 8;
        result
    }

    fn write_u16(&mut self, addr: u16, value: u16) {
        self.write_u8(addr, value as u8);
        self.write_u8(addr + 1, (value >> 8) as u8);
    }
}

// Indices are implemented for memory and not the other way around
// to allow implementing this for std types such as Vec<u8>

impl<'a> Index<u16> for dyn Memory + 'a {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        Memory::index(self, index)
    }
}

impl<'a> IndexMut<u16> for dyn Memory + 'a {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        Memory::index_mut(self, index)
    }
}
