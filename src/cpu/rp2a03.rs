// Ricoh 2A03 information (MOS 6502 instruction set)

pub mod units {
    pub const A: u8 = 0x01; // Accumulator
    pub const X: u8 = 0x02; // Register X
    pub const Y: u8 = 0x04; // Register Y
    pub const P: u8 = 0x08; // Program counter
    pub const S: u8 = 0x10; // Stack pointer
    pub const M: u8 = 0x20; // Memory
}

pub mod flags {
    pub const C: u8 = 0x01; // Carry
    pub const Z: u8 = 0x02; // Zero
    pub const I: u8 = 0x04; // Interrupt disable
    pub const D: u8 = 0x08; // Decimal mode (unsupported by HW)
    pub const B: u8 = 0x10; // Break command
                            // No bit 5 (always true)
    pub const V: u8 = 0x40; // Overflow
    pub const N: u8 = 0x80; // Negative
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    ILL, // Illegal
    ADC, // Add with carry
    AND, // Logical AND
    ASL, // Arithmetic shift left
    BCC, // Branch if carry clear
    BCS, // Branch if carry set
    BEQ, // Branch if equal
    BIT, // Bit test
    BMI, // Branch if minus
    BNE, // Branch if not equal
    BPL, // Branch if positive
    BRK, // Force interrupt
    BVC, // Branch if overflow clear
    BVS, // Branch if overflow set
    CLC, // Clear carry flag
    CLD, // Clear decimal mode
    CLI, // Clear interrupt disable
    CLV, // Clear overflow flag
    CMP, // Compare
    CPX, // Compare X register
    CPY, // Compare Y register
    DEC, // Decrement memory
    DEX, // Decrement X register
    DEY, // Decrement Y register
    EOR, // Exclusive OR
    INC, // Increment memory
    INX, // Increment IndexedIndirect register
    INY, // Increment IndirectIndexed register
    JMP, // Jump
    JSR, // Jump to subroutine
    LDA, // Load accumulator
    LDX, // Load X register
    LDY, // Load Y register
    LSR, // Logical shift right
    NOP, // No operation
    ORA, // Logical OR
    PHA, // Push accumulator
    PHP, // Push processor status flags
    PLA, // Pull accumulator
    PLP, // Pull processor status flags
    ROL, // Rotate left
    ROR, // Rotate right
    RTI, // Return from interrupt
    RTS, // Return from subroutine
    SBC, // Subtract with carry
    SEC, // Set carry flag
    SED, // Set decimal mode
    SEI, // Set interrupt disable
    STA, // Store accumulator
    STX, // Store X register
    STY, // Store Y register
    TAX, // Transfer accumulator to X
    TAY, // Transfer accumulator to Y
    TSX, // Transfer stack pointer to X
    TXA, // Transfer X to accumulator
    TXS, // Transfer X to stack pointer
    TYA, // Transfer Y to accumulator

    // ----- Unofficial opcodes -----

