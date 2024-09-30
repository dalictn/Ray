use rodio::source::{SineWave, Source};
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::*;
use std::str::Chars;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use log::error;

fn rem_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next_back();
    chars.as_str()
}

fn play_song(f: Decoder<File>) {

    // Get an output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Play the sound directly on the device

    sink.append(f);

    //sink.sleep_until_end();

    let mut active = true;
    let mut paused = false;

    while active == true {
        let stdin = stdin();
        //setting up stdout and going into raw mode
        let mut stdout = stdout().into_raw_mode().unwrap();
        //printing welcoming message, clearing the screen and going to left top corner with the cursor
        write!(stdout, r#"{}{}ctrl + q to exit, ctrl + h to print "Hello world!", alt + t to print "termion is cool""#, termion::cursor::Goto(1, 1), termion::clear::All)
            .unwrap();
        stdout.flush().unwrap();

        for c in stdin.keys() {
            //clearing the screen and going to top left corner
            write!(
                stdout,
                "{}{}",
                termion::cursor::Goto(1, 1),
                termion::clear::All
            )
                .unwrap();

            //i reckon this speaks for itself
            match c.unwrap() {
                Key::Char('k') => {if paused == true{
                    sink.pause(); paused = false;
                }
                    else {
                        sink.play(); paused = true;
                    }
                },
                Key::Backspace => {active = false; break},
                //Key::Alt('t') => println!("termion is cool"),
                _ => (),
            }

            stdout.flush().unwrap();
    }


    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(150));

}}

fn main() {
    // choosing music file input
    let mut input = String::new().to_string();
    println!("please input song name: ");
    //TODO: list multiple songs in dir
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

    // Load a sound from a file, using a path relative to Cargo.toml
    rem_last(input.to_string().as_str());
    let mut trimmed_input = input.to_string().trim().to_string();
    let trimmed_input_result = File::open(format!("songs/{trimmed_input}.mp3"));
    let file = match trimmed_input_result {
        Ok(trimmed_input) => trimmed_input,
        Err(error) => return println!("{}", error),
    };

    let mut playing_song: bool = false;
    println!("{input} is playing... ");
    //TODO: Add keyboard input for song controls

    // Decode that sound file into a source
    let source_result = Decoder::new(file);

    let source = match source_result {
        Ok(source) => {playing_song = true; source}
        Err(error) => return println!("{}", error),
    };


    play_song(source)




}
