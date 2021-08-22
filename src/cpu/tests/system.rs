use super::*;
use crate::cpu::rp2a03::flags;

#[test]
fn test_brk() {
    for addr in 0..=0xffff {
        let (mut cpu, mut mem) = test_cpu(&vec![BRK_IMP]);
        mem[0xfffe] = lo(addr);
        mem[0xffff] = hi(addr);
        cpu.reg_s = 0x02;
        cpu.flag_carry = ((addr as u8) & flags::C) != 0;
        cpu.flag_zero = ((addr as u8) & flags::Z) != 0;
        cpu.flag_interrupt_disable = ((addr as u8) & flags::I) != 0;
        cpu.flag_break = ((addr as u8) & flags::B) != 0;
        cpu.flag_overflow = ((addr as u8) & flags::V) != 0;
        cpu.flag_negative = ((addr as u8) & flags::N) != 0;
        let return_addr = cpu.pc + 1;
        assert_eq!(7, cpu.run_one(&mut mem));
        assert_eq!(addr, cpu.pc);
        assert_eq!(lo(return_addr), mem[0x101]);
        assert_eq!(hi(return_addr), mem[0x102]);
        let p = mem[0x100];
        assert_eq!(((addr as u8) & flags::C) != 0, (p & flags::C) != 0);
        assert_eq!(((addr as u8) & flags::Z) != 0, (p & flags::Z) != 0);
        assert_eq!(((addr as u8) & flags::I) != 0, (p & flags::I) != 0);
        assert_eq!(0, p & 0x08);
        assert_eq!(((addr as u8) & flags::B) != 0, (p & flags::B) != 0);
        assert_ne!(0, p & 0x20);
        assert_eq!(((addr as u8) & flags::V) != 0, (p & flags::V) != 0);
        assert_eq!(((addr as u8) & flags::N) != 0, (p & flags::N) != 0);
    }
}

#[test]
fn test_nop() {
    let (mut cpu, mut mem) = test_cpu(&vec![NOP_IMP]);
    let copy = cpu.clone();
    assert_eq!(2, cpu.run_one(&mut mem));
    assert_eq!(copy.pc + 1, cpu.pc);
    assert_eq!(copy.reg_a, cpu.reg_a);
    assert_eq!(copy.reg_x, cpu.reg_x);
    assert_eq!(copy.reg_y, cpu.reg_y);
    assert_eq!(copy.reg_s, cpu.reg_s);
    assert_eq!(copy.flag_carry, cpu.flag_carry);
    assert_eq!(copy.flag_zero, cpu.flag_zero);
    assert_eq!(copy.flag_interrupt_disable, cpu.flag_interrupt_disable);
    assert_eq!(copy.flag_break, cpu.flag_break);
    assert_eq!(copy.flag_overflow, cpu.flag_overflow);
    assert_eq!(copy.flag_negative, cpu.flag_negative);
}

#[test]
fn test_rti() {
    for addr in 0..=0xffff {
        let (mut cpu, mut mem) = test_cpu(&vec![RTI_IMP]);
        let mut p = 0x20;
        p |= if ((addr as u8) & flags::C) != 0 { flags::C } else { 0 };
        p |= if ((addr as u8) & flags::Z) != 0 { flags::Z } else { 0 };
        p |= if ((addr as u8) & flags::I) != 0 { flags::I } else { 0 };
        p |= if ((addr as u8) & flags::B) != 0 { flags::B } else { 0 };
        p |= if ((addr as u8) & flags::V) != 0 { flags::V } else { 0 };
        p |= if ((addr as u8) & flags::N) != 0 { flags::N } else { 0 };
        mem[0x100] = p;
        mem[0x101] = lo(addr);
        mem[0x102] = hi(addr);
        cpu.reg_s = 0xff;
        assert_eq!(6, cpu.run_one(&mut mem));
        assert_eq!(addr, cpu.pc);
        assert_eq!(((addr as u8) & flags::C) != 0, cpu.flag_carry);
        assert_eq!(((addr as u8) & flags::Z) != 0, cpu.flag_zero);
        assert_eq!(((addr as u8) & flags::I) != 0, cpu.flag_interrupt_disable);
        assert_eq!(((addr as u8) & flags::B) != 0, cpu.flag_break);
        assert_eq!(((addr as u8) & flags::V) != 0, cpu.flag_overflow);
        assert_eq!(((addr as u8) & flags::N) != 0, cpu.flag_negative);
    }
}
