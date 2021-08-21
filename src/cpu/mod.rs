mod cpu;
pub mod rp2a03;

pub use cpu::*;

#[cfg(test)]
mod tests {
    use super::*;
    use rp2a03::opcodes::*;

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

    #[test]
    fn test_lda() {
        // Values/flags
        for i in 0..=0xff {
            let (mut cpu, mut ram) = test_cpu(&vec![LDA_IMM, i]);
            cpu.run_one(&mut ram);
            assert_eq!(i, cpu.reg_a);
            assert_zn!(cpu, i == 0, i >= 0x80);
        }
        
        // Addressing
        let x = 0x10;
        let y = 0x20;
        let y2 = 0xc0;
        let v = 0x40;
        let zpg = 0x80;
        let zpg_inx = 0xa0;
        let inx = 0x6789;
        let zpg_iny = 0xc0;
        let iny = 0x3456;
        let abs = 0x1234;
        let abs2 = 0x12f8;
        let (mut cpu, mut ram) = test_cpu(&vec![
            LDA_IMM, v + 0,
            LDA_ZPG, zpg,
            LDA_ZPX, zpg,
            LDA_ABS, lo(abs), hi(abs),
            LDA_ABX, lo(abs), hi(abs),
            LDA_ABY, lo(abs), hi(abs),
            LDA_INX, zpg_inx,
            LDA_INY, zpg_iny,

            // Page crossing
            LDA_ABX, lo(abs2), hi(abs2),
            LDA_ABY, lo(abs2), hi(abs2),
            LDA_INY, zpg_iny,
        ]);
        cpu.reg_x = x;
        cpu.reg_y = y;
        ram[zpg as usize] = v + 1;
        ram[(zpg + x) as usize] = v + 2;
        ram[(zpg_inx + x) as usize] = lo(inx);
        ram[(zpg_inx + x + 1) as usize] = hi(inx);
        ram[zpg_iny as usize] = lo(iny);
        ram[(zpg_iny + 1) as usize] = hi(iny);
        ram[abs as usize] = v + 3;
        ram[(abs + x as u16) as usize] = v + 4;
        ram[(abs + y as u16) as usize] = v + 5;
        ram[inx as usize] = v + 6;
        ram[(iny + y as u16) as usize] = v + 7;
        ram[(abs2 + x as u16) as usize] = v + 8;
        ram[(abs2 + y as u16) as usize] = v + 9;
        ram[(iny + y2 as u16) as usize] = v + 10;

        assert_eq!(2, cpu.run_one(&mut ram));
        assert_eq!(v + 0, cpu.reg_a);
        assert_eq!(3, cpu.run_one(&mut ram));
        assert_eq!(v + 1, cpu.reg_a);
        assert_eq!(4, cpu.run_one(&mut ram));
        assert_eq!(v + 2, cpu.reg_a);
        assert_eq!(4, cpu.run_one(&mut ram));
        assert_eq!(v + 3, cpu.reg_a);
        assert_eq!(4, cpu.run_one(&mut ram));
        assert_eq!(v + 4, cpu.reg_a);
        assert_eq!(4, cpu.run_one(&mut ram));
        assert_eq!(v + 5, cpu.reg_a);
        assert_eq!(6, cpu.run_one(&mut ram));
        assert_eq!(v + 6, cpu.reg_a);
        assert_eq!(5, cpu.run_one(&mut ram));
        assert_eq!(v + 7, cpu.reg_a);

        assert_eq!(5, cpu.run_one(&mut ram));
        assert_eq!(v + 8, cpu.reg_a);
        assert_eq!(5, cpu.run_one(&mut ram));
        assert_eq!(v + 9, cpu.reg_a);
        cpu.reg_y = y2;
        assert_eq!(6, cpu.run_one(&mut ram));
        assert_eq!(v + 10, cpu.reg_a);
    }

    #[test]
    fn test_adc_imm() {
        let (mut cpu, mut ram) = test_cpu(&vec![ADC_IMM, 42]);
        cpu.run_one(&mut ram);
        assert_eq!(42, cpu.reg_a);
    }
}
