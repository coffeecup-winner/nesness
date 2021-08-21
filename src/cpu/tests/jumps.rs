use super::*;

#[test]
fn test_jmp() {
    // Values
    for addr in 0..=0xffff {
        let (mut cpu, mut ram) = test_cpu(&vec![JMP_ABS, lo(addr), hi(addr)]);
        assert_eq!(3, cpu.run_one(&mut ram));
        assert_eq!(addr, cpu.pc);
    }
    for addr in 0..=0xffff {
        let (mut cpu, mut ram) = test_cpu(&vec![JMP_IND, 0x34, 0x12]);
        ram[0x1234] = lo(addr);
        ram[0x1235] = hi(addr);
        assert_eq!(5, cpu.run_one(&mut ram));
        assert_eq!(addr, cpu.pc);
    }
}

#[test]
fn test_jsr() {
    // Values
    for addr in 0..=0xffff {
        let (mut cpu, mut ram) = test_cpu(&vec![JSR_ABS, lo(addr), hi(addr)]);
        cpu.reg_s = 0x01;
        let return_addr = cpu.pc + 2;
        assert_eq!(6, cpu.run_one(&mut ram));
        assert_eq!(addr, cpu.pc);
        assert_eq!(lo(return_addr), ram[0x100]);
        assert_eq!(hi(return_addr), ram[0x101]);
    }
}

#[test]
fn test_rts() {
    // Values
    for addr in 0..=0xffff {
        let (mut cpu, mut ram) = test_cpu(&vec![RTS_IMP]);
        cpu.reg_s = 0x7f;
        ram[0x180] = lo(addr);
        ram[0x181] = hi(addr);
        assert_eq!(6, cpu.run_one(&mut ram));
        assert_eq!(addr.wrapping_add(1), cpu.pc);
    }
}
