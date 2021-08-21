use super::*;

#[test]
fn test_tax() {
    // Values/flags
    for i in 0..=0xff {
        let (mut cpu, mut ram) = test_cpu(&vec![TAX_IMP, i]);
        cpu.reg_a = i;
        cpu.reg_x = !i;
        cpu.run_one(&mut ram);
        assert_eq!(i, cpu.reg_x);
        assert_zn!(cpu, i == 0, i >= 0x80);
    }
}

#[test]
fn test_tay() {
    // Values/flags
    for i in 0..=0xff {
        let (mut cpu, mut ram) = test_cpu(&vec![TAY_IMP, i]);
        cpu.reg_a = i;
        cpu.reg_y = !i;
        cpu.run_one(&mut ram);
        assert_eq!(i, cpu.reg_y);
        assert_zn!(cpu, i == 0, i >= 0x80);
    }
}

#[test]
fn test_txa() {
    // Values/flags
    for i in 0..=0xff {
        let (mut cpu, mut ram) = test_cpu(&vec![TXA_IMP, i]);
        cpu.reg_x = i;
        cpu.reg_a = !i;
        cpu.run_one(&mut ram);
        assert_eq!(i, cpu.reg_a);
        assert_zn!(cpu, i == 0, i >= 0x80);
    }
}

#[test]
fn test_tya() {
    // Values/flags
    for i in 0..=0xff {
        let (mut cpu, mut ram) = test_cpu(&vec![TYA_IMP, i]);
        cpu.reg_y = i;
        cpu.reg_a = !i;
        cpu.run_one(&mut ram);
        assert_eq!(i, cpu.reg_a);
        assert_zn!(cpu, i == 0, i >= 0x80);
    }
}