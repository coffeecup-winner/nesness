use crate::util::ClockDivider;

use super::{envelope_generator::EnvelopeGenerator, length_counter::LengthCounter};

pub struct ChannelPulse {
    is_enabled: bool,
    envelope_generator: EnvelopeGenerator,
    length_counter: LengthCounter,
    timer_clock_divider: ClockDivider<2>,
    timer_period: u16,
    waveform: [bool; 8],
    seq_index: usize,

    // Raw register values
    reg_0: u8,
    reg_1: u8,
    reg_2: u8,
    reg_3: u8,
}

impl ChannelPulse {
    pub fn new() -> Self {
        ChannelPulse {
            is_enabled: false,
            envelope_generator: EnvelopeGenerator::new(),
            length_counter: LengthCounter::new(),
            timer_clock_divider: ClockDivider::new(),
            timer_period: 1,
            waveform: [false, true, false, false, false, false, false, false],
            seq_index: 0,

            reg_0: 0,
            reg_1: 0,
            reg_2: 0,
            reg_3: 0,
        }
    }

    pub fn read_reg_0(&self) -> u8 {
        self.reg_0
    }

    pub fn write_reg_0(&mut self, value: u8) {
        self.reg_0 = value;
        match value >> 6 {
            0 => {
                self.waveform = [false, true, false, false, false, false, false, false];
            }
            1 => {
                self.waveform = [false, true, true, false, false, false, false, false];
            }
            2 => {
                self.waveform = [false, true, true, true, true, false, false, false];
            }
            3 => {
                self.waveform = [true, false, false, true, true, true, true, true];
            }
            _ => unreachable!(),
        }
        let flag = ((value >> 5) & 0x01) == 0x01;
        self.length_counter.set_halt(flag);
        self.envelope_generator.set_loop(flag);
        self.envelope_generator
            .set_enabled(((value >> 4) & 0x01) == 0x01);
        self.envelope_generator.set_period(value & 0x0f);
    }

    pub fn read_reg_1(&self) -> u8 {
        self.reg_1
    }

    pub fn write_reg_1(&mut self, value: u8) {
        dbg!(value);
        self.reg_1 = value;
    }

    pub fn read_reg_2(&self) -> u8 {
        self.reg_2
    }

    pub fn write_reg_2(&mut self, value: u8) {
        self.reg_2 = value;
        self.update_timer_period();
    }

    pub fn read_reg_3(&self) -> u8 {
        self.reg_3
    }

    pub fn write_reg_3(&mut self, value: u8) {
        self.reg_3 = value;
        self.length_counter.set_index(value >> 3);
        self.envelope_generator.reset();
        self.seq_index = 0;
        self.update_timer_period();
    }

    pub fn get_volume(&self) -> u8 {
        let volume = self.envelope_generator.get_volume();
        // TODO: sweep
        if self.waveform[self.seq_index] && !self.length_counter.value_is_zero() {
            volume
        } else {
            0
        }
    }

    fn update_timer_period(&mut self) {
        self.timer_period = (((self.reg_3 as u16) & 0x7) | (self.reg_2 as u16)) + 1;
    }

    pub fn set_enabled(&mut self, value: bool) {
        self.is_enabled = value;
    }

    pub fn tick_timer(&mut self) {
        self.timer_clock_divider.tick();
        if self.timer_clock_divider.is_triggered() {
            self.seq_index = (self.seq_index + 1) % self.waveform.len();
            self.timer_clock_divider.delay(self.timer_period as u64);
        }
    }

    pub fn tick_length_counter(&mut self) {
        self.length_counter.tick();
    }

    pub fn tick_envelope_generator(&mut self) {
        self.envelope_generator.tick();
    }
}
