use std::io;

mod app;
mod db;
mod notification;
mod terminal;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    terminal::run().await
}
