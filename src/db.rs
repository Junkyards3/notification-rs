use sqlx::migrate::MigrateDatabase;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::{QueryBuilder, Sqlite, SqlitePool};

const DB_URL: &str = "sqlite://sqlite.db";

#[derive(sqlx::FromRow, Clone)]
pub struct NotificationContent {
    pub(crate) title: String,
    pub(crate) body: String,
    pub(crate) date: NaiveDateTime,
}

pub struct SqliteDb {
    pool: SqlitePool,
}

impl SqliteDb {
    pub async fn set_up_db() -> Result<SqliteDb, sqlx::Error> {
        if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
            if let Err(error) = Sqlite::create_database(DB_URL).await {
                panic!("error: {}", error)
            }
        } else {
        }

        let db = SqliteDb {
            pool: SqlitePool::connect(DB_URL).await?,
        };

        db.create_table(false).await?;

        Ok(db)
    }

    async fn create_table(&self, should_put_values: bool) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS notifications (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                body TEXT NOT NULL,
                date TEXT NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await?;
        if should_put_values {
            sqlx::query(
                r#"
                INSERT INTO notifications (title, body, date)
                VALUES ('Courses', 'Penser à acheter coca', '2023-07-16 13:00:00')
                "#,
            )
            .execute(&self.pool)
            .await?;
        }
        Ok(())
    }

    pub async fn load_notifications(&self) -> Result<Vec<NotificationContent>, sqlx::Error> {
        let notifications = sqlx::query_as(
            r#"
            SELECT title, body, date
            FROM notifications
            ORDER BY date
            "#,
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(notifications)
    }

    pub async fn synchronize_notifications(
        &self,
        notifications: Vec<NotificationContent>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
                DELETE FROM notifications
                "#,
        )
        .execute(&self.pool)
        .await?;

        if !notifications.is_empty() {
            QueryBuilder::new("INSERT INTO notifications (title, body, date) ")
                .push_values(notifications, |mut b, notification| {
                    b.push_bind(notification.title)
                        .push_bind(notification.body)
                        .push_bind(notification.date);
                })
                .build()
                .execute(&self.pool)
                .await?;
        }

        Ok(())
    }
}
