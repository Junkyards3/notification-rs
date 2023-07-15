use crate::db::{Notification, SqliteDb};

pub struct App {
    db: SqliteDb,
    pub notifications: Vec<Notification>,
    pub needs_redraw: bool,
}

impl App {
    pub async fn new() -> App {
        let db = SqliteDb::set_up_db().await.unwrap();
        let notifications = db.load_notifications().await.unwrap();
        App {
            db,
            notifications,
            needs_redraw: true,
        }
    }
}
