use iced::{Application, Settings};
use wave_plotter::WavePlotter;

mod wave_plotter;
fn main() {
    println!("Hello, world!");
    WavePlotter::run(Settings::default());
}
