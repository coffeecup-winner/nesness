use super::*;

#[test]
fn test_clc() {
    let (mut cpu, mut ram) = test_cpu(&vec![
        CLC_IMP,
        CLC_IMP,
    ]);
    cpu.flag_carry = true;

    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(false, cpu.flag_carry);
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(false, cpu.flag_carry);
}

#[test]
fn test_cli() {
    let (mut cpu, mut ram) = test_cpu(&vec![
        CLI_IMP,
        CLI_IMP,
    ]);
    cpu.flag_interrupt_disable = true;

    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(false, cpu.flag_interrupt_disable);
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(false, cpu.flag_interrupt_disable);
}

#[test]
fn test_clv() {
    let (mut cpu, mut ram) = test_cpu(&vec![
        CLV_IMP,
        CLV_IMP,
    ]);
    cpu.flag_overflow = true;

    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(false, cpu.flag_overflow);
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(false, cpu.flag_overflow);
}

#[test]
fn test_sec() {
    let (mut cpu, mut ram) = test_cpu(&vec![
        SEC_IMP,
        SEC_IMP,
    ]);
    cpu.flag_carry = false;

    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(true, cpu.flag_carry);
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(true, cpu.flag_carry);
}

#[test]
fn test_sei() {
    let (mut cpu, mut ram) = test_cpu(&vec![
        SEI_IMP,
        SEI_IMP,
    ]);
    cpu.flag_interrupt_disable = false;

    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(true, cpu.flag_interrupt_disable);
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(true, cpu.flag_interrupt_disable);
}
