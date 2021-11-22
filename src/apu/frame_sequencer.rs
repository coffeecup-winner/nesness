use crate::util::ClockDivider;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FrameSequencerMode {
    FourStep,
    FiveStep,
}

pub struct FrameSequencer {
    mode: FrameSequencerMode,
    num_steps: u8,
    disable_irq: bool,
    step: u8,
    clock_divider: ClockDivider<12>, // Matches CPU speed
    step_x: ClockDivider<1>,         // This one is explicitly controlled
}

#[derive(Clone, Copy)]
pub struct FrameSequencerTriggers {
    envelopes: bool,
    length_counters: bool,
    frame_interrupt: bool,
}

impl FrameSequencer {
    pub fn new() -> Self {
        FrameSequencer {
            mode: FrameSequencerMode::FourStep,
            num_steps: 4,
            disable_irq: false,
            step: 0,
            clock_divider: ClockDivider::new(),
            step_x: ClockDivider::new(),
        }
    }

    pub fn tick(&mut self) -> FrameSequencerTriggers {
        self.clock_divider.tick();
        self.step_x.tick();
        if self.clock_divider.is_triggered() {
            self.step_x.tick();
        }
        let mut envelopes_trigger = false;
        let mut length_counters_trigger = false;
        let mut frame_interrupt_trigger = false;
        match self.mode {
            FrameSequencerMode::FourStep => {
                if self.step_x.is_triggered() {
                    match self.step {
                        0 => {
                            envelopes_trigger = true;
                            self.step_x.delay_ticks(7456);
                            self.step = 1;
                        }
                        1 => {
                            envelopes_trigger = true;
                            length_counters_trigger = true;
                            self.step_x.delay_ticks(7458);
                            self.step = 2;
                        }
                        2 => {
                            envelopes_trigger = true;
                            self.step_x.delay_ticks(7457);
                            self.step = 3;
                        }
                        3 => {
                            frame_interrupt_trigger = !self.disable_irq;
                            self.step_x.delay_ticks(1);
                            self.step = 4;
                        }
                        4 => {
                            envelopes_trigger = true;
                            length_counters_trigger = true;
                            frame_interrupt_trigger = !self.disable_irq;
                            self.step_x.delay_ticks(1);
                            self.step = 5;
                        }
                        5 => {
                            frame_interrupt_trigger = !self.disable_irq;
                            self.step_x.delay_ticks(7457);
                            self.step = 0;
                        }
                        _ => unreachable!(),
                    }
                }
            }
            FrameSequencerMode::FiveStep => match self.step {
                0 => {
                    envelopes_trigger = true;
                    self.step_x.delay_ticks(7456);
                    self.step = 1;
                }
                1 => {
                    envelopes_trigger = true;
                    length_counters_trigger = true;
                    self.step_x.delay_ticks(7458);
                    self.step = 2;
                }
                2 => {
                    envelopes_trigger = true;
                    self.step_x.delay_ticks(14910);
                    self.step = 3;
                }
                3 => {
                    envelopes_trigger = true;
                    length_counters_trigger = true;
                    self.step_x.delay_ticks(1);
                    self.step = 4;
                }
                4 => {
                    self.step_x.delay_ticks(7457);
                    self.step = 0;
                }
                _ => unreachable!(),
            },
        }
        FrameSequencerTriggers {
            envelopes: envelopes_trigger,
            length_counters: length_counters_trigger,
            frame_interrupt: frame_interrupt_trigger,
        }
    }

    pub fn reset(&mut self, mode: FrameSequencerMode, disable_irq: bool) {
        self.num_steps = if mode == FrameSequencerMode::FourStep {
            4
        } else {
            5
        };
        self.step = 0;
        self.disable_irq = disable_irq;
        self.clock_divider.reset();
        self.step_x.reset();
        self.step_x.delay_ticks(7457);
    }
}
