use crate::util::ClockDivider;

pub struct EnvelopeGenerator {
    is_enabled: bool,
    is_reset: bool,
    is_looping: bool,
    counter: u8,
    period: u8,
    clock_divider: ClockDivider<1>, // TODO: Should really be another type
}

impl EnvelopeGenerator {
    pub fn new() -> Self {
        EnvelopeGenerator {
            is_enabled: false,
            is_reset: false,
            is_looping: false,
            counter: 0,
            period: 1,
            clock_divider: ClockDivider::new(),
        }
    }

    pub fn get_volume(&self) -> u8 {
        if !self.is_enabled {
            self.period - 1
        } else {
            self.counter
        }
    }

    pub fn set_enabled(&mut self, value: bool) {
        self.is_enabled = value;
    }

    pub fn set_loop(&mut self, value: bool) {
        self.is_looping = value;
    }

    pub fn set_period(&mut self, value: u8) {
        self.period = value + 1;
    }

    pub fn reset(&mut self) {
        self.is_reset = true;
    }

    pub fn tick(&mut self) {
        if self.is_reset {
            self.is_reset = false;
            self.counter = 15;
            self.clock_divider.delay_ticks(self.period as u64);
        } else {
            self.clock_divider.tick();
            if self.clock_divider.is_triggered() {
                if self.is_looping && self.counter == 0 {
                    self.counter = 15;
                } else if self.counter > 0 {
                    self.counter -= 1;
                }
                self.clock_divider.delay_ticks(self.period as u64);
            }
        }
    }
}
