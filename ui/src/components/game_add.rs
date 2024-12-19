use iced::{
    widget::{button, column, container, row, text, text_input},
    Alignment, Element, Length, Task as Command,
};
use iced_aw::card;
use std::path::PathBuf;

#[derive(Debug)]
pub struct GameAdd {
    pub name_input: String,
    pub path_input: PathBuf,
    pub icon_input: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GameAddMessage {
    NameInputChanged(String),
    NameInputSubmit,
    Add,
}

impl GameAdd {
    pub fn new() -> Self {
        GameAdd {
            name_input: String::new(),
            path_input: PathBuf::new(),
            icon_input: None,
        }
    }
    pub fn update(&mut self, msg: GameAddMessage) -> Command<GameAddMessage> {
        match msg {
            GameAddMessage::NameInputChanged(input) => {
                self.name_input = input;
                Command::none()
            }
            GameAddMessage::NameInputSubmit => todo!(),
            GameAddMessage::Add => unreachable!(),
        }
    }
    pub fn view(&self) -> Element<GameAddMessage> {
        let input_column = column![
            text_input("Enter name of the game", &self.name_input)
                .on_input(GameAddMessage::NameInputChanged),
            text_input("Enter path", &self.path_input.to_str().unwrap())
                .on_input(GameAddMessage::NameInputChanged),
        ]
        .align_x(Alignment::Center)
        .spacing(10.0);
        let content = column![
            text("Add game").size(24.0),
            input_column,
            button("Add").on_press(GameAddMessage::Add)
        ]
        .align_x(Alignment::Center)
        .spacing(5.0);
        container(content).into()
    }
}
