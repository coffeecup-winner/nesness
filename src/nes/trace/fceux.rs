// FCEUX trace log format

use lazy_static::lazy_static;
use regex::Regex;

use super::*;

pub struct FceuxTrace<'a> {
    lines: Vec<&'a str>,
    pos: usize,
    total_cycles: u64,
    reg_a: u8,
    reg_x: u8,
    reg_y: u8,
    reg_s: u8,
    pc: u16,
    flag_carry: bool,
    flag_zero: bool,
    flag_interrupt_disable: bool,
    flag_decimal_mode: bool,
    flag_break: bool,
    flag_overflow: bool,
    flag_negative: bool,
}

impl<'a> FceuxTrace<'a> {
    pub fn new(text: &'a str) -> Self {
        FceuxTrace {
            lines: text.lines().into_iter().skip(1).collect(),
            pos: 0,
            total_cycles: 0,
            reg_a: 0,
            reg_x: 0,
            reg_y: 0,
            reg_s: 0,
            pc: 0,
            flag_carry: false,
            flag_zero: false,
            flag_interrupt_disable: false,
            flag_decimal_mode: false,
            flag_break: false,
            flag_overflow: false,
            flag_negative: false,
        }
    }
}

impl<'a> ExecutionTrace for FceuxTrace<'a> {
    fn cycles_start_with_0(&self) -> bool {
        true
    }
    
    fn advance(&mut self) -> bool {
        lazy_static! {
            static ref REGEX: Regex = Regex::new("^f\\d+ +c(\\d+) +i\\d+ +A:([0-9A-F]{2}) X:([0-9A-F]{2}) Y:([0-9A-F]{2}) S:([0-9A-F]{2}) P:([nN])([vV])u([bB])([dD])([iI])([zZ])([cC]) +\\$([0-9A-F]{4}):.*$").expect("Failed to create a regex");
        };
        if self.pos >= self.lines.len() {
            return false;
        }
        if let Some(caps) = REGEX.captures(self.lines[self.pos]) {
            self.total_cycles = caps[1].parse().expect("Failed to parse cycles count");
            self.reg_a = u8::from_str_radix(&caps[2], 16).expect("Failed to parse reg A");
            self.reg_x = u8::from_str_radix(&caps[3], 16).expect("Failed to parse reg X");
            self.reg_y = u8::from_str_radix(&caps[4], 16).expect("Failed to parse reg Y");
            self.reg_s = u8::from_str_radix(&caps[5], 16).expect("Failed to parse reg S");
            self.pc = u16::from_str_radix(&caps[13], 16).expect("Failed to parse PC");
            self.flag_carry = &caps[12] == "C";
            self.flag_zero = &caps[11] == "Z";
            self.flag_interrupt_disable = &caps[10] == "I";
            self.flag_decimal_mode = &caps[9] == "D";
            self.flag_break = &caps[8] == "B";
            self.flag_overflow = &caps[7] == "V";
            self.flag_negative = &caps[6] == "N";
        } else {
            panic!("Failed to parse line `{}`", self.lines[self.pos])
        }
        self.pos += 1;
        true
    }

    fn total_cycles(&self) -> u64 {
        self.total_cycles
    }

    fn reg_a(&self) -> u8 {
        self.reg_a
    }

    fn reg_x(&self) -> u8 {
        self.reg_x
    }

    fn reg_y(&self) -> u8 {
        self.reg_y
    }

    fn reg_s(&self) -> u8 {
        self.reg_s
    }

    fn pc(&self) -> u16 {
        self.pc
    }

    fn flag_carry(&self) -> bool {
        self.flag_carry
    }

    fn flag_zero(&self) -> bool {
        self.flag_zero
    }

    fn flag_interrupt_disable(&self) -> bool {
        self.flag_interrupt_disable
    }

    fn flag_decimal_mode(&self) -> bool {
        self.flag_decimal_mode
    }

    fn flag_break(&self) -> bool {
        self.flag_break
    }

    fn flag_overflow(&self) -> bool {
        self.flag_overflow
    }

    fn flag_negative(&self) -> bool {
        self.flag_negative
    }
}
