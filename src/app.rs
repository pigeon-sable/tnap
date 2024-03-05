use crate::convert_image_to_ascii::convert_image_to_ascii;
use crate::util::*;
use crate::APP_EXIT;
use crate::PATHS;
use ansi_to_tui::IntoText;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use ratatui::{backend::Backend, Terminal};
use ratatui_image::{
    picker::Picker,
    protocol::{ImageSource, StatefulProtocol},
    Resize, StatefulImage,
};
use std::path::Path;
use std::sync::atomic::Ordering::SeqCst;
use std::time::{Duration, Instant};

const DURATION: u64 = 3;

pub fn run(dir: &Path, ascii: bool) -> Result<()> {
    let files = get_files(dir)?;
    PATHS.lock().unwrap().extend_from_slice(&files);

    let mut app = App::new(ascii);
    let mut terminal = init_terminal()?;

    app.run_tui(&mut terminal)?;
    reset_terminal()?;

    Ok(())
}

struct App {
    ascii: bool,
    index: usize,

    picker: Picker,
    image_source: ImageSource,
    image_state: Box<dyn StatefulProtocol>,
}

impl App {
    fn new(ascii: bool) -> Self {
        let binding = PATHS.lock().unwrap();
        let path = binding.first().unwrap();

        let dyn_img = image::io::Reader::open(path).unwrap().decode().unwrap();

        let mut picker = Picker::from_termios().unwrap();
        picker.guess_protocol();

        let image_source = ImageSource::new(dyn_img.clone(), picker.font_size);
        let image_state = picker.new_resize_protocol(dyn_img);

        Self {
            ascii,
            index: 0,
            picker,
            image_source,
            image_state,
        }
    }
}

impl App {
    fn run_tui<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        let tick_rate = Duration::from_secs(DURATION);
        let mut last_tick = Instant::now();

        loop {
            terminal.draw(|f| self.ui(f))?;

            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('a') => self.ascii = !self.ascii,
                        KeyCode::Char('q') => {
                            APP_EXIT.store(true, SeqCst);
                            return Ok(());
                        }
                        _ => (),
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                self.on_tick();
                last_tick = Instant::now();
            }
        }
    }

    fn ui(&mut self, frame: &mut Frame) {
        let frame_size = frame.size();

        if self.ascii {
            let binding = PATHS.lock().unwrap();
            let path = binding.get(self.index).unwrap();

            let ascii_art = convert_image_to_ascii(path)
                .expect("Failed to convert image to ascii art")
                .into_text()
                .unwrap();

            let ascii_height = ascii_art.lines.len() as u16;
            let offset_y = (frame_size.height - ascii_height) / 2;
            let area = ratatui::layout::Rect::new(0, offset_y, frame_size.width, ascii_height);
            // println!("frame.size(): {}, area: {}", frame.size(), area);
            // println!("ascii_lines: {}", ascii_lines);

            let paragraph = Paragraph::new(ascii_art);
            frame.render_widget(paragraph, area)
        } else {
            let image = StatefulImage::new(None).resize(Resize::Fit);

            let image_width = frame_size.width * 4 / 5;
            let image_height = frame_size.height * 4 / 5;

            // Calculate centering of drawing area
            let area = ratatui::layout::Rect::new(
                (frame_size.width / 2).saturating_sub(image_width / 2) + frame_size.width / 7,
                // (frame_size.width / 2).saturating_sub(image_width / 2),
                (frame_size.height / 2).saturating_sub(image_height / 2),
                image_width,
                image_height,
            );
            // println!(
            //     "frame_size.width: {}, frame_size.height: {}",
            //     frame_size.width, frame_size.height
            // );
            // println!(
            //     "image_width: {}, image_height: {}",
            //     image_width, image_height
            // );
            // println!("area: {}", area);
            // println!(
            //     "self.image_source.image.width(): {},
            //     self.image_source.image.height(): {}",
            //     self.image_source.image.width(),
            //     self.image_source.image.height()
            // );
            frame.render_stateful_widget(image, area, &mut self.image_state);
        }
    }

    fn on_tick(&mut self) {
        let binding = PATHS.lock().unwrap();
        let length = binding.len();
        self.index = (self.index + 1) % length;

        let path = binding.get(self.index).unwrap();
        let dyn_img = image::io::Reader::open(path).unwrap().decode().unwrap();
        self.image_source = ImageSource::new(dyn_img.clone(), self.picker.font_size);
        self.image_state = self.picker.new_resize_protocol(dyn_img);
    }
}
