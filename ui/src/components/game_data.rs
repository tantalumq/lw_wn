use iced::{
    widget::{button, column, container, row, text},
    Element, Task as Command,
};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub struct GameData {
    id: Uuid,
    state: GameDataState,
    name: String,
    path: PathBuf,
    icon: String, // TODO
    minutes_played: i32,
}

#[derive(Debug, PartialEq)]
enum GameDataState {
    Playing,
    Idle,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GameDataMessage {
    Play,
    Exit,
    Edit,
    Edited((String, PathBuf, String)),
    FinishEdition,
    Delete,
}

impl GameData {
    pub fn new(name: &String, path: &PathBuf, _icon: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            state: GameDataState::Idle,
            name: name.to_string(),
            path: path.to_path_buf(),
            icon: String::new(),
            minutes_played: 0,
        }
    }
    pub fn update(&self, msg: GameDataMessage) -> Command<GameDataMessage> {
        match &self.state {
            GameDataState::Playing => match msg {
                GameDataMessage::Exit => todo!(),
                GameDataMessage::Play
                | GameDataMessage::Edit
                | GameDataMessage::Edited(_)
                | GameDataMessage::FinishEdition
                | GameDataMessage::Delete => unreachable!(),
            },
            GameDataState::Idle => match msg {
                GameDataMessage::Play => todo!(),
                GameDataMessage::Exit => unreachable!(),
                GameDataMessage::Edit => todo!(),
                GameDataMessage::Edited((new_name, new_path, new_icon)) => todo!(),
                GameDataMessage::FinishEdition => todo!(),
                GameDataMessage::Delete => todo!(),
            },
        }
    }
    pub fn view(&self) -> Element<GameDataMessage> {
        match &self.state {
            GameDataState::Playing => todo!(),
            GameDataState::Idle => {
                let info = row![text(&self.name), text(format!("{}m", &self.minutes_played))]
                    .spacing(10.0); // TODO
                let button_row = row![
                    button("Play").on_press(GameDataMessage::Play),
                    button("Edit").on_press(GameDataMessage::Edit)
                ]
                .spacing(4.0);

                let content = column![info, button_row].padding(4.0).spacing(4.0);
                container(content).padding(4.0).into()
            }
        }
    }
}
