use std::io;
use std::process;

static UGUU: &[u8] = include_bytes!("uguu_1.mp3");

fn main() {
    let device = if let Some(device) = rodio::default_output_device() {
        device
    } else {
        eprintln!("No audio output device available");
        process::exit(1);
    };

    let sink = rodio::play_once(&device, io::Cursor::new(UGUU)).unwrap();
    sink.sleep_until_end();
}
