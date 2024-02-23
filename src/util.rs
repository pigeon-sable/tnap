use anyhow::Result;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{stdout, Stdout};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn get_files(theme: &str) -> Result<Vec<PathBuf>> {
    let mut files = vec![];

    let dir = Path::new("themes").join(theme);
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_file() {
            files.push(path);
        }
    }

    Ok(files)
}

pub fn init_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    Ok(terminal)
}

pub fn reset_terminal() -> Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}
