use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Sample, SampleFormat, Stream,
};
use raylib::get_random_value;

pub struct APU {
    stream: Stream,
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

        APU { stream }
    }

    pub fn play(&mut self) {
        self.stream.play().expect("Failed to start playing audio");
    }
}
