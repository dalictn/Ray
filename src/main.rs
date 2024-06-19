use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};
use rodio::source::{SineWave, Source};
use std::io;
use std::io::*;
use std::str::Chars;

fn rem_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next_back();
    chars.as_str()
}



fn main() {
// choosing music file input
    let mut input = String::new().to_string();
    println!("please input songname: ");
    io::stdin().read_line(&mut input).expect("error: unable to read user input");
    println!("{input}");
    println!("{input} is playing... ");
    ///let rem_last = |value: &str| -> &str {
       /// let mut chars = value.chars();
        ///chars.next_back();
        ///chars.as_str();
       /// return &input;
    ///};


// Get an output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

// Load a sound from a file, using a path relative to Cargo.toml
    rem_last(input.as_str());
    let mut trimmedinput = input.trim().to_string();
    let file = BufReader::new(File::open(format!("songs/{trimmedinput}.mp3")).unwrap());
    print!("{input}");
// Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
// Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples());

// The sound plays in a separate audio thread,
// so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(150));
}
