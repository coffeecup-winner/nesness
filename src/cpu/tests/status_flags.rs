use super::*;

#[test]
fn test_clc() {
    let (mut cpu, mut mem) = test_cpu(&vec![vec![CLC_IMP], vec![CLC_IMP]].concat());
    cpu.flag_carry = true;

    assert_eq!(2, cpu.run_one(&mut mem));
    assert_eq!(false, cpu.flag_carry);
    assert_eq!(2, cpu.run_one(&mut mem));
    assert_eq!(false, cpu.flag_carry);
}

#[test]
fn test_cli() {
    let (mut cpu, mut mem) = test_cpu(&vec![vec![CLI_IMP], vec![CLI_IMP]].concat());
    cpu.flag_interrupt_disable = true;

    assert_eq!(2, cpu.run_one(&mut mem));
    assert_eq!(false, cpu.flag_interrupt_disable);
    assert_eq!(2, cpu.run_one(&mut mem));
    assert_eq!(false, cpu.flag_interrupt_disable);
}

#[test]
fn test_clv() {
    let (mut cpu, mut mem) = test_cpu(&vec![vec![CLV_IMP], vec![CLV_IMP]].concat());
    cpu.flag_overflow = true;

    assert_eq!(2, cpu.run_one(&mut mem));
    assert_eq!(false, cpu.flag_overflow);
    assert_eq!(2, cpu.run_one(&mut mem));
    assert_eq!(false, cpu.flag_overflow);
}

#[test]
fn test_sec() {
    let (mut cpu, mut mem) = test_cpu(&vec![vec![SEC_IMP], vec![SEC_IMP]].concat());
    cpu.flag_carry = false;

    assert_eq!(2, cpu.run_one(&mut mem));
    assert_eq!(true, cpu.flag_carry);
    assert_eq!(2, cpu.run_one(&mut mem));
    assert_eq!(true, cpu.flag_carry);
}

#[test]
fn test_sei() {
    let (mut cpu, mut mem) = test_cpu(&vec![vec![SEI_IMP], vec![SEI_IMP]].concat());
    cpu.flag_interrupt_disable = false;

    assert_eq!(2, cpu.run_one(&mut mem));
    assert_eq!(true, cpu.flag_interrupt_disable);
    assert_eq!(2, cpu.run_one(&mut mem));
    assert_eq!(true, cpu.flag_interrupt_disable);
}
