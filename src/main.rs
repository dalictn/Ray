use rodio::queue::queue;
use rodio::source::{SineWave, Source};
use rodio::{queue, Decoder, OutputStream, Sink};
use std::fs::File;
use std::io;
use std::io::*;
use std::ops::Index;
use std::{collections::VecDeque, io::stdin};
use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};
use termion::clear;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

fn rem_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next_back();
    chars.as_str()
}

fn strip_playlist(mut list: Vec<String>) -> Vec<String> {
    //strip playlist vector. This vec is planned to be used alongside sink.queue to hopefully mirror the current queue and be used as a playlist basically

    for mut item in list.iter_mut() {
        //item.to_string().replace(".mp3", "");
        *item = item.replace("songs/", item).to_string();
    }
    list
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

    writeln!(stdout, "[o] load ../songs/\r\n[s] load track\r").unwrap();
    //writeln!(stdout, "[s] load track").unwrap();
    let mut paused = false;

    for c in std::io::stdin().keys() {
        match c.unwrap() {
            Key::Char('o') => {
                println!("{}", clear::All);
                stdout.suspend_raw_mode().unwrap();
                play_songs(&sink, active, path, paused);
                break;
            }
            Key::Ctrl('q') => panic!(),
            Key::Char('s') => {
                println!("{}", clear::All);
                stdout.suspend_raw_mode().unwrap();
                play_song(&sink, &active, &mut stdout, &paused);
                break;
            }
            _ => (),
        }
    }

    // Load a sound from a file, using a path relative to Cargo.toml
    let mut trimmed_input = input.to_string().trim().to_string();
    let mut playing_song: bool = false;
}

fn play_song(sink: &Sink, mut active: &bool, stdout: &mut RawTerminal<Stdout>, mut paused: &bool) {
    let mut track = String::new().to_string();
    //todo: Change input method to use indices or add autocompletion as typing the song name will be tedious
    println!("Please enter file name: songs/...");
    let untrimmed_track = io::stdin().read_line(&mut track).unwrap();
    let track = rem_last(&track);
    //track.to_string().trim().to_string();
    let song = File::open(format!("songs/{track}.mp3"))
        .expect("Could not open song file. Perhaps the filename was misspelt?");
    let source = Decoder::new(song).unwrap();

    // append sound to sink and play

    sink.append(source);

    let mut sink_state = sink.empty();

    println!("Song is now playing.\r");
    println!("Press K to pause and resume, Ctrl + q to quit.\r");
    stdout.flush().unwrap();
    stdout.activate_raw_mode().unwrap();

    for c in stdin().keys() {
        match c.unwrap() {
            Key::Char('k') => {
                if paused == &true {
                    sink.pause();
                    paused = &false;
                } else {
                    sink.play();
                    paused = &true;
                }
            }
            Key::Ctrl('q') => panic!(),

            Key::Ctrl('x') => {
                println!("{}", clear::All);
                active = &true;
                return;
            }
            _ => (),
        }
    }
}
fn play_songs(sink: &Sink, mut active: bool, path: &str, mut paused: bool) {
    let mut list = recurse_files(path).unwrap();
    let mut list2 = list.clone();
    let mut playlist: Vec<String> = Vec::new();
    let mut view_list: Vec<String> = Vec::new();

    for entry in &mut list2 {
        //let file = File::open(&entry).unwrap();
        //let source = Decoder::new(file).unwrap();
        //sink.append(source);
        let entry_string = entry.display().to_string();
        view_list.push(entry_string);
    }

    //todo: extend file support to beyond mp3s. This will probably have to be rewritten in future
    //Strip file names of directory and extension
    for mut item in view_list.iter_mut() {
        *item = item.to_string().replace(".mp3", "");
        *item = item.replace("songs/", "").to_string();
    }

    //Add index of the songs in the list of found songs.
    for (i, el) in view_list.iter_mut().enumerate() {
        let i = i + 1;
        let ind_char = char::from_digit(i as u32, 10).unwrap();
        let mut index_braces: String = String::from("[] ");
        index_braces.insert(1, ind_char);

        el.insert_str(0, &index_braces);
    }

    println!("List of found songs; Enter the indices of the song(s) you'd like to queue: \r");
    println!("{:?}", view_list);

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    //todo: Add all option to queue all songs
    if input == "all" {
        list = list.clone();
        list = list.into()
    } else {
        let indices: Vec<_> = input
            .trim()
            .split(',')
            .flat_map(|s| s.trim().parse::<usize>().ok())
            .filter(|&idx| idx > 0 && idx <= list.len())
            .map(|idx| list[idx - 1].clone())
            .collect();
        list = indices.into();
    }

    //For each path in the list found, append to sink
    for entry in list {
        let file = File::open(&entry).unwrap();
        let source = Decoder::new(file).unwrap();
        sink.append(source);
        let entry_string = entry.display().to_string();
        playlist.insert(0, entry_string);
    }

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    println!("songs queued: \r");
    println!("{:?}\r", playlist);
    println!("Press K to pause and resume, Ctrl + c to quit. Ctrl + n for next song in queue\r");
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
            }
            Key::Ctrl('c') => panic!(),
            Key::Ctrl('n') => sink.skip_one(),

            Key::Ctrl('x') => {
                stdout.suspend_raw_mode().unwrap();
                active = true;
                return;
            }
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
