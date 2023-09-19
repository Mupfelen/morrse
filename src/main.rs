mod morse_map;
mod morse;
//mod morse;

use rocket::get;
use crate::morse::MorseOptions;

#[get("/sound/file")]
fn sound_file() -> &'static str {
    "Hello, world!"
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let morse_options = MorseOptions {
        wpm: 40,
        frequency: 440.0,
        channels: 1,
        sample_rate: 48000,
        volume: 0.5
    };

    let message = "Hello, world!";
    let code = morse::translate_message(message, &morse_map::morse_map());
    let buffer = morse::generate_morse_audio(code, &morse_options);
    morse::save_audio(buffer, "morse.wav", &morse_options)?;
    Ok(())
}
