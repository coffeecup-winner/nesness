use core::panic;

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    SampleFormat, Stream,
};
use crossbeam::channel::{bounded, Sender};

use crate::{apu::frame_sequencer::FrameSequencerMode, util::ClockDivider};

use super::{ch_pulse::ChannelPulse, frame_sequencer::FrameSequencer};

struct AudioBuffer {
    buffer: Vec<u8>,
    idx_write: usize,
    sender: Sender<Vec<u8>>,
}

impl AudioBuffer {
    pub fn new(size: usize, sender: Sender<Vec<u8>>) -> Self {
        AudioBuffer {
            buffer: vec![0; size],
            idx_write: 0,
            sender,
        }
    }

    pub fn push(&mut self, value: u8) {
        self.buffer[self.idx_write] = value;
        self.idx_write += 1;
        if self.idx_write == self.buffer.len() {
            self.idx_write = 0;
            // TODO: Use a circular buffer instead of copying
            self.sender
                .send(self.buffer.clone())
                .expect("Failed to send the audio buffer");
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
pub struct APU {
    stream: Stream,
    audio_buffer: AudioBuffer,
    audio_clock_divider: ClockDivider<400>, // TODO: fix this value

    // Functional units
    frame_sequencer: FrameSequencer,

    // Channels
    channel_pulse1: ChannelPulse,
    channel_pulse2: ChannelPulse,
    is_channel_triangle_enabled: bool,
    is_channel_noise_enabled: bool,
    is_channel_dmc_enabled: bool,

    // Registers
    reg_triangle_0: u8,
    reg_dummy_x09: u8,
    reg_triangle_1: u8,
    reg_triangle_2: u8,
    reg_noise_0: u8,
    reg_dummy_x0d: u8,
    reg_noise_1: u8,
    reg_noise_2: u8,
    reg_dmc_0: u8,
    reg_dmc_1: u8,
    reg_dmc_2: u8,
    reg_dmc_3: u8,
    reg_dummy_x14: u8,
    reg_status: u8,
    reg_dummy_x16: u8,
    reg_frame_counter: u8,
    reg_dummy_x18: u8,
    reg_dummy_x19: u8,
    reg_dummy_x1a: u8,
    reg_dummy_x1b: u8,
    reg_dummy_x1c: u8,
    reg_dummy_x1d: u8,
    reg_dummy_x1e: u8,
    reg_dummy_x1f: u8,
}

impl APU {
    pub fn new() -> Self {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("Failed to get the default output device");
        let mut configs = device
            .supported_output_configs()
            .expect("Failed to get supported output configs");
        let main_config = configs
            .find(|c| c.sample_format() == SampleFormat::F32 && c.channels() == 2)
            .expect("No output configs")
            .with_max_sample_rate();
        let sample_format = main_config.sample_format();
        let config = main_config.into();
        let (s, r) = bounded::<Vec<u8>>(1);
        let stream = match sample_format {
            SampleFormat::F32 => device.build_output_stream(
                &config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    let buf = r.recv().expect("Failed to receive the audio buffer");
                    for (i, frame) in data.chunks_mut(2).enumerate() {
                        let x = buf[i % buf.len()] * 10;
                        for sample in frame.iter_mut() {
                            *sample = x as f32 / 100.0;
                        }
                    }
                },
                |_| {},
                None,
            ),
            _ => panic!("Invalid sample format"),
        }
        .unwrap();

        APU {
            stream,
            audio_buffer: AudioBuffer::new(480, s),
            audio_clock_divider: ClockDivider::new(),
            frame_sequencer: FrameSequencer::new(),
            channel_pulse1: ChannelPulse::new(),
            channel_pulse2: ChannelPulse::new(),
            is_channel_triangle_enabled: false,
            is_channel_noise_enabled: false,
            is_channel_dmc_enabled: false,
            reg_triangle_0: 0,
            reg_dummy_x09: 0,
            reg_triangle_1: 0,
            reg_triangle_2: 0,
            reg_noise_0: 0,
            reg_dummy_x0d: 0,
            reg_noise_1: 0,
            reg_noise_2: 0,
            reg_dmc_0: 0,
            reg_dmc_1: 0,
            reg_dmc_2: 0,
            reg_dmc_3: 0,
            reg_dummy_x14: 0,
            reg_status: 0,
            reg_dummy_x16: 0,
            reg_frame_counter: 0,
            reg_dummy_x18: 0,
            reg_dummy_x19: 0,
            reg_dummy_x1a: 0,
            reg_dummy_x1b: 0,
            reg_dummy_x1c: 0,
            reg_dummy_x1d: 0,
            reg_dummy_x1e: 0,
            reg_dummy_x1f: 0,
        }
    }

    pub fn play(&mut self) {
        self.stream.play().expect("Failed to start playing audio");
    }

    pub fn tick(&mut self) {
        let triggers = self.frame_sequencer.tick();
        if triggers.frame_interrupt {
            dbg!("TODO: interrupt");
        }
        if triggers.length_counters {
            self.channel_pulse1.tick_length_counter();
        }
        if triggers.envelopes {
            self.channel_pulse1.tick_envelope_generator();
        }
        self.channel_pulse1.tick_timer();
        self.channel_pulse2.tick_timer();
        self.audio_clock_divider.tick();
        if self.audio_clock_divider.is_triggered() {
            self.audio_buffer.push(self.channel_pulse2.get_volume());
        }
    }

    pub fn read_pulse1_0(&self) -> u8 {
        self.channel_pulse1.read_reg_0()
    }

    pub fn write_pulse1_0(&mut self, value: u8) {
        self.channel_pulse1.write_reg_0(value)
    }

    pub fn read_pulse1_1(&self) -> u8 {
        self.channel_pulse1.read_reg_1()
    }

    pub fn write_pulse1_1(&mut self, value: u8) {
        self.channel_pulse1.write_reg_1(value)
    }

    pub fn read_pulse1_2(&self) -> u8 {
        self.channel_pulse1.read_reg_2()
    }

    pub fn write_pulse1_2(&mut self, value: u8) {
        self.channel_pulse1.write_reg_2(value)
    }

    pub fn read_pulse1_3(&self) -> u8 {
        self.channel_pulse1.read_reg_3()
    }

    pub fn write_pulse1_3(&mut self, value: u8) {
        self.channel_pulse1.write_reg_3(value)
    }

    pub fn read_pulse2_0(&self) -> u8 {
        self.channel_pulse2.read_reg_0()
    }

    pub fn write_pulse2_0(&mut self, value: u8) {
        self.channel_pulse2.write_reg_0(value)
    }

    pub fn read_pulse2_1(&self) -> u8 {
        self.channel_pulse2.read_reg_1()
    }

    pub fn write_pulse2_1(&mut self, value: u8) {
        self.channel_pulse2.write_reg_1(value)
    }

    pub fn read_pulse2_2(&self) -> u8 {
        self.channel_pulse2.read_reg_2()
    }

    pub fn write_pulse2_2(&mut self, value: u8) {
        self.channel_pulse2.write_reg_2(value)
    }

    pub fn read_pulse2_3(&self) -> u8 {
        self.channel_pulse2.read_reg_3()
    }

    pub fn write_pulse2_3(&mut self, value: u8) {
        self.channel_pulse2.write_reg_3(value)
    }

    pub fn read_triangle_0(&self) -> u8 {
        println!("read_triangle_0");
        self.reg_triangle_0
    }

    pub fn write_triangle_0(&mut self, value: u8) {
        println!("write_triangle_0: {}", value);
        self.reg_triangle_0 = value;
    }

    pub fn read_dummy_x09(&self) -> u8 {
        self.reg_dummy_x09
    }

    pub fn write_dummy_x09(&mut self, value: u8) {
        self.reg_dummy_x09 = value;
    }

    pub fn read_triangle_1(&self) -> u8 {
        println!("read_triangle_1");
        self.reg_triangle_1
    }

    pub fn write_triangle_1(&mut self, value: u8) {
        println!("write_triangle_1: {}", value);
        self.reg_triangle_1 = value;
    }

    pub fn read_triangle_2(&self) -> u8 {
        println!("read_triangle_2");
        self.reg_triangle_2
    }

    pub fn write_triangle_2(&mut self, value: u8) {
        println!("write_triangle_2: {}", value);
        self.reg_triangle_2 = value;
    }

    pub fn read_noise_0(&self) -> u8 {
        println!("read_noise_0");
        self.reg_noise_0
    }

    pub fn write_noise_0(&mut self, value: u8) {
        println!("write_noise_0: {}", value);
        self.reg_noise_0 = value;
    }

    pub fn read_dummy_x0d(&self) -> u8 {
        self.reg_dummy_x0d
    }

    pub fn write_dummy_x0d(&mut self, value: u8) {
        self.reg_dummy_x0d = value;
    }

    pub fn read_noise_1(&self) -> u8 {
        println!("read_noise_1");
        self.reg_noise_1
    }

    pub fn write_noise_1(&mut self, value: u8) {
        println!("write_noise_1: {}", value);
        self.reg_noise_1 = value;
    }

    pub fn read_noise_2(&self) -> u8 {
        println!("read_noise_2");
        self.reg_noise_2
    }

    pub fn write_noise_2(&mut self, value: u8) {
        println!("write_noise_2: {}", value);
        self.reg_noise_2 = value;
    }

    pub fn read_dmc_0(&self) -> u8 {
        println!("read_dmc_0");
        self.reg_dmc_0
    }

    pub fn write_dmc_0(&mut self, value: u8) {
        println!("write_dmc_0: {}", value);
        self.reg_dmc_0 = value;
    }

    pub fn read_dmc_1(&self) -> u8 {
        println!("read_dmc_1");
        self.reg_dmc_1
    }

    pub fn write_dmc_1(&mut self, value: u8) {
        println!("write_dmc_1: {}", value);
        self.reg_dmc_1 = value;
    }

    pub fn read_dmc_2(&self) -> u8 {
        println!("read_dmc_2");
        self.reg_dmc_2
    }

    pub fn write_dmc_2(&mut self, value: u8) {
        println!("write_dmc_2: {}", value);
        self.reg_dmc_2 = value;
    }

    pub fn read_dmc_3(&self) -> u8 {
        println!("read_dmc_3");
        self.reg_dmc_3
    }

    pub fn write_dmc_3(&mut self, value: u8) {
        println!("write_dmc_3: {}", value);
        self.reg_dmc_3 = value;
    }

    pub fn read_dummy_x14(&self) -> u8 {
        self.reg_dummy_x14
    }

    pub fn write_dummy_x14(&mut self, value: u8) {
        self.reg_dummy_x14 = value;
    }

    pub fn read_status(&self) -> u8 {
        println!("read_status");
        self.reg_status
    }

    pub fn write_status(&mut self, value: u8) {
        self.reg_status = value;
        self.channel_pulse1.set_enabled((value & 0x01) == 0x01);
        self.channel_pulse2.set_enabled((value & 0x02) == 0x02);
        self.is_channel_triangle_enabled = (value & 0x04) == 0x04;
        self.is_channel_noise_enabled = (value & 0x08) == 0x08;
        self.is_channel_dmc_enabled = (value & 0x10) == 0x10;
        // TODO: channel length counters clear
        // TODO: DMC clear
        // TODO: interrupt clear
    }

    pub fn read_dummy_x16(&self) -> u8 {
        self.reg_dummy_x16
    }

    pub fn write_dummy_x16(&mut self, value: u8) {
        self.reg_dummy_x16 = value;
    }

    pub fn read_frame_counter(&self) -> u8 {
        self.reg_frame_counter
    }

    pub fn write_frame_counter(&mut self, value: u8) {
        self.reg_frame_counter = value;
        let mode = if (value & 0x80) == 0x80 {
            FrameSequencerMode::FiveStep
        } else {
            FrameSequencerMode::FourStep
        };
        let disable_irq = (value & 0x40) == 0x40;
        self.frame_sequencer.reset(mode, disable_irq);
    }

    pub fn read_dummy_x18(&self) -> u8 {
        self.reg_dummy_x18
    }

    pub fn write_dummy_x18(&mut self, value: u8) {
        self.reg_dummy_x18 = value;
    }

    pub fn read_dummy_x19(&self) -> u8 {
        self.reg_dummy_x19
    }

    pub fn write_dummy_x19(&mut self, value: u8) {
        self.reg_dummy_x19 = value;
    }

    pub fn read_dummy_x1a(&self) -> u8 {
        self.reg_dummy_x1a
    }

    pub fn write_dummy_x1a(&mut self, value: u8) {
        self.reg_dummy_x1a = value;
    }

    pub fn read_dummy_x1b(&self) -> u8 {
        self.reg_dummy_x1b
    }

    pub fn write_dummy_x1b(&mut self, value: u8) {
        self.reg_dummy_x1b = value;
    }

    pub fn read_dummy_x1c(&self) -> u8 {
        self.reg_dummy_x1c
    }

    pub fn write_dummy_x1c(&mut self, value: u8) {
        self.reg_dummy_x1c = value;
    }

    pub fn read_dummy_x1d(&self) -> u8 {
        self.reg_dummy_x1d
    }

    pub fn write_dummy_x1d(&mut self, value: u8) {
        self.reg_dummy_x1d = value;
    }

    pub fn read_dummy_x1e(&self) -> u8 {
        self.reg_dummy_x1e
    }

    pub fn write_dummy_x1e(&mut self, value: u8) {
        self.reg_dummy_x1e = value;
    }

    pub fn read_dummy_x1f(&self) -> u8 {
        self.reg_dummy_x1f
    }

    pub fn write_dummy_x1f(&mut self, value: u8) {
        self.reg_dummy_x1f = value;
    }
}
