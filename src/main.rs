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
use termion::clear;
use std::{fs::read_dir, path::{Path, PathBuf}};

fn rem_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next_back();
    chars.as_str()
}

fn idle(active: bool) {
    // choosing music file input
    // Get an output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
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

    let mut path: &str = "songs/";

    let files  = recurse_files(path).unwrap();

    //play_song(source,sink,active);
    play_songs(files,sink,active);

}

fn play_song(f: Decoder<File>, sink: Sink, mut active: bool) {




    // append sound to sink and play

    sink.append(f);

    //sink.sleep_until_end();
    let mut paused = false;

    let mut sink_state = sink.empty();


        let stdin = stdin();
        //setting up stdout and going into raw mode
        let mut stdout = stdout().into_raw_mode().unwrap();
        //printing welcoming message, clearing the screen and going to left top corner with the cursor
        //write!(stdout,, termion::cursor::Goto(1, 1))
        //.unwrap();
        println!("Song is now playing.");
        println!("Press K to pause and resume, Ctrl + q to quit.");
        stdout.flush().unwrap();

        for c in stdin.keys() {
            //clearing the screen and going to top left corner

            //i reckon this speaks for itself
            match c.unwrap() {
                Key::Char('k') => {
                    if paused == true {
                        sink.pause();
                        paused = false;
                    } else {
                        sink.play();
                        paused = true;
                    }
                },
                Key::Ctrl('q') => panic!(),

                        //figure out how to go back to main func from func
                Key::Ctrl('x') => {println!("{}", clear::All); active = true;return();},
                _ => (),
            }

            //stdout.flush().unwrap();
        }

    }
fn play_songs(l: Vec<PathBuf>, sink: Sink, mut active: bool) {
    //TODO: clean up and fix keybinds. Keep sink state and loop back to menu when finished. Polish up
    for entry in l {
        let file = File::open(entry).unwrap();
        let source = Decoder::new(file).unwrap();
        sink.append(source);
    }

    while sink.empty() == false {
        let mut paused = false;

        let stdin = stdin();
        //setting up stdout and going into raw mode
        let mut stdout = stdout().into_raw_mode().unwrap();
        //printing welcoming message, clearing the screen and going to left top corner with the cursor
        //write!(stdout,, termion::cursor::Goto(1, 1))
        //.unwrap();
        println!("Song is now playing.");
        println!("Press K to pause and resume, Ctrl + q to quit.");
        stdout.flush().unwrap();

        for c in stdin.keys() {
            //clearing the screen and going to top left corner

            //i reckon this speaks for itself
            match c.unwrap() {
                Key::Char('k') => {
                    if paused == true {
                        sink.pause();
                        paused = false;
                    } else {
                        sink.play();
                        paused = true;
                    }
                },
                Key::Ctrl('q') => panic!(),
                Key::Ctrl('n') => sink.skip_one(),


                //figure out how to go back to main func from func
                Key::Ctrl('x') => {println!("{}", clear::All); active = true;return();},
                _ => (),
            }

            //stdout.flush().unwrap();
        }

        sink.sleep_until_end();
    }


}


fn recurse_files(path: impl AsRef<Path>) -> std::io::Result<Vec<PathBuf>> {
    let mut buf = vec![];
    let entries = read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let meta = entry.metadata()?;

        if meta.is_dir() {
            let mut subdir = recurse_files(entry.path())?;
            buf.append(&mut subdir);
        }

        if meta.is_file() {
            buf.push(entry.path());
        }
    }

    Ok(buf)
}




fn main() {
    let mut active = true;

    //while active == true {
        idle(active);
    //}
    //println!("{:?}", files);


}
