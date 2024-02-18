use std::collections::HashMap;
use rodio::source::{SineWave, Source};
use std::time::Duration;
use clap::Parser;
use hound::{SampleFormat, WavWriter};
use crate::morse_map;

pub struct MorseOptions {
    pub volume: f32,
    pub frequency: f32,
    pub wpm: u32,
    pub channels: u16,
    pub sample_rate: u32,
}

pub fn translate_message(message: &str, morse_map: &HashMap<char, &'static str>) -> String {
    let mut morse_message = String::new();
    for c in message.to_lowercase().chars() {
        let morse_char = morse_map.get(&c).unwrap_or(&"");
        morse_message.push_str(morse_char);
        morse_message.push(' ');
    }
    morse_message
}

#[test]
fn test_translate_message() {
    let message = "Hello, world!";
    let morse_map = morse_map::morse_map();
    let morse_message = translate_message(message, &morse_map);
    assert_eq!(morse_message, ".... . .-.. .-.. --- --..-- / .-- --- .-. .-.. -.. -.-.-- ");
}

fn unit_length(wpm: u32) -> Duration {
    Duration::from_secs_f32(60f32 / (50f32 * wpm as f32))
}

#[test]
fn test_unit_length() {
    let wpm = 20;
    let unit_length = unit_length(wpm);
    assert_eq!(unit_length.as_secs_f32(), 0.06);
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

//test silence function

#[test]
fn test_silence() {
    let duration = Duration::from_secs_f32(0.06);
    let channels = 1;
    let sample_rate = 48000;
    let silence_buffer = silence(duration, channels, sample_rate);
    assert_eq!(silence_buffer.len(), 2880);
}



fn apply_fade(idx: usize, sample: f32, fade_samples: usize, samples: usize) -> i16 {
    let amplitude =
        if idx < fade_samples { idx as f32 / fade_samples as f32 * sample }
        else if idx >= samples - fade_samples { (samples - idx) as f32 / fade_samples as f32 * sample }
        else { sample };
    (amplitude * i16::MAX as f32) as i16
}

fn dah(options: &MorseOptions) -> Vec<i16> {
    let sine_wave = SineWave::new(options.frequency);
    let fade_duration = Duration::from_millis(2);
    let fade_samples = (fade_duration.as_millis() as f32 / 1000.0 * options.sample_rate as f32) as usize;
    let samples = (dah_length(options.wpm).as_secs_f32() * options.sample_rate as f32) as usize;

    sine_wave
        .take_duration(dah_length(options.wpm))
        .amplify(options.volume)
        .enumerate()
        .map(|(idx, sample)| {
            apply_fade(idx, sample, fade_samples, samples)
        })
        .collect::<Vec<i16>>()
}

#[test]
fn test_dah() {
    let wpm = 20;
    let options = MorseOptions {
        wpm,
        frequency: 440.0,
        channels: 1,
        sample_rate: 48000,
        volume: 0.5
    };
    let dah_buffer = dah(&options);
    assert_eq!(dah_buffer.len(), 8640);
}

fn dit(options: &MorseOptions) -> Vec<i16> {
    let sine_wave = SineWave::new(options.frequency);
    let fade_duration = Duration::from_millis(2);
    let fade_samples = (fade_duration.as_millis() as f32 / 1000.0 * options.sample_rate as f32) as usize;
    let samples = (dit_length(options.wpm).as_secs_f32() * options.sample_rate as f32) as usize;

    sine_wave
        .take_duration(dit_length(options.wpm))
        .amplify(options.volume)
        .enumerate()
        .map(|(idx, sample)| {
            apply_fade(idx, sample, fade_samples, samples)
        })
        .collect::<Vec<i16>>()
}

fn intra_char_space(options: &MorseOptions) -> Vec<i16> {
    silence(intra_char_space_length(options.wpm), options.channels, options.sample_rate)
}

#[test]
fn test_intra_char_space() {
    let wpm = 20;
    let options = MorseOptions {
        wpm,
        frequency: 440.0,
        channels: 1,
        sample_rate: 48000,
        volume: 0.5
    };
    let intra_char_space_buffer = intra_char_space(&options);
    assert_eq!(intra_char_space_buffer.len(), 2880);
}

fn inter_char_space(options: &MorseOptions) -> Vec<i16> {
    silence(inter_char_space_length(options.wpm), options.channels, options.sample_rate)
}

#[test]
fn test_inter_char_space() {
    let wpm = 20;
    let options = MorseOptions {
        wpm,
        frequency: 440.0,
        channels: 1,
        sample_rate: 48000,
        volume: 0.5
    };
    let inter_char_space_buffer = inter_char_space(&options);
    let other = silence(unit_length(options.wpm) * 3, options.channels, options.sample_rate);
    assert_eq!(inter_char_space_buffer, other);
}

fn inter_word_space(options: &MorseOptions) -> Vec<i16> {
    silence(inter_word_space_length(options.wpm), options.channels, options.sample_rate)
}

pub fn generate_morse_audio(code: String, options: &MorseOptions) -> Vec<i16> {
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
            '/' => {
                buffer.append(&mut inter_word_space(options));
            }
            _ => (),
        }
    }
    buffer
}



pub fn save_audio(buffer: Vec<i16>, file_name: &str, options: &MorseOptions) -> Result<(), Box<dyn std::error::Error>> {
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
