use std::io;
use std::io::Stdout;
use std::time::{Duration, Instant};

use chrono::Local;
use crossterm::event::Event;
use crossterm::{
    event,
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::backend::CrosstermBackend;
use tui::Terminal;

use crate::app::App;
use crate::db::{NotificationContent, SqliteDb};
use crate::ui::ui;

pub async fn run() -> Result<(), sqlx::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // app
    let db = SqliteDb::set_up_db().await?;
    let notifications = db.load_notifications().await?;
    let app = App::new(notifications);
    let res = run_app(&mut terminal, app, db);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {}", err)
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    mut app: App,
    db: SqliteDb,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(250);

    let mut database_tick = Instant::now();
    let database_tick_rate = Duration::from_secs(10);
    let mut last_update = Local::now();

    loop {
        app.clean_sent_notifications();
        let notifications = app.get_notifications_content();

        terminal.draw(|f| {
            ui(
                f,
                &notifications,
                &mut app.state,
                last_update,
                app.input_mode,
                app.input_field,
                &app.input_values,
            )
        })?;

        last_tick = app_input(&mut app, last_tick, tick_rate)?;

        if database_tick.elapsed() >= database_tick_rate {
            database_tick = Instant::now();
            last_update = Local::now();
            sync_database(&db, notifications);
        }

        if app.should_quit {
            return Ok(());
        }
    }
}

fn app_input(app: &mut App, last_tick: Instant, tick_rate: Duration) -> crossterm::Result<Instant> {
    let timeout = Duration::from_millis(250)
        .checked_sub(last_tick.elapsed())
        .unwrap_or_else(|| Duration::from_secs(0));

    if event::poll(timeout)? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                app.on_key(key.code);
            }
        }
    }

    if last_tick.elapsed() >= tick_rate {
        Ok(Instant::now())
    } else {
        Ok(last_tick)
    }
}

fn sync_database(db: &SqliteDb, notifications: Vec<NotificationContent>) {
    tokio_scoped::scope(|scope| {
        scope.spawn(async {
            db.synchronize_notifications(notifications)
                .await
                .expect("Error synchronizing notifications");
        });
    });
}
