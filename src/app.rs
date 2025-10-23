
use iced::{application, executor, Element, Task, Theme};
use crate::{redis_ops, git_ops, ui};
use anyhow::Result;

#[derive(Debug, Clone)]
pub enum Message {
    Load,
    Loaded(Result<(String, String)>),
    ValueChanged(String),
    Save,
    Saved(Result<()>),
}

pub struct KeyStoryApp {
    pub key: String,
    pub value: String,
    pub status: String,
    pub loading: bool,
}

pub fn main() -> iced::Result {
    application("KeyStory", update, view)
        .theme(theme)
        .init(init)
        .run()
}

/// Initialize the application state and any startup tasks.
fn init(_flags: ()) -> (KeyStoryApp, Task<Message>) {
    (
        KeyStoryApp {
            key: "example-key".into(),
            value: "".into(),
            status: "Loading...".into(),
            loading: true,
        },
        Task::perform(redis_ops::get_key("example-key".into()), Message::Loaded),
    )
}

/// Handle all messages (events) and return the next async task (if any).
fn update(app: &mut KeyStoryApp, message: Message) -> Task<Message> {
    match message {
        Message::Loaded(Ok((key, value))) => {
            app.key = key;
            app.value = value;
            app.status = "Ready".into();
            app.loading = false;
        }
        Message::Loaded(Err(e)) => {
            app.status = format!("Error loading key: {e}");
            app.loading = false;
        }
        Message::ValueChanged(v) => {
            app.value = v;
        }
        Message::Save => {
            app.status = "Saving...".into();
            let k = app.key.clone();
            let v = app.value.clone();
            return Task::perform(redis_ops::set_and_commit(k, v), Message::Saved);
        }
        Message::Saved(Ok(())) => {
            app.status = "Saved and committed".into();
        }
        Message::Saved(Err(e)) => {
            app.status = format!("Error saving: {e}");
        }
        _ => {}
    }

    Task::none()
}

/// Produce the UI tree.
fn view(app: &KeyStoryApp) -> Element<Message> {
    ui::view(app)
}

/// Define the theme.
fn theme(_app: &KeyStoryApp) -> Theme {
    Theme::Dark
}
