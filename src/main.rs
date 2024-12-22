use audio_capture::AudioInput;
use iced::{Application, Settings};
use wave_plotter::WavePlotter;

mod wave_plotter;
mod audio_capture;

fn main() {
    println!("Hello, world!");
    // WavePlotter::run(Settings::default());

    let mut audio_input = AudioInput::new();
    audio_input.start_capture();
    std::thread::sleep(std::time::Duration::from_secs(2)); // Record for 5 seconds
    audio_input.pause_capture();
    std::thread::sleep(std::time::Duration::from_secs(1)); // Record for 5 seconds
    audio_input.start_capture();
    std::thread::sleep(std::time::Duration::from_secs(1)); // Record for 5 seconds
}
