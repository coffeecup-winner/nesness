use crate::{
    cpu::rp2a03::{flags, info, AddressingMode, Instruction},
    mem::Memory,
};

#[derive(Debug, Default, Clone)]
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
    pub flag_decimal_mode: bool,
    pub flag_break: bool,
    pub flag_overflow: bool,
    pub flag_negative: bool,

    // Data for testing
    #[cfg(test)]
    __insn_bytes_read: u8,
    #[cfg(test)]
    __saved_a: u8,
    #[cfg(test)]
    __saved_x: u8,
    #[cfg(test)]
    __saved_y: u8,
    #[cfg(test)]
    __saved_pc: u16,
    #[cfg(test)]
    __saved_s: u8,
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

    pub fn reset(&mut self, mem: &dyn Memory) {
        // Detailed in https://www.pagetable.com/?p=410
        // Internals of BRK/IRQ/NMI/RESET on a MOS 6502 by Michael Steil
        *self = Self::default();
        self.flag_interrupt_disable = true;
        self.reg_s = self.reg_s.wrapping_sub(3);
        self.pc = mem.read_u16(0xfffc);
    }

    pub fn run_one(&mut self, mem: &mut dyn Memory) -> u8 {
        #[cfg(test)]
        self.__init_checks();
        let opcode = self.get_next_byte(mem);
        let info = &info::INFO[opcode as usize];
        let extra_cycles = match info.insn {
            // ===== Load/store operations =====
            Instruction::LDA => {
                let AddressedByte {
                    prefetched_byte: m,
                    has_crossed_page,
                    ..
                } = self.get_addressed_byte(info.addressing, mem);
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
                } = self.get_addressed_byte(info.addressing, mem);
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
                } = self.get_addressed_byte(info.addressing, mem);
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
                let m = self.get_addressed_byte_mut(info.addressing, mem).byte;
                *m = a;
                0
            }
            Instruction::STX => {
                let x = self.reg_x;
                let m = self.get_addressed_byte_mut(info.addressing, mem).byte;
                *m = x;
                0
            }
            Instruction::STY => {
                let y = self.reg_y;
                let m = self.get_addressed_byte_mut(info.addressing, mem).byte;
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
                self.reg_a = self.reg_y;
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
                self.push_byte(mem, self.reg_a);
                0
            }
            Instruction::PHP => {
                let mut p = self.pack_flags();
                p |= flags::B; // B is set for PHP
                self.push_byte(mem, p);
                0
            }
            Instruction::PLA => {
                self.reg_a = self.pull_byte(mem);
                self.update_zn_flags(self.reg_a);
                0
            }
            Instruction::PLP => {
                let mut p = self.pull_byte(mem);
                p &= !flags::B; // B is cleared for PLA
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
                } = self.get_addressed_byte(info.addressing, mem);
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
                } = self.get_addressed_byte(info.addressing, mem);
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
                } = self.get_addressed_byte(info.addressing, mem);
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
                    .get_addressed_byte(info.addressing, mem)
                    .prefetched_byte;
                self.flag_zero = (m & a) == 0;
                self.flag_overflow = (m & 0x40) != 0;
                self.flag_negative = (m & 0x80) != 0;
                0
            }

            // ===== Arithmetic =====
            Instruction::ADC => {
                let a = self.reg_a;
                let AddressedByte {
                    prefetched_byte: m,
                    has_crossed_page,
                    ..
                } = self.get_addressed_byte(info.addressing, mem);
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
                } = self.get_addressed_byte(info.addressing, mem);
                // A - M - (1 - C) == A + !M + C
                let mut result = (a as u16).wrapping_add(!(m as u16));
                if self.flag_carry {
                    result = result.wrapping_add(1);
                }
                let res_u8 = result as u8;
                self.reg_a = res_u8;
                self.update_zn_flags(self.reg_a);
                self.flag_carry = (result & 0x0100) == 0;
                // If signs of both inputs is different from the sign of the result
                self.flag_overflow = ((a ^ res_u8) & (!m ^ res_u8) & 0x80) != 0;
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
                } = self.get_addressed_byte(info.addressing, mem);
                self.update_zn_flags(a.wrapping_sub(m));
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
                    .get_addressed_byte(info.addressing, mem)
                    .prefetched_byte;
                self.update_zn_flags(x.wrapping_sub(m));
                self.flag_carry = x >= m;
                0
            }
            Instruction::CPY => {
                let y = self.reg_y;
                let m = self
                    .get_addressed_byte(info.addressing, mem)
                    .prefetched_byte;
                self.update_zn_flags(y.wrapping_sub(m));
                self.flag_carry = y >= m;
                0
            }

            // ===== Increments/decrements =====
            Instruction::INC => {
                let m = self.get_addressed_byte_mut(info.addressing, mem).byte;
                *m = m.wrapping_add(1);
                let result = *m;
                self.update_zn_flags(result);
                0
            }
            Instruction::INX => {
                self.reg_x = self.reg_x.wrapping_add(1);
                self.update_zn_flags(self.reg_x);
                0
            }
            Instruction::INY => {
                self.reg_y = self.reg_y.wrapping_add(1);
                self.update_zn_flags(self.reg_y);
                0
            }
            Instruction::DEC => {
                let m = self.get_addressed_byte_mut(info.addressing, mem).byte;
                *m = m.wrapping_sub(1);
                let result = *m;
                self.update_zn_flags(result);
                0
            }
            Instruction::DEX => {
                self.reg_x = self.reg_x.wrapping_sub(1);
                self.update_zn_flags(self.reg_x);
                0
            }
            Instruction::DEY => {
                self.reg_y = self.reg_y.wrapping_sub(1);
                self.update_zn_flags(self.reg_y);
                0
            }

            // ===== Shifts =====
            Instruction::ASL => {
                let AddressedByteMut {
                    prefetched_byte: prev,
                    byte: v,
                    ..
                } = self.get_addressed_byte_mut(info.addressing, mem);
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
                } = self.get_addressed_byte_mut(info.addressing, mem);
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
                } = self.get_addressed_byte_mut(info.addressing, mem);
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
                } = self.get_addressed_byte_mut(info.addressing, mem);
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
                let addr = self.get_addressed_byte(info.addressing, mem).addr;
                self.pc = addr;
                0
            }
            Instruction::JSR => {
                let addr = self.get_addressed_byte(info.addressing, mem).addr;
                let return_addr = self.pc - 1;
                self.push_addr(mem, return_addr);
                self.pc = addr;
                0
            }
            Instruction::RTS => {
                let mut return_addr = self.pull_addr(mem);
                return_addr = return_addr.wrapping_add(1);
                self.pc = return_addr;
                0
            }

            // ===== Branches =====
            Instruction::BCC => {
                if self.flag_carry {
                    self.get_next_byte(mem); // Advance past offset
                    0
                } else {
                    let AddressedByte {
                        addr,
                        has_crossed_page,
                        ..
                    } = self.get_addressed_byte(info.addressing, mem);
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
                    self.get_next_byte(mem); // Advance past offset
                    0
                } else {
                    let AddressedByte {
                        addr,
                        has_crossed_page,
                        ..
                    } = self.get_addressed_byte(info.addressing, mem);
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
                    self.get_next_byte(mem); // Advance past offset
                    0
                } else {
                    let AddressedByte {
                        addr,
                        has_crossed_page,
                        ..
                    } = self.get_addressed_byte(info.addressing, mem);
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
                    self.get_next_byte(mem); // Advance past offset
                    0
                } else {
                    let AddressedByte {
                        addr,
                        has_crossed_page,
                        ..
                    } = self.get_addressed_byte(info.addressing, mem);
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
                    self.get_next_byte(mem); // Advance past offset
                    0
                } else {
                    let AddressedByte {
                        addr,
                        has_crossed_page,
                        ..
                    } = self.get_addressed_byte(info.addressing, mem);
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
                    self.get_next_byte(mem); // Advance past offset
                    0
                } else {
                    let AddressedByte {
                        addr,
                        has_crossed_page,
                        ..
                    } = self.get_addressed_byte(info.addressing, mem);
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
                    self.get_next_byte(mem); // Advance past offset
                    0
                } else {
                    let AddressedByte {
                        addr,
                        has_crossed_page,
                        ..
                    } = self.get_addressed_byte(info.addressing, mem);
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
                    self.get_next_byte(mem); // Advance past offset
                    0
                } else {
                    let AddressedByte {
                        addr,
                        has_crossed_page,
                        ..
                    } = self.get_addressed_byte(info.addressing, mem);
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
            Instruction::CLD => {
                self.flag_decimal_mode = false;
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
            Instruction::SED => {
                self.flag_decimal_mode = true;
                0
            }
            Instruction::SEI => {
                self.flag_interrupt_disable = true;
                0
            }

            // ===== System functions =====
            Instruction::BRK => {
                self.push_addr(mem, self.pc);
                let p = self.pack_flags();
                self.push_byte(mem, p);
                self.pc = mem.read_u16(0xfffe);
                self.flag_break = true;
                0
            }
            Instruction::NOP => 0,
            Instruction::RTI => {
                let p = self.pull_byte(mem);
                self.unpack_flags(p);
                self.pc = self.pull_addr(mem);
                0
            }

            // ===== Illegal =====
            Instruction::ILL => panic!("Illegal instruction"),
        };
        #[cfg(test)]
        self.__run_checks(info.bytes, info.affected_units, info.affected_flags);
        info.cycles + extra_cycles
    }

    fn update_zn_flags(&mut self, val: u8) {
        self.flag_zero = val == 0;
        self.flag_negative = (val & 0x80) != 0;
    }

    pub fn pack_flags(&self) -> u8 {
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
        if self.flag_decimal_mode {
            p |= flags::D;
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
        self.flag_decimal_mode = (p & flags::D) != 0;
        self.flag_break = (p & flags::B) != 0;
        self.flag_overflow = (p & flags::V) != 0;
        self.flag_negative = (p & flags::N) != 0;
    }

    fn get_addressed_byte(&mut self, mode: AddressingMode, mem: &dyn Memory) -> AddressedByte {
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
                #[cfg(test)]
                {
                    self.__insn_bytes_read += 1;
                }
                AddressedByte::new(addr, mem[addr], false)
            }
            AddressingMode::Relative => {
                let offset = self.get_next_byte(mem) as i8;
                let offset_u16 = (0x100 + offset as i16) as u16;
                let addr = (self.pc - 0x100 + offset_u16) as u16;
                let has_crossed_page = (self.pc & 0x0100) != (addr & 0x0100);
                AddressedByte::new(addr, mem[addr], has_crossed_page)
            }
            AddressingMode::ZeroPage
            | AddressingMode::ZeroPageX
            | AddressingMode::ZeroPageY
            | AddressingMode::Absolute
            | AddressingMode::Indirect
            | AddressingMode::IndexedIndirect => {
                let addr = self.get_address(mode, mem);
                AddressedByte::new(addr, mem[addr], false)
            }
            AddressingMode::AbsoluteX => {
                let addr = self.get_address(mode, mem);
                let has_crossed_page = self.reg_x > addr as u8;
                AddressedByte::new(addr, mem[addr], has_crossed_page)
            }
            AddressingMode::AbsoluteY | AddressingMode::IndirectIndexed => {
                let addr = self.get_address(mode, mem);
                let has_crossed_page = self.reg_y > addr as u8;
                AddressedByte::new(addr, mem[addr], has_crossed_page)
            }
        }
    }

    fn get_addressed_byte_mut<'a>(
        &'a mut self,
        mode: AddressingMode,
        mem: &'a mut dyn Memory,
    ) -> AddressedByteMut<'a> {
        match mode {
            AddressingMode::Implicit => {
                panic!("Implicit addressing mode must be handled by the caller")
            }
            AddressingMode::Immediate => {
                panic!("Can't address immediate as mutable")
            }
            AddressingMode::Relative => {
                panic!("Relative addressing mode must be handled by the caller")
            }
            AddressingMode::Indirect => {
                panic!("Don't need to address indirect as mutable")
            }
            AddressingMode::Accumulator => AddressedByteMut::new(&mut self.reg_a),
            AddressingMode::ZeroPage
            | AddressingMode::ZeroPageX
            | AddressingMode::ZeroPageY
            | AddressingMode::Absolute
            | AddressingMode::AbsoluteX
            | AddressingMode::AbsoluteY
            | AddressingMode::IndexedIndirect
            | AddressingMode::IndirectIndexed => {
                let addr = self.get_address(mode, mem);
                AddressedByteMut::new(&mut mem[addr])
            }
        }
    }

    fn get_address(&mut self, mode: AddressingMode, mem: &dyn Memory) -> u16 {
        match mode {
            AddressingMode::Implicit => {
                panic!("Implicit addressing mode must be handled by the caller")
            }
            AddressingMode::Accumulator => {
                panic!("Accumulator addressing mode must be handled by the caller")
            }
            AddressingMode::Immediate => {
                panic!("Immediate addressing mode must be handled by the caller")
            }
            AddressingMode::ZeroPage => self.get_next_byte(mem) as u16,
            AddressingMode::ZeroPageX => (self.get_next_byte(mem).wrapping_add(self.reg_x)) as u16,
            AddressingMode::ZeroPageY => (self.get_next_byte(mem).wrapping_add(self.reg_y)) as u16,
            AddressingMode::Relative => {
                panic!("Relative addressing mode must be handled by the caller")
            }
            AddressingMode::Absolute => {
                let lo = self.get_next_byte(mem);
                let hi = self.get_next_byte(mem);
                ((hi as u16) << 8) + lo as u16
            }
            AddressingMode::AbsoluteX => {
                let lo = self.get_next_byte(mem);
                let hi = self.get_next_byte(mem);
                let addr_base = ((hi as u16) << 8) + lo as u16;
                addr_base.wrapping_add(self.reg_x as u16)
            }
            AddressingMode::AbsoluteY => {
                let lo = self.get_next_byte(mem);
                let hi = self.get_next_byte(mem);
                let addr_base = ((hi as u16) << 8) + lo as u16;
                addr_base.wrapping_add(self.reg_y as u16)
            }
            AddressingMode::Indirect => {
                let lo = self.get_next_byte(mem);
                let hi = self.get_next_byte(mem);
                // Note: replicating bug in 6502 where the addresses crossing
                // the page boundary read from the same page instead of the next
                let addr_lo = ((hi as u16) << 8) + lo as u16;
                let addr_hi = ((hi as u16) << 8) + (lo.wrapping_add(1)) as u16;
                let mut addr = mem[addr_lo] as u16;
                addr |= (mem[addr_hi] as u16) << 8;
                addr
            }
            AddressingMode::IndexedIndirect => {
                // Note: address reads from zero page are wrapping
                let zero_page_addr_lo = self.get_next_byte(mem).wrapping_add(self.reg_x);
                let zero_page_addr_hi = zero_page_addr_lo.wrapping_add(1);
                let mut addr = mem[zero_page_addr_lo as u16] as u16;
                addr |= (mem[zero_page_addr_hi as u16] as u16) << 8;
                addr
            }
            AddressingMode::IndirectIndexed => {
                // Note: address reads from zero page are wrapping
                let zero_page_addr_lo = self.get_next_byte(mem);
                let zero_page_addr_hi = zero_page_addr_lo.wrapping_add(1);
                let mut addr_base = mem[zero_page_addr_lo as u16] as u16;
                addr_base |= (mem[zero_page_addr_hi as u16] as u16) << 8;
                addr_base.wrapping_add(self.reg_y as u16)
            }
        }
    }

    fn get_next_byte(&mut self, mem: &dyn Memory) -> u8 {
        let byte = mem[self.pc];
        self.pc += 1;
        #[cfg(test)]
        {
            self.__insn_bytes_read += 1;
        }
        byte
    }

    fn push_byte(&mut self, mem: &mut dyn Memory, b: u8) {
        mem[0x0100 + self.reg_s as u16] = b;
        self.reg_s = self.reg_s.wrapping_sub(1);
    }

    fn pull_byte(&mut self, mem: &dyn Memory) -> u8 {
        self.reg_s = self.reg_s.wrapping_add(1);
        return mem[0x0100 + self.reg_s as u16];
    }

    fn push_addr(&mut self, mem: &mut dyn Memory, addr: u16) {
        self.push_byte(mem, (addr >> 8) as u8);
        self.push_byte(mem, addr as u8);
    }

    fn pull_addr(&mut self, mem: &dyn Memory) -> u16 {
        let mut addr = self.pull_byte(mem) as u16;
        addr |= (self.pull_byte(mem) as u16) << 8;
        addr
    }

    #[cfg(test)]
    fn __init_checks(&mut self) {
        self.__insn_bytes_read = 0;
        self.__saved_a = self.reg_a;
        self.__saved_x = self.reg_x;
        self.__saved_y = self.reg_y;
        self.__saved_pc = self.pc;
        self.__saved_s = self.reg_s;
        self.__saved_flags = self.pack_flags();
    }

    #[cfg(test)]
    fn __run_checks(&self, bytes: u8, allowed_units: u8, allowed_flags: u8) {
        assert_eq!(bytes, self.__insn_bytes_read);

        use crate::cpu::rp2a03::units;
        if (allowed_units & units::A) == 0 {
            assert_eq!(self.__saved_a, self.reg_a);
        }
        if (allowed_units & units::X) == 0 {
            assert_eq!(self.__saved_x, self.reg_x);
        }
        if (allowed_units & units::Y) == 0 {
            assert_eq!(self.__saved_y, self.reg_y);
        }
        if (allowed_units & units::P) == 0 {
            assert_eq!(self.__saved_pc + bytes as u16, self.pc);
        }
        if (allowed_units & units::S) == 0 {
            assert_eq!(self.__saved_s, self.reg_s);
        }
        // TODO: Add a check for units::M

        let current_flags = self.pack_flags();
        let change = self.__saved_flags ^ current_flags;
        assert_eq!(
            0,
            change & !allowed_flags,
            "Unexpected flags have been modified"
        );
    }
}
