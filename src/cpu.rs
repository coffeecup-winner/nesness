#[derive(Default, Debug)]
pub struct CPU {
    pub pc: u16,
    // Registers
    pub reg_a: u8,
    pub reg_x: u8,
    pub reg_y: u8,
    // Flags
    pub flag_carry: bool,
    pub flag_zero: bool,
    pub flag_interrupt_disable: bool,
    pub flag_break: bool,
    pub flag_overflow: bool,
    pub flag_negative: bool,
}

#[allow(dead_code)]
impl CPU {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run_one(&mut self, ram: &mut [u8]) -> u8 {
        let opcode = self.get_next_byte(ram);
        use crate::rp2a03::{info, opcodes::*};
        match opcode {
            ADC_IMM => {
                let info = &info::ADC_IMM;
                let a = self.reg_a;
                let b = self.get_next_byte(ram);
                let mut result = a as u16 + b as u16;
                if self.flag_carry {
                    result += 1;
                }
                let res_u8 = result as u8;
                self.reg_a = res_u8;
                if (result & 0x0100) != 0 {
                    self.flag_carry = true;
                }
                if self.reg_a == 0 {
                    self.flag_zero = true;
                }
                // If signs of both inputs is different from the sign of the result
                if ((a ^ res_u8) & (b ^ res_u8) & 0x80) != 0 {
                    self.flag_overflow = true;
                }
                if (self.reg_a & 0x80) != 0 {
                    self.flag_negative = true;
                }
                info.cycles
            }
            _ => panic!("Illegal instruction: {}", opcode),
        }
    }

    fn get_next_byte(&mut self, ram: &[u8]) -> u8 {
        let byte = ram[self.pc as usize];
        self.pc += 1;
        byte
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rp2a03::opcodes::*;

    #[test]
    fn test_adc_imm() {
        let mut ram = vec![ADC_IMM, 42];
        let mut cpu = CPU::new();
        cpu.run_one(&mut ram);
        assert_eq!(42, cpu.reg_a);
    }
}
