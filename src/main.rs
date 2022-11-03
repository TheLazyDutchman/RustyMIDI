use std::{thread::sleep, time::Duration, fmt, f32::consts::PI};

use cpal::{self, traits::{HostTrait, DeviceTrait, StreamTrait}, Sample, SampleFormat, SampleRate};

pub struct Note {
    buffer: Vec<f32>,
    index: usize
}

impl Note {
    pub fn new(rate: SampleRate, frequency: f32) -> Self {
        let mut buffer = Vec::new();
        for i in 0..((rate.0 as f32 / frequency * 100.0) as u32) {
            buffer.push((i as f32 / rate.0 as f32 * PI * 2.0 * frequency).sin());
        }

        println!("first: {:?}, last: {:?}", &buffer[0..5], &buffer[buffer.len() - 6 .. buffer.len() - 1]);

        Self { buffer, index: 0 }
    }

    pub fn read_next(&mut self) -> f32 {
        let value = self.buffer[self.index];
        self.index += 1;
        self.index %= self.buffer.len();
        value
    }
}

fn main() {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("No output device available.");
    let config = device.supported_output_configs()
        .expect("error when querying supported configs")
        .next()
        .expect("No config?!")
        .with_max_sample_rate();
    
    let sample_format = config.sample_format();

    let mut note = Note::new(config.sample_rate(), 261.63);

    let err_fn = |err| eprintln!("Error in buffer: {}", err);

    let config = config.into();
    let stream = match sample_format {
        SampleFormat::F32 => device.build_output_stream(&config, move |data, _| write_note::<f32>(&mut note, data), err_fn),
        SampleFormat::I16 => device.build_output_stream(&config, move |data, _| write_note::<i16>(&mut note, data), err_fn),
        SampleFormat::U16 => device.build_output_stream(&config, move |data, _| write_note::<u16>(&mut note, data), err_fn),
    }.unwrap();

    stream.play().unwrap();

    sleep(Duration::from_secs(5));
}

fn write_note<T: Sample + fmt::Debug>(note: &mut Note, data: &mut [T]) {
    for sample in data.chunks_mut(2) {
        let value = note.read_next();
        sample[0] = Sample::from(&value);
        sample[1] = Sample::from(&value);
    }
}