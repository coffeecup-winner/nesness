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
        let extra_cycles = match info.insn {
            Instruction::ILL => panic!("Illegal instruction"),
            Instruction::ADC => {
                let a = self.reg_a;
                let (m, has_crossed_page) = self.get_addressed_byte(info.addressing, ram);
                let mut result = a as u16 + m as u16;
                if self.flag_carry {
                    result += 1;
                }
                let res_u8 = result as u8;
                self.reg_a = res_u8;
                self.update_zn_flags(self.reg_a);
                self.flag_carry = (result & 0x0100) != 0;
                // If signs of both inputs is different from the sign of the result
                self.flag_overflow = ((a ^ res_u8) & (m ^ res_u8) & 0x80) != 0;
                if has_crossed_page {
                    1
                } else {
                    0
                }
            }
            Instruction::AND => {
                let a = self.reg_a;
                let (m, has_crossed_page) = self.get_addressed_byte(info.addressing, ram);
                self.reg_a = a & m;
                self.update_zn_flags(self.reg_a);
                if has_crossed_page {
                    1
                } else {
                    0
                }
            }
            Instruction::ASL => {
                let (m, _) = self.get_addressed_byte(info.addressing, ram);
                self.reg_a = m << 1;
                self.update_zn_flags(self.reg_a);
                self.flag_carry = (m & 0x80) != 0;
                0
            }
            Instruction::BCC => todo!(),
            Instruction::BCS => todo!(),
            Instruction::BEQ => todo!(),
            Instruction::BIT => {
                let a = self.reg_a;
                let (m, _) = self.get_addressed_byte(info.addressing, ram);
                let result = a & m;
                self.update_zn_flags(result);
                self.flag_overflow = (result & 0x40) != 0;
                0
            }
            Instruction::BMI => todo!(),
            Instruction::BNE => todo!(),
            Instruction::BPL => todo!(),
            Instruction::BRK => todo!(),
            Instruction::BVC => todo!(),
            Instruction::BVS => todo!(),
            Instruction::CLC => {
                self.flag_carry = false;
                0
            }
            Instruction::CLI => {
                self.flag_interrupt_disable = false;
                0
            }
            Instruction::CLV => {
                self.flag_overflow = false;
                0
            }
            Instruction::CMP => {
                let a = self.reg_a;
                let (m, has_crossed_page) = self.get_addressed_byte(info.addressing, ram);
                self.update_zn_flags(a - m);
                self.flag_carry = a >= m;
                if has_crossed_page {
                    1
                } else {
                    0
                }
            }
            Instruction::CPX => {
                let x = self.reg_x;
                let (m, _) = self.get_addressed_byte(info.addressing, ram);
                self.update_zn_flags(x - m);
                self.flag_carry = x >= m;
                0
            }
            Instruction::CPY => {
                let y = self.reg_y;
                let (m, _) = self.get_addressed_byte(info.addressing, ram);
                self.update_zn_flags(y - m);
                self.flag_carry = y >= m;
                0
            }
            Instruction::DEC => todo!(),
            Instruction::DEX => {
                self.reg_x -= 1;
                self.update_zn_flags(self.reg_x);
                0
            }
            Instruction::DEY => {
                self.reg_y -= 1;
                self.update_zn_flags(self.reg_y);
                0
            }
            Instruction::EOR => {
                let a = self.reg_a;
                let (m, has_crossed_page) = self.get_addressed_byte(info.addressing, ram);
                self.reg_a = a ^ m;
                self.update_zn_flags(self.reg_a);
                if has_crossed_page {
                    1
                } else {
                    0
                }
            }
            Instruction::INC => todo!(),
            Instruction::INX => {
                self.reg_x += 1;
                self.update_zn_flags(self.reg_x);
                0
            }
            Instruction::INY => {
                self.reg_y += 1;
                self.update_zn_flags(self.reg_y);
                0
            }
            Instruction::JMP => todo!(),
            Instruction::JSR => todo!(),
            Instruction::LDA => {
                let (m, has_crossed_page) = self.get_addressed_byte(info.addressing, ram);
                self.reg_a = m;
                self.update_zn_flags(self.reg_a);
                if has_crossed_page {
                    1
                } else {
                    0
                }
            }
            Instruction::LDX => {
                let (m, has_crossed_page) = self.get_addressed_byte(info.addressing, ram);
                self.reg_x = m;
                self.update_zn_flags(self.reg_x);
                if has_crossed_page {
                    1
                } else {
                    0
                }
            }
            Instruction::LDY => {
                let (m, has_crossed_page) = self.get_addressed_byte(info.addressing, ram);
                self.reg_y = m;
                self.update_zn_flags(self.reg_y);
                if has_crossed_page {
                    1
                } else {
                    0
                }
            }
            Instruction::LSR => {
                let (m, _) = self.get_addressed_byte(info.addressing, ram);
                self.reg_a = m >> 1;
                self.update_zn_flags(self.reg_a);
                self.flag_carry = (m & 0x01) != 0;
                0
            }
            Instruction::NOP => 0,
            Instruction::ORA => {
                let a = self.reg_a;
                let (m, has_crossed_page) = self.get_addressed_byte(info.addressing, ram);
                self.reg_a = a | m;
                self.update_zn_flags(self.reg_a);
                if has_crossed_page {
                    1
                } else {
                    0
                }
            }
            Instruction::PHA => todo!(),
            Instruction::PHP => todo!(),
            Instruction::PLA => todo!(),
            Instruction::PLP => todo!(),
            Instruction::ROL => {
                let (m, _) = self.get_addressed_byte(info.addressing, ram);
                self.reg_a = m << 1;
                if self.flag_carry {
                    self.reg_a |= 0x01;
                }
                self.update_zn_flags(self.reg_a);
                self.flag_carry = (m & 0x80) != 0;
                0
            }
            Instruction::ROR => {
                let (m, _) = self.get_addressed_byte(info.addressing, ram);
                self.reg_a = m >> 1;
                if self.flag_carry {
                    self.reg_a |= 0x80;
                }
                self.update_zn_flags(self.reg_a);
                self.flag_carry = (m & 0x01) != 0;
                0
            }
            Instruction::RTI => todo!(),
            Instruction::RTS => todo!(),
            Instruction::SBC => todo!(),
            Instruction::SEC => {
                self.flag_carry = true;
                0
            }
            Instruction::SEI => {
                self.flag_interrupt_disable = true;
                0
            }
            Instruction::STA => todo!(),
            Instruction::STX => todo!(),
            Instruction::STY => todo!(),
            Instruction::TAX => {
                self.reg_x = self.reg_a;
                self.update_zn_flags(self.reg_x);
                0
            }
            Instruction::TAY => {
                self.reg_y = self.reg_a;
                self.update_zn_flags(self.reg_y);
                0
            }
            Instruction::TSX => todo!(),
            Instruction::TXA => {
                self.reg_a = self.reg_x;
                self.update_zn_flags(self.reg_a);
                0
            }
            Instruction::TXS => todo!(),
            Instruction::TYA => {
                self.reg_a = self.reg_x;
                self.update_zn_flags(self.reg_a);
                0
            }
        };
        info.cycles + extra_cycles
    }

    fn update_zn_flags(&mut self, val: u8) {
        self.flag_zero = val == 0;
        self.flag_negative = (val & 0x80) != 0;
    }

    // Returns (byte, has crossed the page)
    fn get_addressed_byte(&mut self, mode: AddressingMode, ram: &[u8]) -> (u8, bool) {
        match mode {
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
