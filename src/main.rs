#![windows_subsystem = "windows"]

use std::{error::Error, io::Cursor};

use rodio::{Decoder, OutputStream, Sink};

static UGUU: &[u8] = include_bytes!("uguu_1.mp3");

fn main() -> Result<(), Box<dyn Error>> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    sink.set_volume(0.2);
    sink.append(Decoder::new(Cursor::new(UGUU))?);
    sink.sleep_until_end();
    Ok(())
}
