mod db;
mod notification;
mod terminal;

use crate::terminal::ui;
use crossterm::terminal::disable_raw_mode;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::time::Duration;
use tokio::time::sleep;
use tui::widgets::Borders;
use tui::{backend::CrosstermBackend, widgets::Block, Terminal};

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let db = db::SqliteDb::set_up_db().await.unwrap();

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        ui(f);
    })?;

    sleep(Duration::from_millis(5000)).await;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
