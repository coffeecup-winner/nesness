use crate::cpu::rp2a03::{flags, info, AddressingMode, Instruction};

#[derive(Default, Debug)]
pub struct CPU {
    // Registers
    pub pc: u16,
    pub reg_a: u8,
    pub reg_x: u8,
    pub reg_y: u8,
    pub reg_s: u8,

    // Flags
    pub flag_carry: bool,
    pub flag_zero: bool,
    pub flag_interrupt_disable: bool,
    pub flag_break: bool,
    pub flag_overflow: bool,
    pub flag_negative: bool,
    
    // Data for testing
    #[cfg(test)]
    __saved_flags: u8,
}

struct AddressedByte {
    pub addr: u16,
    pub prefetched_byte: u8,
    pub has_crossed_page: bool,
}

impl AddressedByte {
    pub fn new(addr: u16, byte: u8, has_crossed_page: bool) -> Self {
        AddressedByte {
            addr,
            prefetched_byte: byte,
            has_crossed_page,
        }
    }
}

struct AddressedByteMut<'a> {
    pub prefetched_byte: u8,
    pub byte: &'a mut u8,
}

impl<'a> AddressedByteMut<'a> {
    pub fn new(byte: &'a mut u8) -> Self {
        AddressedByteMut {
            prefetched_byte: *byte,
            byte,
        }
    }
}

