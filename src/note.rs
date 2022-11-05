use std::{time::Duration, fmt};

use rodio::{source::{SineWave, TakeDuration}, Source};

#[derive(Copy, Clone, Debug)]
#[allow(unused)]
pub enum Notes {
	C,
	Cs,
	D,
	Ds,
	E,
	F,
	Fs,
	G,
	Gs,
	A,
	As,
	B
}

#[derive(Clone, Debug)]
pub struct Note {
	octave: u8,
	note: Notes,
	frequency: f32,
	duration: Duration,
	iter: TakeDuration<SineWave>,
}

impl Note {
	pub fn new(note: Notes, octave: u8, duration: Duration) -> Note {
		let frequency = get_frequency(octave, &note);
		let iter = SineWave::new(frequency).take_duration(duration);
		Self { 
			octave, 
			note,
			frequency,
			duration,
			iter
		}
	}
}

impl Iterator for Note {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
		self.iter.next()
	}
}

impl Source for Note {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        44100
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(self.duration)
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Note({:?}{}, duration={:?}, frequency={})", self.note, self.octave, self.duration, self.frequency)
    }
}

fn get_frequency(octave: u8, note: &Notes) -> f32 {
	let step_factor = (2.0_f32).powf(1.0 / 12.0);
	let index = get_index(octave, note);
	let a4_index = get_index(4, &Notes::A);
	440.0 * step_factor.powf((index as i16 - a4_index as i16) as f32)
}

fn get_index(octave: u8, note: &Notes) -> u8 {
	let octave_size = 12;
    let index = *note as u8;
	index + (octave as u8 * octave_size)
}