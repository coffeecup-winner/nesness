// Ricoh 2A03 information (MOS 6502 instruction set)

pub mod flags {
    // TODO: use the correct bits
    pub const C: u8 = 0x01; // Carry
    pub const Z: u8 = 0x02; // Zero
    pub const I: u8 = 0x04; // Interrupt disable
                            // No D flag
    pub const B: u8 = 0x08; // Break command
    pub const V: u8 = 0x10; // Overflow
    pub const N: u8 = 0x20; // Negative
}

pub struct Info {
    pub opcode: u8,
    pub bytes: u8,
    pub cycles: u8,
    pub affected_flags: u8,
    pub name: &'static str,
}

macro_rules! opcodes {
    ($($opcode: ident, $value: literal, $bytes: literal, $cycles: literal, [ $($affected_flags: tt)* ]),* $(,)?) => {
        pub mod opcodes {
            $(
                #[allow(dead_code)]
                pub const $opcode: u8 = $value;
            )*

            lazy_static::lazy_static! {
                pub static ref OPCODES: Vec<u8> = {
                    let mut result = vec![];
                    $(
                        result.push($value);
                    )*
                    result
                };
            }
        }
        pub mod info {
            use $crate::rp2a03::Info;
            $(
                #[allow(dead_code)]
                pub const $opcode: Info = Info {
                    opcode: $value,
                    bytes: $bytes,
                    cycles: $cycles,
                    affected_flags: 0x00 $(| $crate::rp2a03::flags::$affected_flags)*,
                    name: stringify!($opcode),
                };
            )*

            lazy_static::lazy_static! {
                pub static ref INFO: Vec<Info> = {
                    let mut result = vec![];
                    for i in 0..0xffu8 {
                        result.push(Info {
                            opcode: i,
                            bytes: 0,
                            cycles: 0,
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
    // Name-Opcode-Bytes-Cycles-Affected flags

    // ADC - Add with carry
    ADC_IMM,    0x69,   2,  2,  [C Z V N],
    ADC_ZPG,    0x65,   2,  3,  [C Z V N],
    ADC_ZPX,    0x75,   2,  4,  [C Z V N],
    ADC_ABS,    0x6d,   3,  4,  [C Z V N],
    ADC_ABX,    0x7d,   3,  4,  [C Z V N],
    ADC_ABY,    0x79,   3,  4,  [C Z V N],
    ADC_INX,    0x61,   2,  6,  [C Z V N],
    ADC_INY,    0x71,   2,  5,  [C Z V N],

    // AND - Logical AND
    AND_IMM,    0x29,   2,  2,  [Z N],
    AND_ZPG,    0x25,   2,  3,  [Z N],
    AND_ZPX,    0x35,   2,  4,  [Z N],
    AND_ABS,    0x2d,   3,  4,  [Z N],
    AND_ABX,    0x3d,   3,  4,  [Z N],
    AND_ABY,    0x39,   3,  4,  [Z N],
    AND_INX,    0x21,   2,  6,  [Z N],
    AND_INY,    0x31,   2,  5,  [Z N],

    // ASL - Arithmetic shift left
    ASL_ACC,    0x0a,   1,  2,  [C Z N],
    ASL_ZPG,    0x06,   2,  5,  [C Z N],
    ASL_ZPX,    0x16,   2,  6,  [C Z N],
    ASL_ABS,    0x0e,   3,  6,  [C Z N],
    ASL_ABX,    0x1e,   3,  7,  [C Z N],

    // BCC - Branch if carry clear
    BCC_REL,    0x90,   2,  2,  [],

    // BCS - Branch if carry set
    BCS_REL,    0xb0,   2,  2,  [],

    // BEQ - Branch if equal
    BEQ_REL,    0xf0,   2,  2,  [],

    // BIT - Bit test
    BIT_ZPG,    0x24,   2,  3,  [Z V N],
    BIT_ABS,    0x2c,   3,  4,  [Z V N],

    // BMI - Branch if minus
    BMI_REL,    0x30,   2,  2,  [],

    // BNE - Branch if not equal
    BNE_REL,    0xd0,   2,  2,  [],

    // BPL - Branch if positive
    BPL_REL,    0x10,   2,  2,  [],

    // BRK - Force interrupt
    BRK_IMP,    0x00,   1,  7,  [B],

    // BVC - Branch if overflow clear
    BVC_REL,    0x50,   2,  2,  [],

    // BVS - Branch if overflow set
    BVS_REL,    0x70,   2,  2,  [],

    // CLC - Clear carry flag
    CLC_IMP,    0x18,   1,  2,  [C],

    // CLD - Clear decimal mode
    // Is not present on 2A03 (0xd8)

    // CLI - Clear interrupt disable
    CLI_IMP,    0x58,   1,  2,  [I],

    // CLV - Clear overflow flag
    CLV_IMP,    0xb8,   1,  2,  [V],

    // CMP - Compare
    CMP_IMM,    0xc9,   2,  2,  [C Z N],
    CMP_ZPG,    0xc5,   2,  3,  [C Z N],
    CMP_ZPX,    0xd5,   2,  4,  [C Z N],
    CMP_ABS,    0xcd,   3,  4,  [C Z N],
    CMP_ABX,    0xdd,   3,  4,  [C Z N],
    CMP_ABY,    0xd9,   3,  4,  [C Z N],
    CMP_INX,    0xc1,   2,  6,  [C Z N],
    CMP_INY,    0xd1,   2,  5,  [C Z N],

    // CPX - Compare X register
    CPX_IMM,    0xe0,   2,  2,  [C Z N],
    CPX_ZPG,    0xe4,   2,  3,  [C Z N],
    CPX_ABS,    0xec,   3,  4,  [C Z N],

    // CPY - Compare Y register
    CPY_IMM,    0xc0,   2,  2,  [C Z N],
    CPY_ZPG,    0xc4,   2,  3,  [C Z N],
    CPY_ABS,    0xcc,   3,  4,  [C Z N],

    // DEC - Decrement memory
    DEC_ZPG,    0xc6,   2,  5,  [Z N],
    DEC_ZPX,    0xd6,   2,  6,  [Z N],
    DEC_ABS,    0xce,   3,  6,  [Z N],
    DEC_ABX,    0xde,   3,  7,  [Z N],

    // DEX - Decrement X register
    DEX_IMP,    0xca,   1,  2,  [Z N],

    // DEY - Decrement Y register
    DEY_IMP,    0x88,   1,  2,  [Z N],

    // EOR - Exclusive OR
    EOR_IMM,    0x49,   2,  2,  [Z N],
    EOR_ZPG,    0x45,   2,  3,  [Z N],
    EOR_ZPX,    0x55,   2,  4,  [Z N],
    EOR_ABS,    0x4d,   3,  4,  [Z N],
    EOR_ABX,    0x5d,   3,  4,  [Z N],
    EOR_ABY,    0x59,   3,  4,  [Z N],
    EOR_INX,    0x41,   2,  6,  [Z N],
    EOR_INY,    0x51,   2,  5,  [Z N],

    // INC - Increment memory
    INC_ZPG,    0xe6,   2,  5,  [Z N],
    INC_ZPX,    0xf6,   2,  6,  [Z N],
    INC_ABS,    0xee,   3,  6,  [Z N],
    INC_ABX,    0xfe,   3,  7,  [Z N],

    // INX - Increment X register
    INX_IMP,    0xe8,   1,  2,  [Z N],

    // INY - Increment Y register
    INY_IMP,    0xc8,   1,  2,  [Z N],

    // JMP - Jump
    JMP_ABS,    0x4c,   3,  3,  [],
    JMP_IND,    0x6c,   3,  5,  [],

    // JSR - Jump to subroutine
    JSR_ABS,    0x20,   3,  6,  [],

    // LDA - Load accumulator
    LDA_IMM,    0xa9,   2,  2,  [Z N],
    LDA_ZPG,    0xa5,   2,  3,  [Z N],
    LDA_ZPX,    0xb5,   2,  4,  [Z N],
    LDA_ABS,    0xad,   3,  4,  [Z N],
    LDA_ABX,    0xbd,   3,  4,  [Z N],
    LDA_ABY,    0xb9,   3,  4,  [Z N],
    LDA_INX,    0xa1,   2,  6,  [Z N],
    LDA_INY,    0xb1,   2,  5,  [Z N],

    // LDX - Load X register
    LDX_IMM,    0xa2,   2,  2,  [Z N],
    LDX_ZPG,    0xa6,   2,  3,  [Z N],
    LDX_ZPY,    0xb6,   2,  4,  [Z N],
    LDX_ABS,    0xae,   3,  4,  [Z N],
    LDX_ABY,    0xbe,   3,  4,  [Z N],

    // LDY - Load Y register
    LDY_IMM,    0xa0,   2,  2,  [Z N],
    LDY_ZPG,    0xa4,   2,  3,  [Z N],
    LDY_ZPX,    0xb4,   2,  4,  [Z N],
    LDY_ABS,    0xac,   3,  4,  [Z N],
    LDY_ABX,    0xbc,   3,  4,  [Z N],

    // LSR - Logical shift right
    LSR_ACC,    0x4a,   1,  2,  [C Z N],
    LSR_ZPG,    0x46,   2,  5,  [C Z N],
    LSR_ZPX,    0x56,   2,  6,  [C Z N],
    LSR_ABS,    0x4e,   3,  6,  [C Z N],
    LSR_ABX,    0x5e,   3,  7,  [C Z N],

    // NOP - No operation
    NOP_IMP,    0xea,   1,  2,  [],

    // ORA - Logical OR
    ORA_IMM,    0x09,   2,  2,  [Z N],
    ORA_ZPG,    0x05,   2,  3,  [Z N],
    ORA_ZPX,    0x15,   2,  4,  [Z N],
    ORA_ABS,    0x0d,   3,  4,  [Z N],
    ORA_ABX,    0x1d,   3,  4,  [Z N],
    ORA_ABY,    0x19,   3,  4,  [Z N],
    ORA_INX,    0x01,   2,  6,  [Z N],
    ORA_INY,    0x11,   2,  5,  [Z N],

    // PHA - Push accumulator
    PHA_IMP,    0x48,   1,  3,  [],

    // PHP - Push processor status flags
    PHP_IMP,    0x08,   1,  3,  [],

    // PLA - Pull accumulator
    PLA_IMP,    0x68,   1,  4,  [Z N],

    // PLP - Pull processor status flags
    PLP_IMP,    0x28,   1,  4,  [C Z I B V N],

    // ROL - Rotate left
    ROL_ACC,    0x2a,   1,  2,  [C Z N],
    ROL_ZPG,    0x26,   2,  5,  [C Z N],
    ROL_ZPX,    0x36,   2,  6,  [C Z N],
    ROL_ABS,    0x2e,   3,  6,  [C Z N],
    ROL_ABX,    0x3e,   3,  7,  [C Z N],

    // ROR - Rotate right
    ROR_ACC,    0x6a,   1,  2,  [C Z N],
    ROR_ZPG,    0x66,   2,  5,  [C Z N],
    ROR_ZPX,    0x76,   2,  6,  [C Z N],
    ROR_ABS,    0x6e,   3,  6,  [C Z N],
    ROR_ABX,    0x7e,   3,  7,  [C Z N],

    // RTI - Return from interrupt
    RTI_IMP,    0x40,   1,  6,  [C Z I B V N],

    // RTS - Return from subroutine
    RTS_IMP,    0x60,   1,  6,  [],

    // SBC - Subtract with carry
    SBC_IMM,    0xe9,   2,  2,  [C Z V N],
    SBC_ZPG,    0xe5,   2,  3,  [C Z V N],
    SBC_ZPX,    0xf5,   2,  4,  [C Z V N],
    SBC_ABS,    0xed,   3,  4,  [C Z V N],
    SBC_ABX,    0xfd,   3,  4,  [C Z V N],
    SBC_ABY,    0xf9,   3,  4,  [C Z V N],
    SBC_INX,    0xe1,   2,  6,  [C Z V N],
    SBC_INY,    0xf1,   2,  5,  [C Z V N],

    // SEC - Set carry flag
    SEC_IMP,    0x38,   1,  2,  [C],

    // SED - Set decimal mode
    // Is not present on 2A03 (0xf8)

    // SEI - Set interrupt disable
    SEI_IMP,    0x78,   1,  2,  [I],

    // STA - Store accumulator
    STA_ZPG,    0x85,   2,  3,  [],
    STA_ZPX,    0x95,   2,  4,  [],
    STA_ABS,    0x8d,   3,  4,  [],
    STA_ABX,    0x9d,   3,  5,  [],
    STA_ABY,    0x99,   3,  5,  [],
    STA_INX,    0x81,   2,  6,  [],
    STA_INY,    0x91,   2,  6,  [],

    // STX - Store X register
    STX_ZPG,    0x86,   2,  3,  [],
    STX_ZPY,    0x96,   2,  4,  [],
    STX_ABS,    0x8e,   3,  4,  [],

    // STY - Store Y register
    STY_ZPG,    0x84,   2,  3,  [],
    STY_ZPY,    0x94,   2,  4,  [],
    STY_ABS,    0x8c,   3,  4,  [],

    // TAX - Transfer accumulator to X
    TAX_IMP,    0xaa,   1,  2,  [Z N],

    // TAY - Transfer accumulator to Y
    TAY_IMP,    0xa8,   1,  2,  [Z N],

    // TSX - Transfer stack pointer to X
    TSX_IMP,    0xba,   1,  2,  [Z N],

    // TXA - Transfer X to accumulator
    TXA_IMP,    0x8a,   1,  2,  [Z N],

    // TXS - Transfer X to stack pointer
    TXS_IMP,    0x9a,   1,  2,  [],

    // TYA - Transfer Y to accumulator
    TYA_IMP,    0x98,   1,  2,  [Z N],
}
