use std::collections::HashMap;
use rodio::source::{SineWave, Source};
use std::time::Duration;
use hound::{SampleFormat, WavWriter};


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

fn intra_char_space_length(wpm: u32) -> Duration {
    unit_length(wpm)
}

fn inter_char_space_length(wpm: u32) -> Duration {
    unit_length(wpm) * 3
}

fn inter_word_space_length(wpm: u32) -> Duration {
    unit_length(wpm) * 7
}

fn silence(duration: Duration, channels: u16, sample_rate: u32) -> Vec<i16> {
    let secs = duration.as_secs_f32();
    let sample_count = (secs * sample_rate as f32 * channels as f32) as u64;

    vec![0; sample_count as usize]
}

fn dah(options: &MorseOptions) -> Vec<i16> {
    let sine_wave = SineWave::new(options.frequency);

    sine_wave
        .take_duration(dah_length(options.wpm))
        .map(|sample| sample as i16)
        .collect::<Vec<i16>>()
}

fn dit(options: &MorseOptions) -> Vec<i16> {
    let sine_wave = SineWave::new(options.frequency);

    sine_wave
        .take_duration(dit_length(options.wpm))
        .map(|sample| sample as i16)
        .collect::<Vec<i16>>()
}

fn intra_char_space(options: &MorseOptions) -> Vec<i16> {
    silence(intra_char_space_length(options.wpm), options.channels, options.sample_rate)
}

fn inter_char_space(options: &MorseOptions) -> Vec<i16> {
    silence(inter_char_space_length(options.wpm), options.channels, options.sample_rate)
}

fn inter_word_space(options: &MorseOptions) -> Vec<i16> {
    silence(inter_word_space_length(options.wpm), options.channels, options.sample_rate)
}

fn generate_morse_audio(code: String, options: &MorseOptions) -> Vec<i16> {
    let mut buffer: Vec<i16> = Vec::new();
    for c in code.chars() {
        match c {
            '.' => {
                buffer.append(&mut dit(options));
                buffer.append(&mut intra_char_space(options));
            }
            '-' => {
                buffer.append(&mut dah(options));
                buffer.append(&mut intra_char_space(options));
            }
            ' ' => {
                buffer.append(&mut inter_char_space(options));
            },
            _ => (),
        }
        buffer.append(&mut inter_word_space(options));
    }
    buffer
}

fn save_audio(buffer: Vec<i16>, file_name: &str, options: &MorseOptions) -> Result<(), Box<dyn std::error::Error>> {
    let spec = hound::WavSpec {
        channels: options.channels,
        sample_rate: options.sample_rate,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };

    let mut writer = WavWriter::create(file_name, spec)?;
    for sample in buffer {
        writer.write_sample(sample)?;
    }
    Ok(())
}

struct MorseGenerator {}
