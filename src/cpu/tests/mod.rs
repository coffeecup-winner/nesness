mod load_store;
mod status_flags;

// These exports are used by submodules
use super::*;
use rp2a03::opcodes::*;
use crate::assert_zn;

#[macro_export]
macro_rules! assert_zn {
    ($cpu: ident, $z: expr, $n: expr) => {
        assert_eq!($z, $cpu.flag_zero);
        assert_eq!($n, $cpu.flag_negative);
    };
}

fn test_cpu(program: &[u8]) -> (CPU, Vec<u8>) {
    let pc = 0x1000u16;
    let mut ram = vec![0; 0x10000];
    ram.splice(
        pc as usize..(pc as usize) + program.len(),
        program.iter().cloned(),
    );
    ram[0xfffc] = pc as u8;
    ram[0xfffd] = (pc >> 8) as u8;
    let mut cpu = CPU::new();
    cpu.reset(&ram);
    (cpu, ram)
}

fn lo(v: u16) -> u8 {
    v as u8
}

fn hi(v: u16) -> u8 {
    (v >> 8) as u8
}
