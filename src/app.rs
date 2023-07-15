use crate::db::{Notification, SqliteDb};
use tui::widgets::ListState;

pub struct App {
    db: SqliteDb,
    pub state: ListState,
    pub notifications: Vec<Notification>,
    pub should_quit: bool,
}

impl App {
    pub async fn new() -> App {
        let db = SqliteDb::set_up_db().await.unwrap();
        let notifications = db.load_notifications().await.unwrap();
        App {
            db,
            notifications,
            state: ListState::default(),
            should_quit: false,
        }
    }

    async fn delete_selected_notification(&mut self) -> Result<(), sqlx::Error> {
        if let Some(i) = self.state.selected() {
            let id = self.notifications[i].id;
            self.db.delete_notification(id).await?;
            self.notifications.remove(i);
            self.state.select(Some(0));
        }
        Ok(())
    }
    pub async fn on_key(&mut self, c: char) {
        match c {
            'q' => self.should_quit = true,
            'x' => {
                self.delete_selected_notification().await.unwrap();
            }
            _ => {}
        }
    }

    pub fn on_up(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i > 0 {
                    i - 1
                } else {
                    self.notifications.len() - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn on_down(&mut self) {
        if let Some(i) = self.state.selected() {
            if i >= self.notifications.len() - 1 {
                self.state.select(Some(0));
            } else {
                self.state.select(Some(i + 1));
            }
        } else {
            self.state.select(Some(0));
        }
    }
}
