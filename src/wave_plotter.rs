use std::{env, f32::consts::PI, time::Duration};

use iced::{
  event::Status, executor, keyboard::{self, KeyCode}, widget::canvas::{stroke::{LineCap, Stroke}, Canvas, Cursor, Frame, Geometry, Path, Program}, Application, Command, Element, Event, Point, Subscription, Theme
};
use rand::Rng;

pub struct WavePlotter {
  points: Vec<Point>,
  paused: bool
}

#[derive(Debug, Clone)]
pub enum WavePlotterMessage {
  UpdatePoints(Vec<Point>),
  EventOccurred(iced_native::Event)
}

impl Application for WavePlotter {
  type Message = WavePlotterMessage;
  type Executor = executor::Default;

  type Theme = Theme;
  
  type Flags = ();

  fn new(flags: Self::Flags) -> (WavePlotter, iced::Command<WavePlotterMessage>) {
    // let args: Vec<String> = env::args().collect();
    // let arg_one = args.get(1).unwrap();
    
    return (Self { points: Vec::new(), paused: false }, Command::none());
  }

  fn title(&self) -> String {
    return String::from("Wave Plotter v0.1");
  }

  fn update(&mut self, message: Self::Message) -> iced::Command<WavePlotterMessage> {

    match message {
        WavePlotterMessage::UpdatePoints(points) => {
          self.points = points;
        },
        WavePlotterMessage::EventOccurred(event) => {
          match event {
            Event::Keyboard(keyboard::Event::KeyReleased { key_code: KeyCode::Space, modifiers }) => {
              println!("Spacebar pressed!");
              self.paused = !self.paused;
            },
            _ => {
              println!("Event happened :)");
            }
          }
      }
    }
    // self.mem_visualizer.update(&mut self.cpu);

    Command::none()
  }

  fn view(&self) -> Element<'_, Self::Message> {
    Canvas::new(WavePlotterCanvas {
      points: self.points.clone(),
    })
    .width(iced::Length::Fill)
    .height(iced::Length::Fill)
    .into()
  }

  fn subscription(&self) -> Subscription<WavePlotterMessage> {
    let mut subs = vec![];
    subs.push(iced_native::subscription::events().map(WavePlotterMessage::EventOccurred));
    if !self.paused {
      subs.push(iced::time::every(Duration::from_millis(50)).map(|em| {
        let newPoints: Vec<Point> = generate_random_wave();
        WavePlotterMessage::UpdatePoints(newPoints)
      }));
    }
    return Subscription::batch(subs);
  }
}

struct WavePlotterCanvas {
  points: Vec<Point>,
}

impl Program<WavePlotterMessage> for WavePlotterCanvas {
  type State = ();

  fn draw(
    &self,
    _state: &Self::State,
    renderer: &Theme,
    bounds: iced::Rectangle,
    _cursor: Cursor,
  ) -> Vec<Geometry> {
    let mut frame = Frame::new(bounds.size());

    // Draw axes
    let center = Point::new(bounds.width / 2.0, bounds.height / 2.0);
    frame.stroke(
        &Path::line(Point::new(0.0, center.y), Point::new(bounds.width, center.y)),
        Stroke::default().with_width(1.0),
    );
    frame.stroke(
        &Path::line(Point::new(center.x, 0.0), Point::new(center.x, bounds.height)),
        Stroke::default().with_width(1.0),
    );

    // Draw sine wave
    let path = Path::new(|builder| {
        if let Some(first_point) = self.points.first() {
            builder.move_to(Point::new(first_point.x * 50.0, center.y - first_point.y * 50.0));
        }
        for point in &self.points {
            let x = point.x * 50.0; // Scale X-axis
            let y = center.y - point.y * 50.0; // Scale Y-axis
            builder.line_to(Point::new(x, y));
        }
    });

    frame.stroke(
        &path,
        Stroke::default()
            .with_width(2.0)
            .with_line_cap(LineCap::Round),
    );

    vec![frame.into_geometry()]
  }

  fn update(
    &self,
    _state: &mut Self::State,
    _event: iced::widget::canvas::Event,
    _bounds: iced::Rectangle,
    _cursor: Cursor,
  ) -> (Status, Option<WavePlotterMessage>) {
      (Status::Ignored, None)
  }
}


fn generate_random_wave() -> Vec<Point> {
  let mut result = Vec::new();

  let mut rng = rand::thread_rng();
  let random_f32: f32 = rng.gen();
  for i in 1..=100 {
    let t: f32 = i as f32 / 10.0;
    result.push(Point::new(t, (t * PI * random_f32).sin()));
  }
  result
}