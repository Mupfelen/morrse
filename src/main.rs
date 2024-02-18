mod morse_map;
mod morse;
//mod morse;

use clap::Parser;
use fundsp::shape::Shape::Clip;
use rocket::get;
use crate::morse::MorseOptions;

#[get("/sound/file")]
fn sound_file() -> &'static str {
    "Hello, world!"
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of times to greet
    message: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let morse_options = MorseOptions {
        wpm: 20,
        frequency: 440.0,
        channels: 1,
        sample_rate: 48000,
        volume: 0.5
    };
    
    let cli = Args::parse();
    let message = cli.message;
    
    let code = morse::translate_message(&message, &morse_map::morse_map());
    let buffer = morse::generate_morse_audio(code, &morse_options);
    morse::save_audio(buffer, "morse.wav", &morse_options)?;
    Ok(())
}
