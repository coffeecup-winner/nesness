use crate::rp2a03::{info, AddressingMode, Instruction};

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
        let info = &info::INFO[opcode as usize];
        match info.insn {
            Instruction::ADC => {
                let a = self.reg_a;
                let (b, has_crossed_page) = self.get_addressed_byte(info.addressing, ram);
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
                info.cycles + if has_crossed_page { 1 } else { 0 }
            }
            i => panic!("Illegal instruction: {:?}", i),
        }
    }

    // Returns (byte, has crossed the page)
    fn get_addressed_byte(&mut self, mode: AddressingMode, ram: &[u8]) -> (u8, bool) {
        match mode {
            AddressingMode::Invalid => panic!("Invalid addressing mode"),
            AddressingMode::Implicit => {
                panic!("Implicit addressing mode must be handled by the caller")
            }
            AddressingMode::Accumulator => (self.reg_a, false),
            AddressingMode::Immediate => (self.get_next_byte(ram), false),
            AddressingMode::ZeroPage => (ram[self.get_next_byte(ram) as usize], false),
            AddressingMode::ZeroPageX => {
                (ram[(self.get_next_byte(ram) + self.reg_x) as usize], false)
            }
            AddressingMode::ZeroPageY => {
                (ram[(self.get_next_byte(ram) + self.reg_y) as usize], false)
            }
            AddressingMode::Relative => {
                panic!("Relative addressing mode must be handled by the caller")
            }
            AddressingMode::Absolute => {
                let lo = self.get_next_byte(ram);
                let hi = self.get_next_byte(ram);
                let addr = (hi as u16) << 8 + lo;
                (ram[addr as usize], false)
            }
            AddressingMode::AbsoluteX => {
                let lo = self.get_next_byte(ram);
                let hi = self.get_next_byte(ram);
                let addr_base = (hi as u16) << 8 + lo;
                let addr = addr_base + self.reg_x as u16;
                let has_crossed_page = self.reg_x > addr as u8;
                (ram[addr as usize], has_crossed_page)
            }
            AddressingMode::AbsoluteY => {
                let lo = self.get_next_byte(ram);
                let hi = self.get_next_byte(ram);
                let addr_base = (hi as u16) << 8 + lo;
                let addr = addr_base + self.reg_y as u16;
                let has_crossed_page = self.reg_y > addr as u8;
                (ram[addr as usize], has_crossed_page)
            }
            AddressingMode::Indirect => {
                let lo = self.get_next_byte(ram);
                let hi = self.get_next_byte(ram);
                let addr_indirect = ((hi as u16) << 8 + lo) as usize;
                let lo = ram[addr_indirect];
                let hi = ram[addr_indirect + 1];
                let addr = (hi as u16) << 8 + lo;
                (ram[addr as usize], false)
            }
            AddressingMode::IndexedIndirect => {
                let zero_page_addr = self.get_next_byte(ram);
                let addr = ram[(zero_page_addr + self.reg_x) as usize];
                (ram[addr as usize], false)
            }
            AddressingMode::IndirectIndexed => {
                let zero_page_addr = self.get_next_byte(ram);
                let addr_base = ram[zero_page_addr as usize] as u16;
                let addr = addr_base + self.reg_y as u16;
                let has_crossed_page = self.reg_y > addr as u8;
                (ram[addr as usize], has_crossed_page)
            }
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
