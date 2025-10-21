use iced::{executor, Application, Command, Element, Theme};
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

impl Application for KeyStoryApp {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            Self {
                key: "example-key".into(),
                value: "".into(),
                status: "Loading...".into(),
                loading: true,
            },
            Command::perform(redis_ops::get_key("example-key".into()), Message::Loaded),
        )
    }

    fn title(&self) -> String {
        "KeyStory".into()
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::Loaded(Ok((key, value))) => {
                self.key = key;
                self.value = value;
                self.status = "Ready".into();
                self.loading = false;
            }
            Message::Loaded(Err(e)) => {
                self.status = format!("Error loading key: {e}");
                self.loading = false;
            }
            Message::ValueChanged(v) => {
                self.value = v;
            }
            Message::Save => {
                self.status = "Saving...".into();
                let k = self.key.clone();
                let v = self.value.clone();
                return Command::perform(redis_ops::set_and_commit(k, v), Message::Saved);
            }
            Message::Saved(Ok(())) => {
                self.status = "Saved and committed".into();
            }
            Message::Saved(Err(e)) => {
                self.status = format!("Error saving: {e}");
            }
            _ => {}
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        ui::view(self)
    }
}