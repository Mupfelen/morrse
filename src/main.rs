mod morse_map;
mod morse;

use rocket::get;
use rodio::source::{SineWave, Source};
use std::time::Duration;
use hound::{WavWriter, SampleFormat};

#[get("/sound/file")]
fn sound_file() -> &'static str {
    "Hello, world!"
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let frequency = 440.0; // Frequency in Hertz (e.g., A4 note)
    let duration = Duration::new(1, 0); // 1 second duration
    let sine_wave = SineWave::new(frequency)
        .take_duration(duration)
        .amplify(0.5); // Adjust amplitude as needed

    // Create a buffer to hold the audio data
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100, // Sample rate in Hz
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };
    let mut buffer: Vec<i16> = Vec::new();

    // Buffer the audio data
    for sample in sine_wave.into_iter() {
        buffer.push((sample * i16::MAX as f32) as i16);
    }

    // Write the audio data to a WAV file
    let mut writer = WavWriter::create("sine_wave.wav", spec)?;
    for sample in buffer {
        writer.write_sample(sample)?;
    }


    Ok(())
}
