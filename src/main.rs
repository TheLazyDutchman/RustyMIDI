use std::{thread::sleep, time::Duration, fmt};

use cpal::{self, traits::{HostTrait, DeviceTrait, StreamTrait}, Sample, SampleFormat};

fn main() {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("No output device available.");
    let config = device.supported_output_configs()
        .expect("error when querying supported configs")
        .next()
        .expect("No config?!")
        .with_max_sample_rate();
    
    let sample_format = config.sample_format();

    let err_fn = |err| eprintln!("Error in buffer: {}", err);

    let config = config.into();
    let stream = match sample_format {
        SampleFormat::F32 => device.build_output_stream(&config, write_silence::<f32>, err_fn),
        SampleFormat::I16 => device.build_output_stream(&config, write_silence::<i16>, err_fn),
        SampleFormat::U16 => device.build_output_stream(&config, write_silence::<u16>, err_fn),
    }.unwrap();

    stream.play().unwrap();

    sleep(Duration::from_secs(5));
}

fn write_silence<T: Sample + fmt::Debug>(data: &mut [T], _: &cpal::OutputCallbackInfo) {
    for sample in data.iter_mut() {
        *sample = Sample::from(&0.0);
    }
}