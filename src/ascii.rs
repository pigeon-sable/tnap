use crate::convert_image_to_ascii::convert_image_to_ascii;
use crate::util::*;
use ansi_to_tui::IntoText;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::Frame;
use ratatui::{backend::Backend, widgets::Paragraph, Terminal};
use std::path::PathBuf;
use std::time::{Duration, Instant};

struct App {
    files: Vec<PathBuf>,
    index: usize,
}

impl App {
    fn new(files: &[PathBuf]) -> Self {
        Self {
            files: files.to_vec(),
            index: 0,
        }
    }
}

pub fn run(theme: &str) -> Result<()> {
    let files = get_files(theme)?;
    let app = App::new(&files);
    let mut terminal = init_terminal()?;

    run_tui(&mut terminal, app)?;
    reset_terminal()?;

    Ok(())
}

fn run_tui<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<()> {
    let tick_rate = Duration::from_secs(3);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    // TODO: Switch between ASCII art and image mode
                    KeyCode::Char('s') => eprintln!("Unimplemented feature."),
                    _ => (),
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.index = (app.index + 1) % app.files.len();
            last_tick = Instant::now();
        }
    }
}

fn ui(frame: &mut Frame, app: &mut App) {
    let path = app
        .files
        .get(app.index)
        .expect("Failed to get an image path");
    let ascii_art = convert_image_to_ascii(&path)
        .expect("Failed to convert image to ascii art")
        .into_text()
        .unwrap();
    let paragraph = Paragraph::new(ascii_art).centered();

    frame.render_widget(paragraph, frame.size())
}
