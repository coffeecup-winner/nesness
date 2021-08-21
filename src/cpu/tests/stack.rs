use super::*;
use crate::cpu::rp2a03::flags;

#[test]
fn test_tsx() {
    // Values/flags
    for i in 0..=0xff {
        let (mut cpu, mut ram) = test_cpu(&vec![TSX_IMP]);
        cpu.reg_s = i;
        cpu.reg_x = !i;
        assert_eq!(2, cpu.run_one(&mut ram));
        assert_eq!(i, cpu.reg_x);
        assert_zn!(cpu, i == 0, i >= 0x80);
    }
}

#[test]
fn test_txs() {
    // Values
    for i in 0..=0xff {
        let (mut cpu, mut ram) = test_cpu(&vec![TXS_IMP]);
        cpu.reg_x = i;
        cpu.reg_s = !i;
        assert_eq!(2, cpu.run_one(&mut ram));
        assert_eq!(i, cpu.reg_s);
    }
}

#[test]
fn test_pha() {
    // Values
    for i in 0..=0xff {
        let (mut cpu, mut ram) = test_cpu(&vec![PHA_IMP]);
        cpu.reg_a = i;
        cpu.reg_s = i;
        ram[0x100 + i as usize] = !i;
        assert_eq!(3, cpu.run_one(&mut ram));
        assert_eq!(ram[0x100 + i as usize], i);
    }
}

#[test]
fn test_php() {
    // Values
    for carry in 0..=1 {
        for zero in 0..=1 {
            for interrupt_disable in 0..=1 {
                for break_ in 0..=1 {
                    for overflow in 0..=1 {
                        for negative in 0..=1 {
                            let (mut cpu, mut ram) = test_cpu(&vec![PHP_IMP]);
                            cpu.flag_carry = carry != 0;
                            cpu.flag_zero = zero != 0;
                            cpu.flag_interrupt_disable = interrupt_disable != 0;
                            cpu.flag_break = break_ != 0;
                            cpu.flag_overflow = overflow != 0;
                            cpu.flag_negative = negative != 0;
                            assert_eq!(3, cpu.run_one(&mut ram));
                            let p = ram[0x100 + cpu.reg_s as usize + 1];
                            assert_eq!(carry != 0, (p & flags::C) != 0);
                            assert_eq!(zero != 0, (p & flags::Z) != 0);
                            assert_eq!(interrupt_disable != 0, (p & flags::I) != 0);
                            assert_eq!(0, p & 0x08);
                            assert_eq!(break_ != 0, (p & flags::B) != 0);
                            assert_ne!(0, p & 0x20);
                            assert_eq!(overflow != 0, (p & flags::V) != 0);
                            assert_eq!(negative != 0, (p & flags::N) != 0);
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn test_pla() {
    // Values/flags
    let (mut cpu, mut ram) = test_cpu(&vec![PLA_IMP; 256]);
    for i in (0..=0xffu8).rev() {
        ram[0x100 + i as usize] = i.wrapping_sub(128);
    }
    cpu.reg_s = 0xff;
    for i in 0..=0xff {
        cpu.reg_a = !i;
        let expected = i.wrapping_sub(128);
        assert_eq!(4, cpu.run_one(&mut ram));
        assert_eq!(expected, cpu.reg_a);
        assert_zn!(cpu, expected == 0, (expected & 0x80) != 0);
    }
}

#[test]
fn test_plp() {
    // Values
    for carry in 0..=1 {
        for zero in 0..=1 {
            for interrupt_disable in 0..=1 {
                for break_ in 0..=1 {
                    for overflow in 0..=1 {
                        for negative in 0..=1 {
                            let (mut cpu, mut ram) = test_cpu(&vec![PLP_IMP]);
                            let mut p = 0x20;
                            p |= if carry != 0 { flags::C } else { 0 };
                            p |= if zero != 0 { flags::Z } else { 0 };
                            p |= if interrupt_disable != 0 { flags::I } else { 0 };
                            p |= if break_ != 0 { flags::B } else { 0 };
                            p |= if overflow != 0 { flags::V } else { 0 };
                            p |= if negative != 0 { flags::N } else { 0 };
                            ram[0x100] = p;
                            cpu.reg_s = 0xff;
                            assert_eq!(4, cpu.run_one(&mut ram));
                            assert_eq!(carry != 0, cpu.flag_carry);
                            assert_eq!(zero != 0, cpu.flag_zero);
                            assert_eq!(interrupt_disable != 0, cpu.flag_interrupt_disable);
                            assert_eq!(break_ != 0, cpu.flag_break);
                            assert_eq!(overflow != 0, cpu.flag_overflow);
                            assert_eq!(negative != 0, cpu.flag_negative);
                        }
                    }
                }
            }
        }
    }
}
