use sqlx::migrate::MigrateDatabase;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::{Sqlite, SqlitePool};

const DB_URL: &str = "sqlite://sqlite.db";

#[derive(sqlx::FromRow, Debug)]
pub struct Notification {
    pub(crate) id: u32,
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
            println!("Creating database {}", DB_URL);
            match Sqlite::create_database(DB_URL).await {
                Ok(_) => println!("Create db success"),
                Err(error) => panic!("error: {}", error),
            }
        } else {
            println!("Database already exists");
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

    pub async fn load_notifications(&self) -> Result<Vec<Notification>, sqlx::Error> {
        let notifications = sqlx::query_as(
            r#"
            SELECT id, title, body, date
            FROM notifications
            ORDER BY date
            "#,
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(notifications)
    }

    pub async fn delete_notification(&self, id: u32) -> Result<bool, sqlx::Error> {
        let query_result = sqlx::query(
            r#"
                DELETE FROM notifications
                WHERE id = $1
                "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(query_result.rows_affected() > 0)
    }
}
