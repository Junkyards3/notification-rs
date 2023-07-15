use std::io;
use std::io::Stdout;

use crate::app::App;
use crate::db::Notification;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::backend::CrosstermBackend;
use tui::text::{Span, Spans};
use tui::widgets::{List, ListItem};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Frame, Terminal,
};

pub async fn run() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // app
    let app = App::new().await;
    run_app(&mut terminal, app);

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

fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>, app: App) -> io::Result<()> {
    loop {
        if app.needs_redraw {
            terminal.draw(|f| ui(f, &app.notifications))?;
        }
    }
}
fn ui<B: Backend>(f: &mut Frame<B>, notifications: &[Notification]) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(f.size());

    let block = Block::default()
        .title("Notifications")
        .borders(Borders::ALL);
    f.render_widget(block, chunks[0]);

    let list_items = notifications
        .iter()
        .map(|n| ListItem::new(Spans::from(vec![Span::raw(&n.title), Span::raw(&n.body)])))
        .collect::<Vec<_>>();
    let list_items = List::new(list_items).block(Block::default().borders(Borders::ALL));
    f.render_widget(list_items, chunks[1]);
}
