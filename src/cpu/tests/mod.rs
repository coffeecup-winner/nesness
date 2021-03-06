mod arithmetics;
mod branches;
mod inc_dec;
mod jumps;
mod load_store;
mod logical;
mod reg_transfers;
mod shifts;
mod stack;
mod status_flags;
mod system;

mod rom;

// These exports are used by submodules
use super::*;
use crate::{assert_zn, mem::Memory};
use rp2a03::opcodes::*;

impl Memory for Vec<u8> {
    fn read_u8(&self, addr: u16) -> u8 {
        self[addr as usize]
    }

    fn write_u8(&mut self, addr: u16, value: u8) {
        self[addr as usize] = value;
    }
}

#[macro_export]
macro_rules! assert_zn {
    ($cpu: ident, $z: expr, $n: expr) => {
        assert_eq!($z, $cpu.flag_zero);
        assert_eq!($n, $cpu.flag_negative);
    };
}

fn test_cpu(program: &[u8]) -> (CPU, Vec<u8>) {
    let pc = 0x1000u16;
    let mut mem = vec![0; 0x10000];
    mem.splice(
        pc as usize..(pc as usize) + program.len(),
        program.iter().cloned(),
    );
    mem[0xfffc] = pc as u8;
    mem[0xfffd] = (pc >> 8) as u8;
    let mut cpu = CPU::new();
    cpu.reset(&mem);
    (cpu, mem)
}

fn lo(v: u16) -> u8 {
    v as u8
}

fn hi(v: u16) -> u8 {
    (v >> 8) as u8
}