    SLO, // Shift left and OR
    RLA, // Rotate left and AND
    SRE, // Shift right and XOR
    RRA, // Rotate right and add
    SAX, // Store A & X
    LAX, // Load accumulator and X
    DCP, // Decrement and compare
    ISB, // Increment and subtract
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressingMode {
    Implicit,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndexedIndirect,
    IndirectIndexed,
}

#[derive(Debug)]
pub struct Info {
    pub opcode: u8,                 // Actual opcode value
    pub insn: Instruction,          // Decoded logical instruction
    pub addressing: AddressingMode, // Decoded addressing mode for the instruction
    pub bytes: u8,                  // Number of bytes taken by the instruction (including opcode)
    pub cycles: u8,                 // Number of cycles it takes to execute
    pub affected_units: u8,         // Units it can affect (for PC it means apart from normal insn parsing)
    pub affected_flags: u8,         // Flags it can affect
    pub name: &'static str,         // Opcode name as a string
}

macro_rules! opcodes {
    ($(
        $opcode: ident,
        $insn: tt,
        $addressing: tt,
        $value: literal,
        $bytes: literal,
        $cycles: literal,
        [ $($affected_units: tt)* ],
        [ $($affected_flags: tt)* ]
    ),* $(,)?) => {
        pub mod opcodes {
            $(
                #[allow(dead_code)]
                pub const $opcode: u8 = $value;
            )*

            lazy_static::lazy_static! {
                pub static ref OPCODES: Vec<u8> = vec![$($value,)*];
            }
        }
        pub mod info {
            use $crate::cpu::rp2a03::Info;
            $(
                pub const $opcode: Info = Info {
                    opcode: $value,
                    insn: $crate::cpu::rp2a03::Instruction::$insn,
                    addressing: $crate::cpu::rp2a03::AddressingMode::$addressing,
                    bytes: $bytes,
                    cycles: $cycles,
                    affected_units: 0x00 $(| $crate::cpu::rp2a03::units::$affected_units)*,
                    affected_flags: 0x00 $(| $crate::cpu::rp2a03::flags::$affected_flags)*,
                    name: stringify!($opcode),
                };
            )*

            lazy_static::lazy_static! {
                pub static ref INFO: Vec<Info> = {
                    let mut result = vec![];
                    for i in 0..=0xffu8 {
                        result.push(Info {
                            opcode: i,
                            insn: $crate::cpu::rp2a03::Instruction::ILL,
                            addressing: $crate::cpu::rp2a03::AddressingMode::Implicit,
                            bytes: 0,
                            cycles: 0,
                            affected_units: 0,
                            affected_flags: 0,
                            name: "ILL",
                        });
                    }
                    $(
                        result[$value] = $opcode;
                    )*
                    result
                };
            }
        }
    };
}

// TODO: handle +X cycles
opcodes! {
    // Opcode-Instruction-Addressing-Value-Bytes-Cycles-Affected flags

    // ADC - Add with carry
    ADC_IMM,    ADC,    Immediate,          0x69,   2,  2,  [A],        [C Z V N],
    ADC_ZPG,    ADC,    ZeroPage,           0x65,   2,  3,  [A],        [C Z V N],
    ADC_ZPX,    ADC,    ZeroPageX,          0x75,   2,  4,  [A],        [C Z V N],
    ADC_ABS,    ADC,    Absolute,           0x6d,   3,  4,  [A],        [C Z V N],
    ADC_ABX,    ADC,    AbsoluteX,          0x7d,   3,  4,  [A],        [C Z V N],
    ADC_ABY,    ADC,    AbsoluteY,          0x79,   3,  4,  [A],        [C Z V N],
    ADC_INX,    ADC,    IndexedIndirect,    0x61,   2,  6,  [A],        [C Z V N],
    ADC_INY,    ADC,    IndirectIndexed,    0x71,   2,  5,  [A],        [C Z V N],

    // AND - Logical AND
    AND_IMM,    AND,    Immediate,          0x29,   2,  2,  [A],        [Z N],
    AND_ZPG,    AND,    ZeroPage,           0x25,   2,  3,  [A],        [Z N],
    AND_ZPX,    AND,    ZeroPageX,          0x35,   2,  4,  [A],        [Z N],
    AND_ABS,    AND,    Absolute,           0x2d,   3,  4,  [A],        [Z N],
    AND_ABX,    AND,    AbsoluteX,          0x3d,   3,  4,  [A],        [Z N],
    AND_ABY,    AND,    AbsoluteY,          0x39,   3,  4,  [A],        [Z N],
    AND_INX,    AND,    IndexedIndirect,    0x21,   2,  6,  [A],        [Z N],
    AND_INY,    AND,    IndirectIndexed,    0x31,   2,  5,  [A],        [Z N],

    // ASL - Arithmetic shift left
    ASL_ACC,    ASL,    Accumulator,        0x0a,   1,  2,  [A M],      [C Z N],
    ASL_ZPG,    ASL,    ZeroPage,           0x06,   2,  5,  [A M],      [C Z N],
    ASL_ZPX,    ASL,    ZeroPageX,          0x16,   2,  6,  [A M],      [C Z N],
    ASL_ABS,    ASL,    Absolute,           0x0e,   3,  6,  [A M],      [C Z N],
    ASL_ABX,    ASL,    AbsoluteX,          0x1e,   3,  7,  [A M],      [C Z N],

    // BCC - Branch if carry clear
    BCC_REL,    BCC,    Relative,           0x90,   2,  2,  [P],        [],

    // BCS - Branch if carry set
    BCS_REL,    BCS,    Relative,           0xb0,   2,  2,  [P],        [],

    // BEQ - Branch if equal
    BEQ_REL,    BEQ,    Relative,           0xf0,   2,  2,  [P],        [],

    // BIT - Bit test
    BIT_ZPG,    BIT,    ZeroPage,           0x24,   2,  3,  [],         [Z V N],
    BIT_ABS,    BIT,    Absolute,           0x2c,   3,  4,  [],         [Z V N],

    // BMI - Branch if minus
    BMI_REL,    BMI,    Relative,           0x30,   2,  2,  [P],        [],

    // BNE - Branch if not equal
    BNE_REL,    BNE,    Relative,           0xd0,   2,  2,  [P],        [],

    // BPL - Branch if positive
    BPL_REL,    BPL,    Relative,           0x10,   2,  2,  [P],        [],

    // BRK - Force interrupt
    BRK_IMP,    BRK,    Implicit,           0x00,   1,  7,  [P S M],    [B],

    // BVC - Branch if overflow clear
    BVC_REL,    BVC,    Relative,           0x50,   2,  2,  [P],        [],

    // BVS - Branch if overflow set
    BVS_REL,    BVS,    Relative,           0x70,   2,  2,  [P],        [],

    // CLC - Clear carry flag
    CLC_IMP,    CLC,    Implicit,           0x18,   1,  2,  [],         [C],

    // CLD - Clear decimal mode
    CLD_IMP,    CLD,    Implicit,           0xd8,   1,  2,  [],         [D],

    // CLI - Clear interrupt disable
    CLI_IMP,    CLI,    Implicit,           0x58,   1,  2,  [],         [I],

    // CLV - Clear overflow flag
    CLV_IMP,    CLV,    Implicit,           0xb8,   1,  2,  [],         [V],

    // CMP - Compare
    CMP_IMM,    CMP,    Immediate,          0xc9,   2,  2,  [],         [C Z N],
    CMP_ZPG,    CMP,    ZeroPage,           0xc5,   2,  3,  [],         [C Z N],
    CMP_ZPX,    CMP,    ZeroPageX,          0xd5,   2,  4,  [],         [C Z N],
    CMP_ABS,    CMP,    Absolute,           0xcd,   3,  4,  [],         [C Z N],
    CMP_ABX,    CMP,    AbsoluteX,          0xdd,   3,  4,  [],         [C Z N],
    CMP_ABY,    CMP,    AbsoluteY,          0xd9,   3,  4,  [],         [C Z N],
    CMP_INX,    CMP,    IndexedIndirect,    0xc1,   2,  6,  [],         [C Z N],
    CMP_INY,    CMP,    IndirectIndexed,    0xd1,   2,  5,  [],         [C Z N],

    // CPX - Compare X register
    CPX_IMM,    CPX,    Immediate,          0xe0,   2,  2,  [],         [C Z N],
    CPX_ZPG,    CPX,    ZeroPage,           0xe4,   2,  3,  [],         [C Z N],
    CPX_ABS,    CPX,    Absolute,           0xec,   3,  4,  [],         [C Z N],

    // CPY - Compare Y register
    CPY_IMM,    CPY,    Immediate,          0xc0,   2,  2,  [],         [C Z N],
    CPY_ZPG,    CPY,    ZeroPage,           0xc4,   2,  3,  [],         [C Z N],
    CPY_ABS,    CPY,    Absolute,           0xcc,   3,  4,  [],         [C Z N],

    // DEC - Decrement memory
    DEC_ZPG,    DEC,    ZeroPage,           0xc6,   2,  5,  [M],        [Z N],
    DEC_ZPX,    DEC,    ZeroPageX,          0xd6,   2,  6,  [M],        [Z N],
    DEC_ABS,    DEC,    Absolute,           0xce,   3,  6,  [M],        [Z N],
    DEC_ABX,    DEC,    AbsoluteX,          0xde,   3,  7,  [M],        [Z N],

    // DEX - Decrement X register
    DEX_IMP,    DEX,    Implicit,           0xca,   1,  2,  [X],        [Z N],

    // DEY - Decrement Y register
    DEY_IMP,    DEY,    Implicit,           0x88,   1,  2,  [Y],        [Z N],

    // EOR - Exclusive OR
    EOR_IMM,    EOR,    Immediate,          0x49,   2,  2,  [A],        [Z N],
    EOR_ZPG,    EOR,    ZeroPage,           0x45,   2,  3,  [A],        [Z N],
    EOR_ZPX,    EOR,    ZeroPageX,          0x55,   2,  4,  [A],        [Z N],
    EOR_ABS,    EOR,    Absolute,           0x4d,   3,  4,  [A],        [Z N],
    EOR_ABX,    EOR,    AbsoluteX,          0x5d,   3,  4,  [A],        [Z N],
    EOR_ABY,    EOR,    AbsoluteY,          0x59,   3,  4,  [A],        [Z N],
    EOR_INX,    EOR,    IndexedIndirect,    0x41,   2,  6,  [A],        [Z N],
    EOR_INY,    EOR,    IndirectIndexed,    0x51,   2,  5,  [A],        [Z N],

    // INC - Increment memory
    INC_ZPG,    INC,    ZeroPage,           0xe6,   2,  5,  [M],        [Z N],
    INC_ZPX,    INC,    ZeroPageX,          0xf6,   2,  6,  [M],        [Z N],
    INC_ABS,    INC,    Absolute,           0xee,   3,  6,  [M],        [Z N],
    INC_ABX,    INC,    AbsoluteX,          0xfe,   3,  7,  [M],        [Z N],

    // INX - Increment X register
    INX_IMP,    INX,    Implicit,           0xe8,   1,  2,  [X],        [Z N],

    // INY - Increment Y register
    INY_IMP,    INY,    Implicit,           0xc8,   1,  2,  [Y],        [Z N],

    // JMP - Jump
    JMP_ABS,    JMP,    Absolute,           0x4c,   3,  3,  [P],        [],
    JMP_IND,    JMP,    Indirect,           0x6c,   3,  5,  [P],        [],

    // JSR - Jump to subroutine
    JSR_ABS,    JSR,    Absolute,           0x20,   3,  6,  [P S M],    [],

    // LDA - Load accumulator
    LDA_IMM,    LDA,    Immediate,          0xa9,   2,  2,  [A],        [Z N],
    LDA_ZPG,    LDA,    ZeroPage,           0xa5,   2,  3,  [A],        [Z N],
    LDA_ZPX,    LDA,    ZeroPageX,          0xb5,   2,  4,  [A],        [Z N],
    LDA_ABS,    LDA,    Absolute,           0xad,   3,  4,  [A],        [Z N],
    LDA_ABX,    LDA,    AbsoluteX,          0xbd,   3,  4,  [A],        [Z N],
    LDA_ABY,    LDA,    AbsoluteY,          0xb9,   3,  4,  [A],        [Z N],
    LDA_INX,    LDA,    IndexedIndirect,    0xa1,   2,  6,  [A],        [Z N],
    LDA_INY,    LDA,    IndirectIndexed,    0xb1,   2,  5,  [A],        [Z N],

    // LDX - Load X register
    LDX_IMM,    LDX,    Immediate,          0xa2,   2,  2,  [X],        [Z N],
    LDX_ZPG,    LDX,    ZeroPage,           0xa6,   2,  3,  [X],        [Z N],
    LDX_ZPY,    LDX,    ZeroPageY,          0xb6,   2,  4,  [X],        [Z N],
    LDX_ABS,    LDX,    Absolute,           0xae,   3,  4,  [X],        [Z N],
    LDX_ABY,    LDX,    AbsoluteY,          0xbe,   3,  4,  [X],        [Z N],

    // LDY - Load Y register
    LDY_IMM,    LDY,    Immediate,          0xa0,   2,  2,  [Y],        [Z N],
    LDY_ZPG,    LDY,    ZeroPage,           0xa4,   2,  3,  [Y],        [Z N],
    LDY_ZPX,    LDY,    ZeroPageX,          0xb4,   2,  4,  [Y],        [Z N],
    LDY_ABS,    LDY,    Absolute,           0xac,   3,  4,  [Y],        [Z N],
    LDY_ABX,    LDY,    AbsoluteX,          0xbc,   3,  4,  [Y],        [Z N],

    // LSR - Logical shift right
    LSR_ACC,    LSR,    Accumulator,        0x4a,   1,  2,  [A M],      [C Z N],
    LSR_ZPG,    LSR,    ZeroPage,           0x46,   2,  5,  [A M],      [C Z N],
    LSR_ZPX,    LSR,    ZeroPageX,          0x56,   2,  6,  [A M],      [C Z N],
    LSR_ABS,    LSR,    Absolute,           0x4e,   3,  6,  [A M],      [C Z N],
    LSR_ABX,    LSR,    AbsoluteX,          0x5e,   3,  7,  [A M],      [C Z N],

    // NOP - No operation
    NOP_IMP,    NOP,    Implicit,           0xea,   1,  2,  [],         [],

    // ORA - Logical OR
    ORA_IMM,    ORA,    Immediate,          0x09,   2,  2,  [A],        [Z N],
    ORA_ZPG,    ORA,    ZeroPage,           0x05,   2,  3,  [A],        [Z N],
    ORA_ZPX,    ORA,    ZeroPageX,          0x15,   2,  4,  [A],        [Z N],
    ORA_ABS,    ORA,    Absolute,           0x0d,   3,  4,  [A],        [Z N],
    ORA_ABX,    ORA,    AbsoluteX,          0x1d,   3,  4,  [A],        [Z N],
    ORA_ABY,    ORA,    AbsoluteY,          0x19,   3,  4,  [A],        [Z N],
    ORA_INX,    ORA,    IndexedIndirect,    0x01,   2,  6,  [A],        [Z N],
    ORA_INY,    ORA,    IndirectIndexed,    0x11,   2,  5,  [A],        [Z N],

    // PHA - Push accumulator
    PHA_IMP,    PHA,    Implicit,           0x48,   1,  3,  [S M],      [],

    // PHP - Push processor status flags
    PHP_IMP,    PHP,    Implicit,           0x08,   1,  3,  [S M],      [],

    // PLA - Pull accumulator
    PLA_IMP,    PLA,    Implicit,           0x68,   1,  4,  [S A],      [Z N],

    // PLP - Pull processor status flags
    PLP_IMP,    PLP,    Implicit,           0x28,   1,  4,  [S],        [C Z I D B V N],

    // ROL - Rotate left
    ROL_ACC,    ROL,    Accumulator,        0x2a,   1,  2,  [A M],      [C Z N],
    ROL_ZPG,    ROL,    ZeroPage,           0x26,   2,  5,  [A M],      [C Z N],
    ROL_ZPX,    ROL,    ZeroPageX,          0x36,   2,  6,  [A M],      [C Z N],
    ROL_ABS,    ROL,    Absolute,           0x2e,   3,  6,  [A M],      [C Z N],
    ROL_ABX,    ROL,    AbsoluteX,          0x3e,   3,  7,  [A M],      [C Z N],

    // ROR - Rotate right
    ROR_ACC,    ROR,    Accumulator,        0x6a,   1,  2,  [A M],      [C Z N],
    ROR_ZPG,    ROR,    ZeroPage,           0x66,   2,  5,  [A M],      [C Z N],
    ROR_ZPX,    ROR,    ZeroPageX,          0x76,   2,  6,  [A M],      [C Z N],
    ROR_ABS,    ROR,    Absolute,           0x6e,   3,  6,  [A M],      [C Z N],
    ROR_ABX,    ROR,    AbsoluteX,          0x7e,   3,  7,  [A M],      [C Z N],

    // RTI - Return from interrupt
    RTI_IMP,    RTI,    Implicit,           0x40,   1,  6,  [P S],      [C Z I D B V N],

    // RTS - Return from subroutine
    RTS_IMP,    RTS,    Implicit,           0x60,   1,  6,  [P S],      [],

    // SBC - Subtract with carry
    SBC_IMM,    SBC,    Immediate,          0xe9,   2,  2,  [A],        [C Z V N],
    SBC_ZPG,    SBC,    ZeroPage,           0xe5,   2,  3,  [A],        [C Z V N],
    SBC_ZPX,    SBC,    ZeroPageX,          0xf5,   2,  4,  [A],        [C Z V N],
    SBC_ABS,    SBC,    Absolute,           0xed,   3,  4,  [A],        [C Z V N],
    SBC_ABX,    SBC,    AbsoluteX,          0xfd,   3,  4,  [A],        [C Z V N],
    SBC_ABY,    SBC,    AbsoluteY,          0xf9,   3,  4,  [A],        [C Z V N],
    SBC_INX,    SBC,    IndexedIndirect,    0xe1,   2,  6,  [A],        [C Z V N],
    SBC_INY,    SBC,    IndirectIndexed,    0xf1,   2,  5,  [A],        [C Z V N],

    // SEC - Set carry flag
    SEC_IMP,    SEC,    Implicit,           0x38,   1,  2,  [],         [C],

    // SED - Set decimal mode
    SED_IMP,    SED,    Implicit,           0xf8,   1,  2,  [],         [D],

    // SEI - Set interrupt disable
    SEI_IMP,    SEI,    Implicit,           0x78,   1,  2,  [],         [I],

    // STA - Store accumulator
    STA_ZPG,    STA,    ZeroPage,           0x85,   2,  3,  [M],        [],
    STA_ZPX,    STA,    ZeroPageX,          0x95,   2,  4,  [M],        [],
    STA_ABS,    STA,    Absolute,           0x8d,   3,  4,  [M],        [],
    STA_ABX,    STA,    AbsoluteX,          0x9d,   3,  5,  [M],        [],
    STA_ABY,    STA,    AbsoluteY,          0x99,   3,  5,  [M],        [],
    STA_INX,    STA,    IndexedIndirect,    0x81,   2,  6,  [M],        [],
    STA_INY,    STA,    IndirectIndexed,    0x91,   2,  6,  [M],        [],

    // STX - Store X register
    STX_ZPG,    STX,    ZeroPage,           0x86,   2,  3,  [M],        [],
    STX_ZPY,    STX,    ZeroPageY,          0x96,   2,  4,  [M],        [],
    STX_ABS,    STX,    Absolute,           0x8e,   3,  4,  [M],        [],

    // STY - Store Y register
    STY_ZPG,    STY,    ZeroPage,           0x84,   2,  3,  [M],        [],
    STY_ZPX,    STY,    ZeroPageX,          0x94,   2,  4,  [M],        [],
    STY_ABS,    STY,    Absolute,           0x8c,   3,  4,  [M],        [],

    // TAX - Transfer accumulator to X
    TAX_IMP,    TAX,    Implicit,           0xaa,   1,  2,  [X],        [Z N],

    // TAY - Transfer accumulator to Y
    TAY_IMP,    TAY,    Implicit,           0xa8,   1,  2,  [Y],        [Z N],

    // TSX - Transfer stack pointer to X
    TSX_IMP,    TSX,    Implicit,           0xba,   1,  2,  [X],        [Z N],

    // TXA - Transfer X to accumulator
    TXA_IMP,    TXA,    Implicit,           0x8a,   1,  2,  [A],        [Z N],

    // TXS - Transfer X to stack pointer
    TXS_IMP,    TXS,    Implicit,           0x9a,   1,  2,  [S],        [],

    // TYA - Transfer Y to accumulator
    TYA_IMP,    TYA,    Implicit,           0x98,   1,  2,  [A],        [Z N],

    // -------------------- Unofficial instructions below --------------------
    // See: http://www.oxyron.de/html/opcodes02.html

    // NOP - No operation (unofficial multi-byte and multi-cycle variants)
    NOP_IP0,    NOP,    Implicit,           0x1a,   1,  2,  [],         [],
    NOP_IP1,    NOP,    Implicit,           0x3a,   1,  2,  [],         [],
    NOP_IP2,    NOP,    Implicit,           0x5a,   1,  2,  [],         [],
    NOP_IP3,    NOP,    Implicit,           0x7a,   1,  2,  [],         [],
    NOP_IP4,    NOP,    Implicit,           0xda,   1,  2,  [],         [],
    NOP_IP5,    NOP,    Implicit,           0xfa,   1,  2,  [],         [],
    NOP_IM0,    NOP,    Immediate,          0x80,   2,  2,  [],         [],
    NOP_IM1,    NOP,    Immediate,          0x82,   2,  2,  [],         [],
    NOP_IM2,    NOP,    Immediate,          0x89,   2,  2,  [],         [],
    NOP_IM3,    NOP,    Immediate,          0xc2,   2,  2,  [],         [],
    NOP_IM4,    NOP,    Immediate,          0xe2,   2,  2,  [],         [],
    NOP_ZP0,    NOP,    ZeroPage,           0x04,   2,  3,  [],         [],
    NOP_ZP1,    NOP,    ZeroPage,           0x44,   2,  3,  [],         [],
    NOP_ZP2,    NOP,    ZeroPage,           0x64,   2,  3,  [],         [],
    NOP_ZX0,    NOP,    ZeroPageX,          0x14,   2,  4,  [],         [],
    NOP_ZX1,    NOP,    ZeroPageX,          0x34,   2,  4,  [],         [],
    NOP_ZX2,    NOP,    ZeroPageX,          0x54,   2,  4,  [],         [],
    NOP_ZX3,    NOP,    ZeroPageX,          0x74,   2,  4,  [],         [],
    NOP_ZX4,    NOP,    ZeroPageX,          0xd4,   2,  4,  [],         [],
    NOP_ZX5,    NOP,    ZeroPageX,          0xf4,   2,  4,  [],         [],
    NOP_AB0,    NOP,    Absolute,           0x0c,   3,  4,  [],         [],
    NOP_AX0,    NOP,    AbsoluteX,          0x1c,   3,  4,  [],         [],
    NOP_AX1,    NOP,    AbsoluteX,          0x3c,   3,  4,  [],         [],
    NOP_AX2,    NOP,    AbsoluteX,          0x5c,   3,  4,  [],         [],
    NOP_AX3,    NOP,    AbsoluteX,          0x7c,   3,  4,  [],         [],
    NOP_AX4,    NOP,    AbsoluteX,          0xdc,   3,  4,  [],         [],
    NOP_AX5,    NOP,    AbsoluteX,          0xfc,   3,  4,  [],         [],

    // SLO - Shift left and OR
    SLO_ZPG,    SLO,    ZeroPage,           0x07,   2,  5,  [A M],      [C Z N],
    SLO_ZPX,    SLO,    ZeroPageX,          0x17,   2,  6,  [A M],      [C Z N],
    SLO_ABS,    SLO,    Absolute,           0x0f,   3,  6,  [A M],      [C Z N],
    SLO_ABX,    SLO,    AbsoluteX,          0x1f,   3,  7,  [A M],      [C Z N],
    SLO_ABY,    SLO,    AbsoluteY,          0x1b,   3,  7,  [A M],      [C Z N],
    SLO_INX,    SLO,    IndexedIndirect,    0x03,   2,  8,  [A M],      [C Z N],
    SLO_INY,    SLO,    IndirectIndexed,    0x13,   2,  8,  [A M],      [C Z N],

    // RLA - Rotate left and AND
    RLA_ZPG,    RLA,    ZeroPage,           0x27,   2,  5,  [A M],      [C Z N],
    RLA_ZPX,    RLA,    ZeroPageX,          0x37,   2,  6,  [A M],      [C Z N],
    RLA_ABS,    RLA,    Absolute,           0x2f,   3,  6,  [A M],      [C Z N],
    RLA_ABX,    RLA,    AbsoluteX,          0x3f,   3,  7,  [A M],      [C Z N],
    RLA_ABY,    RLA,    AbsoluteY,          0x3b,   3,  7,  [A M],      [C Z N],
    RLA_INX,    RLA,    IndexedIndirect,    0x23,   2,  8,  [A M],      [C Z N],
    RLA_INY,    RLA,    IndirectIndexed,    0x33,   2,  8,  [A M],      [C Z N],

    // SRE - Shift right and XOR
    SRE_ZPG,    SRE,    ZeroPage,           0x47,   2,  5,  [A M],      [C Z N],
    SRE_ZPX,    SRE,    ZeroPageX,          0x57,   2,  6,  [A M],      [C Z N],
    SRE_ABS,    SRE,    Absolute,           0x4f,   3,  6,  [A M],      [C Z N],
    SRE_ABX,    SRE,    AbsoluteX,          0x5f,   3,  7,  [A M],      [C Z N],
    SRE_ABY,    SRE,    AbsoluteY,          0x5b,   3,  7,  [A M],      [C Z N],
    SRE_INX,    SRE,    IndexedIndirect,    0x43,   2,  8,  [A M],      [C Z N],
    SRE_INY,    SRE,    IndirectIndexed,    0x53,   2,  8,  [A M],      [C Z N],

    // RRA - Rotate right and add
    RRA_ZPG,    RRA,    ZeroPage,           0x67,   2,  5,  [A M],      [C Z V N],
    RRA_ZPX,    RRA,    ZeroPageX,          0x77,   2,  6,  [A M],      [C Z V N],
    RRA_ABS,    RRA,    Absolute,           0x6f,   3,  6,  [A M],      [C Z V N],
    RRA_ABX,    RRA,    AbsoluteX,          0x7f,   3,  7,  [A M],      [C Z V N],
    RRA_ABY,    RRA,    AbsoluteY,          0x7b,   3,  7,  [A M],      [C Z V N],
    RRA_INX,    RRA,    IndexedIndirect,    0x63,   2,  8,  [A M],      [C Z V N],
    RRA_INY,    RRA,    IndirectIndexed,    0x73,   2,  8,  [A M],      [C Z V N],

    // SAX - Store A & X
    SAX_ZPG,    SAX,    ZeroPage,           0x87,   2,  3,  [M],        [],
    SAX_ZPY,    SAX,    ZeroPageY,          0x97,   2,  4,  [M],        [],
    SAX_ABS,    SAX,    Absolute,           0x8f,   3,  4,  [M],        [],
    SAX_INX,    SAX,    IndexedIndirect,    0x83,   2,  6,  [M],        [],

    // LAX - Load accumulator and X
    LAX_IMM,    LAX,    Immediate,          0xab,   2,  2,  [A X],      [Z N], // UNSTABLE
    LAX_ZPG,    LAX,    ZeroPage,           0xa7,   2,  3,  [A X],      [Z N],
    LAX_ZPY,    LAX,    ZeroPageY,          0xb7,   2,  4,  [A X],      [Z N],
    LAX_ABS,    LAX,    Absolute,           0xaf,   3,  4,  [A X],      [Z N],
    LAX_ABY,    LAX,    AbsoluteY,          0xbf,   3,  4,  [A X],      [Z N],
    LAX_INX,    LAX,    IndexedIndirect,    0xa3,   2,  6,  [A X],      [Z N],
    LAX_INY,    LAX,    IndirectIndexed,    0xb3,   2,  5,  [A X],      [Z N],

    // DCP - Decrement and compare
    DCP_ZPG,    DCP,    ZeroPage,           0xc7,   2,  5,  [M],        [C Z N],
    DCP_ZPX,    DCP,    ZeroPageX,          0xd7,   2,  6,  [M],        [C Z N],
    DCP_ABS,    DCP,    Absolute,           0xcf,   3,  6,  [M],        [C Z N],
    DCP_ABX,    DCP,    AbsoluteX,          0xdf,   3,  7,  [M],        [C Z N],
    DCP_ABY,    DCP,    AbsoluteY,          0xdb,   3,  7,  [M],        [C Z N],
    DCP_INX,    DCP,    IndexedIndirect,    0xc3,   2,  8,  [M],        [C Z N],
    DCP_INY,    DCP,    IndirectIndexed,    0xd3,   2,  8,  [M],        [C Z N],

    // ISB - Increment and subtract
    ISB_ZPG,    ISB,    ZeroPage,           0xe7,   2,  5,  [A M],      [C Z V N],
    ISB_ZPX,    ISB,    ZeroPageX,          0xf7,   2,  6,  [A M],      [C Z V N],
    ISB_ABS,    ISB,    Absolute,           0xef,   3,  6,  [A M],      [C Z V N],
    ISB_ABX,    ISB,    AbsoluteX,          0xff,   3,  7,  [A M],      [C Z V N],
    ISB_ABY,    ISB,    AbsoluteY,          0xfb,   3,  7,  [A M],      [C Z V N],
    ISB_INX,    ISB,    IndexedIndirect,    0xe3,   2,  8,  [A M],      [C Z V N],
    ISB_INY,    ISB,    IndirectIndexed,    0xf3,   2,  8,  [A M],      [C Z V N],

    // SBC - Subtract with carry (unofficial variant)
    SBC_IM0,    SBC,    Immediate,          0xeb,   2,  2,  [A],        [C Z V N],
}
