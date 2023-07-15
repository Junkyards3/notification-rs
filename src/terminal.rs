use std::io;
use std::io::Stdout;
use std::time::{Duration, Instant};

use crate::app::App;
use crate::db::Notification;
use crossterm::event::{Event, KeyCode};
use crossterm::{
    event,
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::backend::CrosstermBackend;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{List, ListItem, ListState};
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
    run_app(&mut terminal, app).await?;

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

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    mut app: App,
) -> io::Result<()> {
    let last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &app.notifications, &mut app.state))?;

        let timeout = Duration::from_millis(250)
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char(c) => {
                            app.on_key(c).await;
                        }
                        KeyCode::Up => app.on_up(),
                        KeyCode::Down => app.on_down(),
                        _ => {}
                    }
                }
            }
        }

        if app.should_quit {
            return Ok(());
        }
    }
}
fn ui<B: Backend>(
    f: &mut Frame<B>,
    notifications: &[Notification],
    notifications_state: &mut ListState,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(f.size());

    let block = Block::default().title("Keys").borders(Borders::ALL);
    f.render_widget(block, chunks[0]);

    let list_items = notifications
        .iter()
        .map(|n| {
            ListItem::new(Spans::from(vec![
                Span::styled(&n.title, Style::default().fg(Color::Red)),
                Span::raw(" "),
                Span::raw(&n.body),
                Span::raw(" "),
                Span::styled(
                    n.date.format("%d %B - %R").to_string(),
                    Style::default().fg(Color::Blue),
                ),
            ]))
        })
        .collect::<Vec<_>>();
    let list_items = List::new(list_items)
        .block(
            Block::default()
                .title("Notifications")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green)),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");
    f.render_stateful_widget(list_items, chunks[1], notifications_state);
}
