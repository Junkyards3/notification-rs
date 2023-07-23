mod app;
mod db;
mod notification;
mod terminal;
mod ui;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    terminal::run().await
}
