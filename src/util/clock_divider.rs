pub struct ClockDivider<const RATIO: u64> {
    ticks_until_triggered: u64,
}

impl<const RATIO: u64> ClockDivider<RATIO> {
    pub fn new() -> Self {
        ClockDivider {
            ticks_until_triggered: 0,
        }
    }

    pub fn reset(&mut self) {
        self.ticks_until_triggered = 0;
    }

    pub fn tick(&mut self) {
        if self.ticks_until_triggered == 0 {
            self.ticks_until_triggered = RATIO;
        }
        self.ticks_until_triggered -= 1;
    }

    pub fn delay(&mut self, triggers: u64) {
        self.ticks_until_triggered += triggers * RATIO;
    }

    pub fn delay_ticks(&mut self, ticks: u64) {
        self.ticks_until_triggered += ticks;
    }

    pub fn is_triggered(&self) -> bool {
        self.ticks_until_triggered == 0
    }
}
