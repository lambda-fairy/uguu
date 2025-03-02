#![windows_subsystem = "windows"]

use std::{io::Cursor, process};

use rodio::{Decoder, OutputStream, Sink};

static UGUU: &[u8] = include_bytes!("uguu_1.mp3");

fn main() {
    let (_stream, stream_handle) = if let Ok((_stream, stream_handle)) = OutputStream::try_default() {
        (_stream, stream_handle)
    } else {
        eprintln!("No audio output device available");
        process::exit(1);
    };

    let sink = Sink::try_new(&stream_handle).unwrap();

    sink.set_volume(0.2);

    sink.append(Decoder::new(Cursor::new(UGUU)).unwrap());

    sink.sleep_until_end();
}
