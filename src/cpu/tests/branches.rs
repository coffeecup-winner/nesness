use super::*;

fn test_common<F: FnMut(&mut CPU) -> &mut bool>(opcode: u8, branch_on_set: bool, mut get_flag: F) {
    // Values
    for i in -128..=127i8 {
        let (mut cpu, mut ram) = test_cpu(&vec![opcode, i as u8]);
        *get_flag(&mut cpu) = branch_on_set;
        let prev_pc = cpu.pc;
        let new_pc = (cpu.pc as i16 + i as i16 + 2) as u16;
        let extra_cycle = if ((prev_pc & 0x0100) ^ (new_pc & 0x0100)) != 0 {
            1
        } else {
            0
        };
        assert_eq!(3 + extra_cycle, cpu.run_one(&mut ram));
        assert_eq!(new_pc, cpu.pc);
    }

    // No branch
    for i in -128..=127i8 {
        let (mut cpu, mut ram) = test_cpu(&vec![opcode, i as u8]);
        *get_flag(&mut cpu) = !branch_on_set;
        let new_pc = cpu.pc + 2;
        assert_eq!(2, cpu.run_one(&mut ram));
        assert_eq!(new_pc, cpu.pc);
    }
}

#[test]
fn test_bcc() {
    test_common(BCC_REL, false, |cpu| &mut cpu.flag_carry);
}

#[test]
fn test_bcs() {
    test_common(BCS_REL, true, |cpu| &mut cpu.flag_carry);
}

#[test]
fn test_beq() {
    test_common(BEQ_REL, true, |cpu| &mut cpu.flag_zero);
}

#[test]
fn test_bmi() {
    test_common(BMI_REL, true, |cpu| &mut cpu.flag_negative);
}

#[test]
fn test_bne() {
    test_common(BNE_REL, false, |cpu| &mut cpu.flag_zero);
}

#[test]
fn test_bpl() {
    test_common(BPL_REL, false, |cpu| &mut cpu.flag_negative);
}

#[test]
fn test_bvc() {
    test_common(BVC_REL, false, |cpu| &mut cpu.flag_overflow);
}

#[test]
fn test_bvs() {
    test_common(BVS_REL, true, |cpu| &mut cpu.flag_overflow);
}
