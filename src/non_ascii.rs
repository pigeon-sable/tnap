use crate::util::*;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{backend::Backend, terminal::Frame, Terminal};
use ratatui_image::{picker::Picker, protocol::StatefulProtocol, StatefulImage};

struct App {
    image: Box<dyn StatefulProtocol>,
}

pub fn run(theme: &str) -> Result<()> {
    let files = get_files(theme)?;
    let path = files.first().expect("Directory is empty");

    let mut terminal = init_terminal()?;
    let mut picker = Picker::from_termios().unwrap();
    picker.guess_protocol();
    let dyn_img = image::io::Reader::open(path)?.decode()?;
    let image = picker.new_resize_protocol(dyn_img);
    let mut app = App { image };

    run_tui(&mut terminal, &mut app)?;
    reset_terminal()?;

    Ok(())
}

fn run_tui<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let image = StatefulImage::new(None);
    f.render_stateful_widget(image, f.size(), &mut app.image);
}
