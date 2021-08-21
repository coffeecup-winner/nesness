mod cpu;
pub mod rp2a03;

pub use cpu::*;

#[cfg(test)]
mod tests {
    use super::*;
    use rp2a03::opcodes::*;

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

    #[test]
    fn test_adc_imm() {
        let (mut cpu, mut ram) = test_cpu(&vec![ADC_IMM, 42]);
        cpu.run_one(&mut ram);
        assert_eq!(42, cpu.reg_a);
    }
}
