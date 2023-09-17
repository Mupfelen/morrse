use std::collections::HashMap;
use rodio::source::{SineWave, Silence, Source};
use std::time::Duration;


struct MorseOptions {
    volume: f32,
    frequency: f32,
    wpm: u32,
    channels: u16,
    sample_rate: u32,
}

fn translate_message(message: &str, morse_map: &HashMap<char, &'static str>) -> String {
    let mut morse_message = String::new();
    for c in message.chars() {
        let morse_char = morse_map.get(&c).unwrap_or(&"");
        morse_message.push_str(morse_char);
        morse_message.push_str(" ");
    }
    morse_message
}

fn unit_length(wpm: u32) -> Duration {
    Duration::from_secs_f32(60f32 / 50f32 * wpm as f32)
}
fn dit_length(wpm: u32) -> Duration {
    unit_length(wpm)
}

fn dah_length(wpm: u32) -> Duration {
    unit_length(wpm) * 3
}

fn intra_char_space(wpm: u32) -> Duration {
    unit_length(wpm)
}

fn inter_char_space(wpm: u32) -> Duration {
    unit_length(wpm) * 3
}

fn inter_word_space(wpm: u32) -> Duration {
    unit_length(wpm) * 7
}

fn silence(duration: Duration) -> Vec<i16> {

}

fn generate_morse_audio(code: String, options: &MorseOptions) ->  {
    let sine_wave = SineWave::new(options.frequency);

    let dah = || sine_wave
        .take_duration(dah_length(options.wpm))
        .collect::<Vec<i16>>();
    let dit = || sine_wave
        .take_duration(dit_length(options.wpm))
        .collect::<Vec<i16>>();
    let intra_char_space =

    let mut buffer: Vec<i16> = Vec::new();
    for sample in sine_wave.into_iter() {
        buffer.push((sample * i16::MAX as f32) as i16);
    }
    buffer
}
struct MorseGenerator {

}
