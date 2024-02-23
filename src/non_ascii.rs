use crate::util::*;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{backend::Backend, terminal::Frame, Terminal};
use ratatui_image::{
    picker::Picker,
    protocol::{ImageSource, StatefulProtocol},
    Resize, StatefulImage,
};
use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

struct App {
    files: Vec<PathBuf>,
    index: usize,

    picker: Picker,
    image_source: ImageSource, // TODO: Remove later?
    image_state: Box<dyn StatefulProtocol>,
}

impl App {
    fn new(files: &[PathBuf]) -> Self {
        let path = files.first().unwrap();
        let dyn_img = image::io::Reader::open(path).unwrap().decode().unwrap();

        let mut picker = Picker::from_termios().unwrap();
        picker.guess_protocol();

        let image_source = ImageSource::new(dyn_img.clone(), picker.font_size);
        let image_state = picker.new_resize_protocol(dyn_img);

        Self {
            files: files.to_vec(),
            index: 0,
            picker,
            image_source,
            image_state,
        }
    }

    fn on_tick(&mut self) {
        self.index = (self.index + 1) % self.files.len();
        let path = self.files.get(self.index).unwrap();
        let dyn_img = image::io::Reader::open(path).unwrap().decode().unwrap();
        self.image_source = ImageSource::new(dyn_img.clone(), self.picker.font_size);
        self.image_state = self.picker.new_resize_protocol(dyn_img);
    }
}

pub fn run(theme: &str) -> Result<()> {
    let files = get_files(theme)?;
    let mut app = App::new(&files);
    let mut terminal = init_terminal()?;

    run_tui(&mut terminal, &mut app)?;
    reset_terminal()?;

    Ok(())
}

fn run_tui<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    let tick_rate = Duration::from_secs(3);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui(frame: &mut Frame, app: &mut App) {
    let image = StatefulImage::new(None).resize(Resize::Fit);
    frame.render_stateful_widget(image, frame.size(), &mut app.image_state);
}