#[allow(dead_code)]
impl CPU {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self, ram: &[u8]) {
        // Detailed in https://www.pagetable.com/?p=410
        // Internals of BRK/IRQ/NMI/RESET on a MOS 6502 by Michael Steil
        *self = Self::default();
        self.reg_s = self.reg_s.wrapping_sub(3);
        self.pc = CPU::read_addr(ram, 0xfffc);
    }

    pub fn run_one(&mut self, ram: &mut [u8]) -> u8 {
        let opcode = self.get_next_byte(ram);
        let info = &info::INFO[opcode as usize];
        #[cfg(test)]
        self.__save_flags();
        let extra_cycles = match info.insn {
            // ===== Load/store operations =====
            Instruction::LDA => {
                let AddressedByte {
                    prefetched_byte: m,
                    has_crossed_page,
                    ..
                } = self.get_addressed_byte(info.addressing, ram);
                self.reg_a = m;
                self.update_zn_flags(self.reg_a);
                if has_crossed_page {
                    1
                } else {
                    0
                }
            }
            Instruction::LDX => {
                let AddressedByte {
                    prefetched_byte: m,
                    has_crossed_page,
                    ..
                } = self.get_addressed_byte(info.addressing, ram);
                self.reg_x = m;
                self.update_zn_flags(self.reg_x);
                if has_crossed_page {
                    1
                } else {
                    0
                }
            }
            Instruction::LDY => {
                let AddressedByte {
                    prefetched_byte: m,
                    has_crossed_page,
                    ..
                } = self.get_addressed_byte(info.addressing, ram);
                self.reg_y = m;
                self.update_zn_flags(self.reg_y);
                if has_crossed_page {
                    1
                } else {
                    0
                }
            }
            Instruction::STA => {
                let a = self.reg_a;
                let m = self.get_addressed_byte_mut(info.addressing, ram).byte;
                *m = a;
                0
            }
            Instruction::STX => {
                let x = self.reg_x;
                let m = self.get_addressed_byte_mut(info.addressing, ram).byte;
                *m = x;
                0
            }
            Instruction::STY => {
                let y = self.reg_y;
                let m = self.get_addressed_byte_mut(info.addressing, ram).byte;
                *m = y;
                0
            }

            // ===== Register transfers =====
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
            Instruction::TXA => {
                self.reg_a = self.reg_x;
                self.update_zn_flags(self.reg_a);
                0
            }
            Instruction::TYA => {
                self.reg_a = self.reg_x;
                self.update_zn_flags(self.reg_a);
                0
            }

            // ===== Stack operations =====
            Instruction::TSX => {
                self.reg_x = self.reg_s;
                self.update_zn_flags(self.reg_x);
                0
            }
            Instruction::TXS => {
                self.reg_s = self.reg_x;
                0
            }
            Instruction::PHA => {
                self.push_byte(ram, self.reg_a);
                0
            }
            Instruction::PHP => {
                let p = self.pack_flags();
                self.push_byte(ram, p);
                0
            }
            Instruction::PLA => {
                self.reg_a = self.pull_byte(ram);
                self.update_zn_flags(self.reg_a);
                0
            }
            Instruction::PLP => {
                let p = self.pull_byte(ram);
                self.unpack_flags(p);
                0
            }

            // ===== Logical =====
            Instruction::AND => {
                let a = self.reg_a;
                let AddressedByte {
                    prefetched_byte: m,
                    has_crossed_page,
                    ..
                } = self.get_addressed_byte(info.addressing, ram);
                self.reg_a = a & m;
                self.update_zn_flags(self.reg_a);
                if has_crossed_page {
                    1
                } else {
                    0
                }
            }
            Instruction::EOR => {
                let a = self.reg_a;
                let AddressedByte {
                    prefetched_byte: m,
                    has_crossed_page,
                    ..
                } = self.get_addressed_byte(info.addressing, ram);
                self.reg_a = a ^ m;
                self.update_zn_flags(self.reg_a);
                if has_crossed_page {
                    1
                } else {
                    0
                }
            }
            Instruction::ORA => {
                let a = self.reg_a;
                let AddressedByte {
                    prefetched_byte: m,
                    has_crossed_page,
                    ..
                } = self.get_addressed_byte(info.addressing, ram);
                self.reg_a = a | m;
                self.update_zn_flags(self.reg_a);
                if has_crossed_page {
                    1
                } else {
                    0
                }
            }
            Instruction::BIT => {
                let a = self.reg_a;
                let m = self
                    .get_addressed_byte(info.addressing, ram)
                    .prefetched_byte;
                let result = a & m;
                self.update_zn_flags(result);
                self.flag_overflow = (result & 0x40) != 0;
                0
            }

            // ===== Arithmetic =====
            Instruction::ADC => {
                let a = self.reg_a;
                let AddressedByte {
                    prefetched_byte: m,
                    has_crossed_page,
                    ..
                } = self.get_addressed_byte(info.addressing, ram);
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
            Instruction::SBC => {
                let a = self.reg_a;
                let AddressedByte {
                    prefetched_byte: m,
                    has_crossed_page,
                    ..
                } = self.get_addressed_byte(info.addressing, ram);
                let mut result = a as u16 - m as u16;
                if !self.flag_carry {
                    result -= 1;
                }
                let res_u8 = result as u8;
                self.reg_a = res_u8;
                self.update_zn_flags(self.reg_a);
                self.flag_carry = (result & 0x0100) == 0;
                // If signs of both inputs is different from the sign of the result
                self.flag_overflow = ((a ^ res_u8) & (m ^ res_u8) & 0x80) != 0;
                if has_crossed_page {
                    1
                } else {
                    0
                }
            }
            Instruction::CMP => {
                let a = self.reg_a;
                let AddressedByte {
                    prefetched_byte: m,
                    has_crossed_page,
                    ..
                } = self.get_addressed_byte(info.addressing, ram);
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
                let m = self
                    .get_addressed_byte(info.addressing, ram)
                    .prefetched_byte;
                self.update_zn_flags(x - m);
                self.flag_carry = x >= m;
                0
            }
            Instruction::CPY => {
                let y = self.reg_y;
                let m = self
                    .get_addressed_byte(info.addressing, ram)
                    .prefetched_byte;
                self.update_zn_flags(y - m);
                self.flag_carry = y >= m;
                0
            }

            // ===== Increments/decrements =====
            Instruction::INC => {
                let m = self.get_addressed_byte_mut(info.addressing, ram).byte;
                *m += 1;
                let result = *m;
                self.update_zn_flags(result);
                0
            }
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
            Instruction::DEC => {
                let m = self.get_addressed_byte_mut(info.addressing, ram).byte;
                *m -= 1;
                let result = *m;
                self.update_zn_flags(result);
                0
            }
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

            // ===== Shifts =====
            Instruction::ASL => {
                let AddressedByteMut {
                    prefetched_byte: prev,
                    byte: v,
                    ..
                } = self.get_addressed_byte_mut(info.addressing, ram);
                *v <<= 1;
                let result = *v;
                self.update_zn_flags(result);
                self.flag_carry = (prev & 0x80) != 0;
                0
            }
            Instruction::LSR => {
                let AddressedByteMut {
                    prefetched_byte: prev,
                    byte: v,
                    ..
                } = self.get_addressed_byte_mut(info.addressing, ram);
                *v >>= 1;
                let result = *v;
                self.update_zn_flags(result);
                self.flag_carry = (prev & 0x01) != 0;
                0
            }
            Instruction::ROL => {
                let carry = self.flag_carry;
                let AddressedByteMut {
                    prefetched_byte: prev,
                    byte: v,
                    ..
                } = self.get_addressed_byte_mut(info.addressing, ram);
                *v <<= 1;
                if carry {
                    *v |= 0x01;
                }
                let result = *v;
                self.update_zn_flags(result);
                self.flag_carry = (prev & 0x80) != 0;
                0
            }
            Instruction::ROR => {
                let carry = self.flag_carry;
                let AddressedByteMut {
                    prefetched_byte: prev,
                    byte: v,
                    ..
                } = self.get_addressed_byte_mut(info.addressing, ram);
                *v >>= 1;
                if carry {
                    *v |= 0x80;
                }
                let result = *v;
                self.update_zn_flags(result);
                self.flag_carry = (prev & 0x01) != 0;
                0
            }

            // ===== Jumps/calls =====
            Instruction::JMP => {
                let addr = self.get_addressed_byte(info.addressing, ram).addr;
                self.pc = addr;
                0
            }
            Instruction::JSR => {
                let addr = self.get_addressed_byte(info.addressing, ram).addr;
                let return_addr = self.pc - 1;
                self.push_addr(ram, return_addr);
                self.pc = addr;
                0
            }
            Instruction::RTS => {
                let mut return_addr = self.pull_addr(ram);
                return_addr += 1;
                self.pc = return_addr;
                0
            }

            // ===== Branches =====
            Instruction::BCC => {
                if self.flag_carry {
                    0
                } else {
                    let AddressedByte {
                        addr,
                        has_crossed_page,
                        ..
                    } = self.get_addressed_byte(info.addressing, ram);
                    self.pc = addr;
                    if has_crossed_page {
                        2
                    } else {
                        1
                    }
                }
            }
            Instruction::BCS => {
                if !self.flag_carry {
                    0
                } else {
                    let AddressedByte {
                        addr,
                        has_crossed_page,
                        ..
                    } = self.get_addressed_byte(info.addressing, ram);
                    self.pc = addr;
                    if has_crossed_page {
                        2
                    } else {
                        1
                    }
                }
            }
            Instruction::BEQ => {
                if !self.flag_zero {
                    0
                } else {
                    let AddressedByte {
                        addr,
                        has_crossed_page,
                        ..
                    } = self.get_addressed_byte(info.addressing, ram);
                    self.pc = addr;
                    if has_crossed_page {
                        2
                    } else {
                        1
                    }
                }
            }
            Instruction::BMI => {
                if !self.flag_negative {
                    0
                } else {
                    let AddressedByte {
                        addr,
                        has_crossed_page,
                        ..
                    } = self.get_addressed_byte(info.addressing, ram);
                    self.pc = addr;
                    if has_crossed_page {
                        2
                    } else {
                        1
                    }
                }
            }
            Instruction::BNE => {
                if self.flag_zero {
                    0
                } else {
                    let AddressedByte {
                        addr,
                        has_crossed_page,
                        ..
                    } = self.get_addressed_byte(info.addressing, ram);
                    self.pc = addr;
                    if has_crossed_page {
                        2
                    } else {
                        1
                    }
                }
            }
            Instruction::BPL => {
                if self.flag_negative {
                    0
                } else {
                    let AddressedByte {
                        addr,
                        has_crossed_page,
                        ..
                    } = self.get_addressed_byte(info.addressing, ram);
                    self.pc = addr;
                    if has_crossed_page {
                        2
                    } else {
                        1
                    }
                }
            }
            Instruction::BVC => {
                if self.flag_overflow {
                    0
                } else {
                    let AddressedByte {
                        addr,
                        has_crossed_page,
                        ..
                    } = self.get_addressed_byte(info.addressing, ram);
                    self.pc = addr;
                    if has_crossed_page {
                        2
                    } else {
                        1
                    }
                }
            }
            Instruction::BVS => {
                if !self.flag_overflow {
                    0
                } else {
                    let AddressedByte {
                        addr,
                        has_crossed_page,
                        ..
                    } = self.get_addressed_byte(info.addressing, ram);
                    self.pc = addr;
                    if has_crossed_page {
                        2
                    } else {
                        1
                    }
                }
            }

            // ===== Status flag changes =====
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
            Instruction::SEC => {
                self.flag_carry = true;
                0
            }
            Instruction::SEI => {
                self.flag_interrupt_disable = true;
                0
            }

            // ===== System functions =====
            Instruction::BRK => {
                self.push_addr(ram, self.pc);
                let p = self.pack_flags();
                self.push_byte(ram, p);
                self.pc = CPU::read_addr(ram, 0xfffe);
                self.flag_break = true;
                0
            }
            Instruction::NOP => 0,
            Instruction::RTI => {
                let p = self.pull_byte(ram);
                self.unpack_flags(p);
                self.pc = self.pull_addr(ram);
                0
            }

            // ===== Illegal =====
            Instruction::ILL => panic!("Illegal instruction"),
        };
        #[cfg(test)]
        self.__test_flags(info.affected_flags);
        info.cycles + extra_cycles
    }

    fn update_zn_flags(&mut self, val: u8) {
        self.flag_zero = val == 0;
        self.flag_negative = (val & 0x80) != 0;
    }

    fn pack_flags(&self) -> u8 {
        let mut p = 0x20; // Bit 5 is always set
        if self.flag_carry {
            p |= flags::C;
        }
        if self.flag_zero {
            p |= flags::Z;
        }
        if self.flag_interrupt_disable {
            p |= flags::I;
        }
        if self.flag_break {
            p |= flags::B;
        }
        if self.flag_overflow {
            p |= flags::V;
        }
        if self.flag_negative {
            p |= flags::N;
        }
        p
    }

    fn unpack_flags(&mut self, p: u8) {
        self.flag_carry = (p & flags::C) != 0;
        self.flag_zero = (p & flags::Z) != 0;
        self.flag_interrupt_disable = (p & flags::I) != 0;
        self.flag_break = (p & flags::B) != 0;
        self.flag_overflow = (p & flags::V) != 0;
        self.flag_negative = (p & flags::N) != 0;
    }

    fn get_addressed_byte(&mut self, mode: AddressingMode, ram: &mut [u8]) -> AddressedByte {
        match mode {
            AddressingMode::Implicit => {
                panic!("Implicit addressing mode must be handled by the caller")
            }
            AddressingMode::Accumulator => {
                panic!("Accumulator addressing mode must be handler by the caller")
            }
            AddressingMode::Immediate => {
                let addr = self.pc;
                self.pc += 1; // Need to increase it as we fetch the byte
                AddressedByte::new(addr, ram[addr as usize], false)
            }
            AddressingMode::ZeroPage => {
                let addr = self.get_next_byte(ram) as u16;
                AddressedByte::new(addr, ram[addr as usize], false)
            }
            AddressingMode::ZeroPageX => {
                let addr = (self.get_next_byte(ram) + self.reg_x) as u16;
                AddressedByte::new(addr, ram[addr as usize], false)
            }
            AddressingMode::ZeroPageY => {
                let addr = (self.get_next_byte(ram) + self.reg_y) as u16;
                AddressedByte::new(addr, ram[addr as usize], false)
            }
            AddressingMode::Relative => {
                let offset = self.get_next_byte(ram) as i8;
                let offset_u16 = (0x100 + offset as i16) as u16;
                let addr = (self.pc - 0x100 + offset_u16) as u16;
                let has_crossed_page = (self.pc & 0x0100) != (addr & 0x0100);
                AddressedByte::new(addr, ram[addr as usize], has_crossed_page)
            }
            AddressingMode::Absolute => {
                let lo = self.get_next_byte(ram);
                let hi = self.get_next_byte(ram);
                let addr = ((hi as u16) << 8) + lo as u16;
                AddressedByte::new(addr, ram[addr as usize], false)
            }
            AddressingMode::AbsoluteX => {
                let lo = self.get_next_byte(ram);
                let hi = self.get_next_byte(ram);
                let addr_base = ((hi as u16) << 8) + lo as u16;
                let addr = addr_base + self.reg_x as u16;
                let has_crossed_page = self.reg_x > addr as u8;
                AddressedByte::new(addr, ram[addr as usize], has_crossed_page)
            }
            AddressingMode::AbsoluteY => {
                let lo = self.get_next_byte(ram);
                let hi = self.get_next_byte(ram);
                let addr_base = ((hi as u16) << 8) + lo as u16;
                let addr = addr_base + self.reg_y as u16;
                let has_crossed_page = self.reg_y > addr as u8;
                AddressedByte::new(addr, ram[addr as usize], has_crossed_page)
            }
            AddressingMode::Indirect => {
                let lo = self.get_next_byte(ram);
                let hi = self.get_next_byte(ram);
                let addr_indirect = ((hi as u16) << 8) + lo as u16;
                let addr = CPU::read_addr(ram, addr_indirect);
                AddressedByte::new(addr, ram[addr as usize], false)
            }
            AddressingMode::IndexedIndirect => {
                let zero_page_addr = (self.get_next_byte(ram) + self.reg_x) as u16;
                let addr = CPU::read_addr(ram, zero_page_addr);
                AddressedByte::new(addr, ram[addr as usize], false)
            }
            AddressingMode::IndirectIndexed => {
                let zero_page_addr = self.get_next_byte(ram);
                let addr_base = CPU::read_addr(ram, zero_page_addr as u16);
                let addr = addr_base + self.reg_y as u16;
                let has_crossed_page = self.reg_y > addr as u8;
                AddressedByte::new(addr, ram[addr as usize], has_crossed_page)
            }
        }
    }

    fn get_addressed_byte_mut<'a>(
        &'a mut self,
        mode: AddressingMode,
        ram: &'a mut [u8],
    ) -> AddressedByteMut<'a> {
        match mode {
            AddressingMode::Implicit => {
                panic!("Implicit addressing mode must be handled by the caller")
            }
            AddressingMode::Accumulator => AddressedByteMut::new(&mut self.reg_a),
            AddressingMode::Immediate => {
                panic!("Can't address immediate as mutable")
            }
            AddressingMode::ZeroPage => {
                let addr = self.get_next_byte(ram) as u16;
                AddressedByteMut::new(&mut ram[addr as usize])
            }
            AddressingMode::ZeroPageX => {
                let addr = (self.get_next_byte(ram) + self.reg_x) as u16;
                AddressedByteMut::new(&mut ram[addr as usize])
            }
            AddressingMode::ZeroPageY => {
                let addr = (self.get_next_byte(ram) + self.reg_y) as u16;
                AddressedByteMut::new(&mut ram[addr as usize])
            }
            AddressingMode::Relative => {
                panic!("Relative addressing mode must be handled by the caller")
            }
            AddressingMode::Absolute => {
                let lo = self.get_next_byte(ram);
                let hi = self.get_next_byte(ram);
                let addr = ((hi as u16) << 8) + lo as u16;
                AddressedByteMut::new(&mut ram[addr as usize])
            }
            AddressingMode::AbsoluteX => {
                let lo = self.get_next_byte(ram);
                let hi = self.get_next_byte(ram);
                let addr_base = ((hi as u16) << 8) + lo as u16;
                let addr = addr_base + self.reg_x as u16;
                AddressedByteMut::new(&mut ram[addr as usize])
            }
            AddressingMode::AbsoluteY => {
                let lo = self.get_next_byte(ram);
                let hi = self.get_next_byte(ram);
                let addr_base = ((hi as u16) << 8) + lo as u16;
                let addr = addr_base + self.reg_y as u16;
                AddressedByteMut::new(&mut ram[addr as usize])
            }
            AddressingMode::Indirect => {
                let lo = self.get_next_byte(ram);
                let hi = self.get_next_byte(ram);
                let addr_indirect = ((hi as u16) << 8) + lo as u16;
                let addr = CPU::read_addr(ram, addr_indirect);
                AddressedByteMut::new(&mut ram[addr as usize])
            }
            AddressingMode::IndexedIndirect => {
                let zero_page_addr = (self.get_next_byte(ram) + self.reg_x) as u16;
                let addr = CPU::read_addr(ram, zero_page_addr);
                AddressedByteMut::new(&mut ram[addr as usize])
            }
            AddressingMode::IndirectIndexed => {
                let zero_page_addr = self.get_next_byte(ram);
                let addr_base = CPU::read_addr(ram, zero_page_addr as u16);
                let addr = addr_base + self.reg_y as u16;
                AddressedByteMut::new(&mut ram[addr as usize])
            }
        }
    }

    fn get_next_byte(&mut self, ram: &[u8]) -> u8 {
        let byte = ram[self.pc as usize];
        self.pc += 1;
        byte
    }

    fn push_byte(&mut self, ram: &mut [u8], b: u8) {
        ram[(0x0100 + self.reg_s as u16) as usize] = b;
        self.reg_s -= 1;
    }

    fn pull_byte(&mut self, ram: &[u8]) -> u8 {
        self.reg_s += 1;
        return ram[(0x0100 + self.reg_s as u16) as usize];
    }

    fn push_addr(&mut self, ram: &mut [u8], addr: u16) {
        self.push_byte(ram, (addr >> 8) as u8);
        self.push_byte(ram, addr as u8);
    }

    fn pull_addr(&mut self, ram: &[u8]) -> u16 {
        let mut addr = self.pull_byte(ram) as u16;
        addr |= (self.pull_byte(ram) as u16) << 8;
        addr
    }

    fn read_addr(ram: &[u8], addr: u16) -> u16 {
        let mut result = ram[addr as usize] as u16;
        result |= (ram[addr as usize + 1] as u16) << 8;
        result
    }

    #[cfg(test)]
    fn __save_flags(&mut self) {
        self.__saved_flags = self.pack_flags();
    }

    #[cfg(test)]
    fn __test_flags(&self, allowed_flags: u8) {
        let current_flags = self.pack_flags();
        let change = self.__saved_flags ^ current_flags;
        assert_eq!(0, change & !allowed_flags, "Unexpected flags have been modified");
    }
}
