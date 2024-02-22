use crate::util::*;
use ansi_to_tui::IntoText;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::Frame;
use ratatui::{backend::Backend, widgets::Paragraph, Terminal};
use std::time::{Duration, Instant};

struct App {
    data: Vec<String>,
    index: usize,
}

impl App {
    fn new(data: &[String]) -> Self {
        Self {
            data: data.to_vec(),
            index: 0,
        }
    }
}

pub fn run(ascii_arts: &[String]) -> Result<()> {
    let mut terminal = init_terminal()?;
    let app = App::new(ascii_arts);

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
            app.index = (app.index + 1) % app.data.len();
            last_tick = Instant::now();
        }
    }
}

fn ui(frame: &mut Frame, app: &mut App) {
    let t = app
        .data
        .get(app.index)
        .expect("Failed to get data")
        .into_text()
        .expect("Failed to convert String to Text");
    let p = Paragraph::new(t).centered();
    frame.render_widget(p, frame.size())
}
