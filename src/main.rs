use rodio::source::{SineWave, Source};
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::*;
use std::str::Chars;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use log::error;
use termion::clear;
use std::{fs::read_dir, path::{Path, PathBuf}};

fn rem_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next_back();
    chars.as_str()
}

fn idle(mut active: bool) {
    // choosing music file input
    // Get an output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let mut input = String::new().to_string();
    let mut path: &str = "songs/";

    let stdin = stdin();
    //setting up stdout and going into raw mode
    let mut stdout = stdout().into_raw_mode().unwrap();
    writeln!(stdout, "{}{}", clear::All, termion::cursor::Goto(1, 1)).unwrap();


    writeln!(stdout , "[o] load ../songs/\n [s] load track").unwrap();
    //writeln!(stdout, "[s] load track").unwrap();

    for c in std::io::stdin().keys() {

        //i reckon this speaks for itself
        match c.unwrap() {
            Key::Char('o') => {println!("{}", clear::All); play_songs(&sink, active, path);break;},

            Key::Ctrl('q') => panic!(),

            //figure out how to go back to main func from func
            Key::Char('s') => {println!("{}", clear::All);stdout.suspend_raw_mode().unwrap();play_song(&sink, &active, &mut stdout);break;},
            _ => (),
        }}


    // Load a sound from a file, using a path relative to Cargo.toml
    let mut trimmed_input = input.to_string().trim().to_string();
    let mut playing_song: bool = false;

}

fn play_song(sink: &Sink, mut active: &bool, stdout: &mut RawTerminal<Stdout>) {



    let mut track = String::new().to_string();
    println!("Please enter file name: songs/...");
    let untrimmed_track = io::stdin().read_line(&mut track).unwrap();
    let track = rem_last(&track);
    //track.to_string().trim().to_string();
    let song = File::open(format!("songs/{track}.mp3")).expect("Could not open song file. Perhaps the filename was misspelt?");
    let source = Decoder::new(song).unwrap();



    // append sound to sink and play

    sink.append(source);

    let mut paused = false;

    let mut sink_state = sink.empty();

        println!("Song is now playing.");
        println!("Press K to pause and resume, Ctrl + q to quit.");
        stdout.flush().unwrap();
        stdout.activate_raw_mode().unwrap();

        for c in stdin().keys() {

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

                Key::Ctrl('x') => {println!("{}", clear::All); active = &true;return;},
                _ => (),
            }
        }

    }
fn play_songs(sink: &Sink, mut active: bool, path: &str) {

    let list = recurse_files(path).unwrap();


    for entry in list {
        let file = File::open(entry).unwrap();
        let source = Decoder::new(file).unwrap();
        sink.append(source);
    }

        let mut paused = false;

        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();
        println!("Song is now playing.");
        println!("Press K to pause and resume, Ctrl + q to quit.");
        stdout.flush().unwrap();

        for c in stdin.keys() {
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


                Key::Ctrl('x') => {
                    stdout.suspend_raw_mode().unwrap();
                    active = true;
                    return;
                },
                _ => (),
            }
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



    while active == true {
        idle(active);
    }


}
