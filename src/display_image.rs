use ansi_to_tui::IntoText;
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::Frame;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    text::Text,
    widgets::Paragraph,
    Terminal,
};
use std::{
    io::{stdout, Stdout},
    time::{Duration, Instant},
};

struct App {
    data: Vec<String>,
}

impl App {
    fn new(data: &[String]) -> Self {
        Self {
            data: data.to_vec(),
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

fn init_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    Ok(terminal)
}

fn reset_terminal() -> Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}

fn run_tui<B: Backend>(terminal: &mut Terminal<B>, app: App) -> Result<()> {
    let mut count = 0;
    let tick_rate = Duration::from_secs(5);
    let mut last_tick = Instant::now();
    let mut text = app.data.get(count).expect("Failed to get data");

    loop {
        terminal.draw(|f| ui(f, &text.into_text().unwrap()))?;

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
            count = (count + 1) % app.data.len();
            match app.data.get(count) {
                Some(s) => {
                    text = s;
                    // terminal.clear()?;
                }
                None => return Ok(()),
            }
            last_tick = Instant::now();
        }
    }
}

fn ui(frame: &mut Frame, text: &Text) {
    let p = Paragraph::new(text.clone()).centered();
    frame.render_widget(p, frame.size())
}
