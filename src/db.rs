use sqlx::migrate::MigrateDatabase;
use sqlx::{Sqlite, SqlitePool};

const DB_URL: &str = "sqlite://sqlite.db";

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

        db.create_table().await?;

        Ok(db)
    }

    async fn create_table(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS news (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                body TEXT NOT NULL,
                date TEXT NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
