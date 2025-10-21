use iced::{
    widget::{button, column, row, text, text_input},
    Element,
};
use crate::app::{KeyStoryApp, Message};

pub fn view(app: &KeyStoryApp) -> Element<Message> {
    let content = if app.loading {
        text("Loading...").size(24).into()
    } else {
        column![
            text(format!("Key: {}", app.key)).size(20),
            text_input("Value", &app.value)
                .on_input(Message::ValueChanged)
                .padding(10)
                .size(16),
            row![
                button("Save & Commit").on_press(Message::Save),
                text(&app.status)
            ]
            .spacing(10)
        ]
        .spacing(20)
        .padding(20)
        .into()
    };

    column![content].into()
}
