use iced::{
    widget::{button, column, container, row},
    Element, Length, Task as Command,
};

#[derive(Debug)]
pub struct Header {}

#[derive(Debug, Clone, PartialEq)]
pub enum HeaderMessage {
    SettingsOpen,
    GameAddOpen,
}

impl Header {
    pub fn new() -> Self {
        Header {}
    }
    pub fn update(&self, msg: HeaderMessage) -> Command<HeaderMessage> {
        match msg {
            HeaderMessage::SettingsOpen | HeaderMessage::GameAddOpen => unreachable!(),
        }
    }
    pub fn view(&self) -> Element<HeaderMessage> {
        let buttons_row = row![
            button("Add game").on_press(HeaderMessage::GameAddOpen),
            button("Settings")
                // TODO .style(button::default)
                .on_press(HeaderMessage::SettingsOpen),
        ]
        .padding(4.0)
        .spacing(4.0);
        container(buttons_row)
            .style(container::bordered_box)
            .align_right(Length::Fill)
            .into()
    }
}
