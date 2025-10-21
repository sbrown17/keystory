mod app;
mod redis_ops;
mod git_ops;
mod ui;

use anyhow::Result;
use iced::{Application, Settings};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    app::KeyStoryApp::run(Settings::default())?;
    Ok(())
}