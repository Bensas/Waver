use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}, Device, Host, Stream, StreamConfig, SupportedStreamConfig};

pub struct AudioInput {
  host: Host,
  device: Device,
  config: SupportedStreamConfig,
  stream: Option<Stream>
}

impl AudioInput {
  pub fn new() -> AudioInput {
    let host = cpal::default_host();
    let device = host.default_input_device().expect("No input device available");
    let config= device.default_input_config().unwrap();
    println!("Input device: {}", device.name().unwrap());
    println!("Sample rate: {}", config.sample_rate().0);
    println!("Channels: {}", config.channels());
    return Self {
      host: host,
      device: device,
      config: config,
      stream: None
    }
  }

  pub fn start_capture(&mut self) {
    let err_fn = |err| eprintln!("Error: {}", err);
    let stream = self.device
        .build_input_stream(
            &self.config.clone().into(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                // Process audio data here
                println!("{:?}", &data[..10]); // Print first 10 samples
            },
            err_fn,
            None,
        )
        .unwrap();
    stream.play().unwrap();
    self.stream = Some(stream);
  }

  pub fn pause_capture(&mut self) {
    if let Some(stream) = self.stream.as_ref() {
      stream.pause().unwrap();
    } else {
      println!("Trying to pause an audio capture, but no capture was started.");
    }
  }

}

