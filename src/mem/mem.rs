use std::ops::{Index, IndexMut};

pub trait Memory {
    fn index(&self, addr: u16) -> &u8;
    fn index_mut(&mut self, addr: u16) -> &mut u8;
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
