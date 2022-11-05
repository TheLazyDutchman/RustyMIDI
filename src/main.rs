use std::time::Duration;

use note::{Note, Notes};
use rodio::{OutputStream, Sink};

mod note;

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let note = Note::new(Notes::B, 8, Duration::from_secs(3));
    println!("note: {}", note);
    
    sink.append(note);
    sink.sleep_until_end();
}