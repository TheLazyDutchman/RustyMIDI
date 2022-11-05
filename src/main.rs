use std::time::Duration;

use rodio::{OutputStream, Sink, source::SineWave, Source};

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let note = SineWave::new(200.0).take_duration(Duration::from_secs(5));
    sink.append(note);

    sink.sleep_until_end()
}