mod cpu;
pub mod rp2a03;

pub use cpu::*;

#[cfg(test)]
mod tests {
    use super::*;
    use rp2a03::opcodes::*;

    macro_rules! assert_zn {
        ($cpu: ident, $z: literal, $n: literal) => {
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

    #[test]
    fn test_lda_imm() {
        let (mut cpu, mut ram) = test_cpu(&vec![
            LDX_IMM, 0x10,
            LDY_IMM, 0x20,

            // Addressing modes
            LDA_IMM, 0x42,
            LDA_ZPG, 0x80,
            LDA_ZPX, 0x80,
            LDA_ABS, 0x34, 0x12,
            LDA_ABX, 0x34, 0x12,
            LDA_ABY, 0x34, 0x12,
            LDA_INX, 0xa0,
            LDA_INY, 0xc0,

            // Page crossing
            LDA_ABX, 0xf4, 0x12,
            LDA_ABY, 0xe5, 0x12,
            LDY_IMM, 0xc0,
            LDA_INY, 0xc0,

            // Flags
            LDA_IMM, 0xff,
            LDA_IMM, 0x00,
        ]);
        ram[0x80] = 0x43;
        ram[0x90] = 0x44;
        ram[0xb0] = 0x89;
        ram[0xb1] = 0x67;
        ram[0xc0] = 0x56;
        ram[0xc1] = 0x34;
        ram[0x1234] = 0x45;
        ram[0x1244] = 0x46;
        ram[0x1254] = 0x47;
        ram[0x6789] = 0x48;
        ram[0x3476] = 0x49;
        ram[0x1304] = 0x4a;
        ram[0x1305] = 0x4b;
        ram[0x3516] = 0x4c;
        cpu.run_one(&mut ram);
        cpu.run_one(&mut ram);

        assert_eq!(2, cpu.run_one(&mut ram));
        assert_eq!(0x42, cpu.reg_a);
        assert_zn!(cpu, false, false);
        assert_eq!(3, cpu.run_one(&mut ram));
        assert_eq!(0x43, cpu.reg_a);
        assert_zn!(cpu, false, false);
        assert_eq!(4, cpu.run_one(&mut ram));
        assert_eq!(0x44, cpu.reg_a);
        assert_zn!(cpu, false, false);
        assert_eq!(4, cpu.run_one(&mut ram));
        assert_eq!(0x45, cpu.reg_a);
        assert_zn!(cpu, false, false);
        assert_eq!(4, cpu.run_one(&mut ram));
        assert_eq!(0x46, cpu.reg_a);
        assert_zn!(cpu, false, false);
        assert_eq!(4, cpu.run_one(&mut ram));
        assert_eq!(0x47, cpu.reg_a);
        assert_zn!(cpu, false, false);
        assert_eq!(6, cpu.run_one(&mut ram));
        assert_eq!(0x48, cpu.reg_a);
        assert_zn!(cpu, false, false);
        assert_eq!(5, cpu.run_one(&mut ram));
        assert_eq!(0x49, cpu.reg_a);
        assert_zn!(cpu, false, false);

        assert_eq!(5, cpu.run_one(&mut ram));
        assert_eq!(0x4a, cpu.reg_a);
        assert_zn!(cpu, false, false);
        assert_eq!(5, cpu.run_one(&mut ram));
        assert_eq!(0x4b, cpu.reg_a);
        assert_zn!(cpu, false, false);
        cpu.run_one(&mut ram);
        assert_eq!(6, cpu.run_one(&mut ram));
        assert_eq!(0x4c, cpu.reg_a);
        assert_zn!(cpu, false, false);

        assert_eq!(2, cpu.run_one(&mut ram));
        assert_eq!(0xff, cpu.reg_a);
        assert_zn!(cpu, false, true);
        assert_eq!(2, cpu.run_one(&mut ram));
        assert_eq!(0x00, cpu.reg_a);
        assert_zn!(cpu, true, false);
    }

    #[test]
    fn test_adc_imm() {
        let (mut cpu, mut ram) = test_cpu(&vec![ADC_IMM, 42]);
        cpu.run_one(&mut ram);
        assert_eq!(42, cpu.reg_a);
    }
}
