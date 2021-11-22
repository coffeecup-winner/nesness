use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Sample, SampleFormat, Stream,
};
use raylib::get_random_value;

pub struct APU {
    stream: Stream,

    // Registers
    reg_pulse1_0: u8,
    reg_pulse1_1: u8,
    reg_pulse1_2: u8,
    reg_pulse1_3: u8,
    reg_pulse2_0: u8,
    reg_pulse2_1: u8,
    reg_pulse2_2: u8,
    reg_pulse2_3: u8,
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
            .next()
            .expect("No output configs")
            .with_max_sample_rate();
        let sample_format = main_config.sample_format();
        let config = main_config.into();
        let stream = match sample_format {
            SampleFormat::I16 => device.build_output_stream(
                &config,
                |data: &mut [i16], _: &cpal::OutputCallbackInfo| {
                    for sample in data.iter_mut() {
                        *sample = Sample::from(&0.0);
                    }
                },
                |_| {},
            ),
            SampleFormat::U16 => device.build_output_stream(
                &config,
                |data: &mut [u16], _: &cpal::OutputCallbackInfo| {
                    for sample in data.iter_mut() {
                        *sample = Sample::from(&0.0);
                    }
                },
                |_| {},
            ),
            SampleFormat::F32 => device.build_output_stream(
                &config,
                |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    let x: i16 = get_random_value::<i32>(-100, 100) as i16;
                    dbg!(x);
                    for frame in data.chunks_mut(2) {
                        for sample in frame.iter_mut() {
                            *sample = Sample::from(&(x as f32 / 100.0));
                        }
                    }
                },
                |_| {},
            ),
        }
        .unwrap();

        APU {
            stream,
            reg_pulse1_0: 0,
            reg_pulse1_1: 0,
            reg_pulse1_2: 0,
            reg_pulse1_3: 0,
            reg_pulse2_0: 0,
            reg_pulse2_1: 0,
            reg_pulse2_2: 0,
            reg_pulse2_3: 0,
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

    pub fn read_pulse1_0(&self) -> u8 {
        self.reg_pulse1_0
    }

    pub fn write_pulse1_0(&mut self, value: u8) {
        self.reg_pulse1_0 = value;
    }

    pub fn read_pulse1_1(&self) -> u8 {
        self.reg_pulse1_1
    }

    pub fn write_pulse1_1(&mut self, value: u8) {
        self.reg_pulse1_1 = value;
    }

    pub fn read_pulse1_2(&self) -> u8 {
        self.reg_pulse1_2
    }

    pub fn write_pulse1_2(&mut self, value: u8) {
        self.reg_pulse1_2 = value;
    }

    pub fn read_pulse1_3(&self) -> u8 {
        self.reg_pulse1_3
    }

    pub fn write_pulse1_3(&mut self, value: u8) {
        self.reg_pulse1_3 = value;
    }

    pub fn read_pulse2_0(&self) -> u8 {
        self.reg_pulse2_0
    }

    pub fn write_pulse2_0(&mut self, value: u8) {
        self.reg_pulse2_0 = value;
    }

    pub fn read_pulse2_1(&self) -> u8 {
        self.reg_pulse2_1
    }

    pub fn write_pulse2_1(&mut self, value: u8) {
        self.reg_pulse2_1 = value;
    }

    pub fn read_pulse2_2(&self) -> u8 {
        self.reg_pulse2_2
    }

    pub fn write_pulse2_2(&mut self, value: u8) {
        self.reg_pulse2_2 = value;
    }

    pub fn read_pulse2_3(&self) -> u8 {
        self.reg_pulse2_3
    }

    pub fn write_pulse2_3(&mut self, value: u8) {
        self.reg_pulse2_3 = value;
    }

    pub fn read_triangle_0(&self) -> u8 {
        self.reg_triangle_0
    }

    pub fn write_triangle_0(&mut self, value: u8) {
        self.reg_triangle_0 = value;
    }

    pub fn read_dummy_x09(&self) -> u8 {
        self.reg_dummy_x09
    }

    pub fn write_dummy_x09(&mut self, value: u8) {
        self.reg_dummy_x09 = value;
    }

    pub fn read_triangle_1(&self) -> u8 {
        self.reg_triangle_1
    }

    pub fn write_triangle_1(&mut self, value: u8) {
        self.reg_triangle_1 = value;
    }

    pub fn read_triangle_2(&self) -> u8 {
        self.reg_triangle_2
    }

    pub fn write_triangle_2(&mut self, value: u8) {
        self.reg_triangle_2 = value;
    }

    pub fn read_noise_0(&self) -> u8 {
        self.reg_noise_0
    }

    pub fn write_noise_0(&mut self, value: u8) {
        self.reg_noise_0 = value;
    }

    pub fn read_dummy_x0d(&self) -> u8 {
        self.reg_dummy_x0d
    }

    pub fn write_dummy_x0d(&mut self, value: u8) {
        self.reg_dummy_x0d = value;
    }

    pub fn read_noise_1(&self) -> u8 {
        self.reg_noise_1
    }

    pub fn write_noise_1(&mut self, value: u8) {
        self.reg_noise_1 = value;
    }

    pub fn read_noise_2(&self) -> u8 {
        self.reg_noise_2
    }

    pub fn write_noise_2(&mut self, value: u8) {
        self.reg_noise_2 = value;
    }

    pub fn read_dmc_0(&self) -> u8 {
        self.reg_dmc_0
    }

    pub fn write_dmc_0(&mut self, value: u8) {
        self.reg_dmc_0 = value;
    }

    pub fn read_dmc_1(&self) -> u8 {
        self.reg_dmc_1
    }

    pub fn write_dmc_1(&mut self, value: u8) {
        self.reg_dmc_1 = value;
    }

    pub fn read_dmc_2(&self) -> u8 {
        self.reg_dmc_2
    }

    pub fn write_dmc_2(&mut self, value: u8) {
        self.reg_dmc_2 = value;
    }

    pub fn read_dmc_3(&self) -> u8 {
        self.reg_dmc_3
    }

    pub fn write_dmc_3(&mut self, value: u8) {
        self.reg_dmc_3 = value;
    }

    pub fn read_dummy_x14(&self) -> u8 {
        self.reg_dummy_x14
    }

    pub fn write_dummy_x14(&mut self, value: u8) {
        self.reg_dummy_x14 = value;
    }

    pub fn read_status(&self) -> u8 {
        self.reg_status
    }

    pub fn write_status(&mut self, value: u8) {
        self.reg_status = value;
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
