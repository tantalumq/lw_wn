mod components;

use components::{
    game_add::{GameAdd, GameAddMessage},
    game_data::GameData,
    game_list::{GameList, GameListMessage},
    header::{Header, HeaderMessage},
    modal,
    settings::Settings,
};
use iced::{
    self,
    advanced::Overlay,
    widget::{column, container, overlay, text},
    window::{self, Position},
    Element, Size, Task as Command,
};
fn main() -> iced::Result {
    iced::application(App::title, App::update, App::view)
        .window(window::Settings {
            position: Position::Centered,
            size: Size::new(640.0, 512.0),
            resizable: false,
            ..Default::default()
        })
        .run_with(App::new)
}

#[derive(Debug)]
struct App {
    game_list: GameList,
    header: Header,
    game_add: Option<GameAdd>,
    settings: Option<Settings>,
}

#[derive(Debug, Clone, PartialEq)]
enum AppMessage {
    GameList(GameListMessage),
    Header(HeaderMessage),
    GameAdd(GameAddMessage),
    GameAddClose,
}

impl App {
    fn new() -> (Self, Command<AppMessage>) {
        (
            App {
                game_list: GameList::new(),
                header: Header::new(),
                game_add: None,
                settings: None,
            },
            Command::none(),
        )
    }
    fn title(&self) -> String {
        String::from("lw_wn")
    }
    fn update(&mut self, msg: AppMessage) -> Command<AppMessage> {
        match msg {
            AppMessage::GameList(msg) => self.game_list.update(msg).map(AppMessage::GameList),
            AppMessage::Header(msg) => match msg {
                HeaderMessage::SettingsOpen => todo!(),
                HeaderMessage::GameAddOpen => {
                    self.game_add = Some(GameAdd::new());
                    Command::none()
                }
            },
            AppMessage::GameAddClose => {
                self.game_add = None;
                Command::none()
            }
            AppMessage::GameAdd(msg) => match msg {
                GameAddMessage::Add => {
                    let game_add = self.game_add.as_mut().unwrap();
                    let game_data = GameData::new(&game_add.name_input, &game_add.path_input, None);
                    self.game_list.add(game_data);
                    self.game_add = None;
                    Command::none()
                }
                _ => {
                    if let Some(ref mut game_add) = self.game_add {
                        game_add.update(msg).map(AppMessage::GameAdd)
                    } else {
                        Command::none()
                    }
                }
            },
        }
    }
    fn view(&self) -> Element<AppMessage> {
        let underlay = column![
            self.header.view().map(AppMessage::Header),
            self.game_list.view().map(AppMessage::GameList),
        ]
        .padding(4.0);
        return if let Some(game_add) = &self.game_add {
            let overlay = container(game_add.view().map(AppMessage::GameAdd))
                .padding(10.0)
                .width(320.0)
                .style(container::rounded_box);
            modal(underlay, overlay, AppMessage::GameAddClose)
        } else {
            underlay.into()
        };
    }
}
