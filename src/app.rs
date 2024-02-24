use crate::convert_image_to_ascii::convert_image_to_ascii;
use crate::util::*;
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
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

pub fn run(dir: &Path, ascii: bool) -> Result<()> {
    let files = get_files(dir)?;
    PATHS.lock().unwrap().extend_from_slice(&files);

    let mut app = App::new(&files, ascii);
    let mut terminal = init_terminal()?;

    app.run_tui(&mut terminal)?;
    reset_terminal()?;

    Ok(())
}

struct App {
    ascii: bool,

    files: Vec<PathBuf>, // TODO: Allow changes later (to add generated image paths)
    index: usize,

    picker: Picker,
    image_source: ImageSource,
    image_state: Box<dyn StatefulProtocol>,
}

impl App {
    fn new(files: &[PathBuf], ascii: bool) -> Self {
        // let path = files.first().unwrap();
        let binding = PATHS.lock().unwrap();
        let path = binding.first().unwrap();

        let dyn_img = image::io::Reader::open(path).unwrap().decode().unwrap();

        let mut picker = Picker::from_termios().unwrap();
        picker.guess_protocol();

        let image_source = ImageSource::new(dyn_img.clone(), picker.font_size);
        let image_state = picker.new_resize_protocol(dyn_img);

        Self {
            ascii,
            files: files.to_vec(),
            index: 0,
            picker,
            image_source,
            image_state,
        }
    }
}

impl App {
    fn run_tui<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        let tick_rate = Duration::from_secs(3);
        let mut last_tick = Instant::now();

        loop {
            terminal.draw(|f| self.ui(f))?;

            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('a') => self.ascii = !self.ascii,
                        KeyCode::Char('q') => return Ok(()),
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
        if self.ascii {
            // let path = self.files.get(self.index).unwrap();
            let binding = PATHS.lock().unwrap();
            let path = binding.get(self.index).unwrap();

            let ascii_art = convert_image_to_ascii(path)
                .expect("Failed to convert image to ascii art")
                .into_text()
                .unwrap();
            let paragraph = Paragraph::new(ascii_art);
            frame.render_widget(paragraph, frame.size())
        } else {
            let image = StatefulImage::new(None).resize(Resize::Fit);
            frame.render_stateful_widget(image, frame.size(), &mut self.image_state);
        }
    }

    fn on_tick(&mut self) {
        let mut index = self.index + 1;
        let length = {
            let paths = PATHS.lock().unwrap();
            paths.len()
        };
        if index >= length {
            index %= length;
        }
        self.index = index;
        // self.index = (self.index + 1) % self.files.len();

        if !self.ascii {
            let path = self.files.get(self.index).unwrap();
            let dyn_img = image::io::Reader::open(path).unwrap().decode().unwrap();
            self.image_source = ImageSource::new(dyn_img.clone(), self.picker.font_size);
            self.image_state = self.picker.new_resize_protocol(dyn_img);
        }
    }
}
