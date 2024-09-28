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
    let input_read_result = io::stdin().read_line(&mut input);
    let file = match input_read_result {
        Ok(input) => input.to_string(),
        Err(error) => return println!("{}", error),
    };

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
    rem_last(input.to_string().as_str());
    let mut trimmed_input = input.to_string().trim().to_string();
    let trimmed_input_result = File::open(format!("songs/{trimmed_input}.mp3"));
    let file = match trimmed_input_result {
        Ok(trimmed_input) => trimmed_input,
        Err(error) => return println!("{}", error),
    };
    println!("{input} is playing... ");

    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
// Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples());

// The sound plays in a separate audio thread,
// so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(150));
}
