use std::{
    borrow::Borrow,
    collections::{HashMap, VecDeque},
    path::PathBuf,
};

use iced::{
    alignment::{Horizontal, Vertical},
    widget::{column, container, row, scrollable},
    Alignment, Element, Length, Task as Command,
};
use iced_aw::{grid, grid_row, Grid, GridRow};
use index_map::IndexMap;

use super::game_data::{GameData, GameDataMessage};

#[derive(Debug)]
pub struct GameList {
    games: IndexMap<GameData>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GameListMessage {
    GameData(GameDataMessage),
}

impl GameList {
    pub fn new() -> Self {
        GameList {
            games: IndexMap::new(),
        }
    }
    pub fn add(&mut self, game_data: GameData) {
        let _ = self.games.insert(game_data);
    }
    pub fn update(&self, msg: GameListMessage) -> Command<GameListMessage> {
        match msg {
            GameListMessage::GameData(msg) => self
                .games
                .get(0)
                .unwrap()
                .update(msg)
                .map(GameListMessage::GameData),
        }
    }
    pub fn view(&self) -> Element<GameListMessage> {
        let mut grid = Grid::new();
        let mut data_vec = VecDeque::new();
        let mut row_vec = VecDeque::new();
        for (_, game) in &self.games {
            if data_vec.len() < 5 {
                data_vec.push_back(game);
                if data_vec.len() > 0 {
                    row_vec.pop_back();
                    row_vec.push_back(data_vec.clone());
                }
            }
            if data_vec.len() == 5 {
                row_vec.push_back(VecDeque::new());
                data_vec = VecDeque::new();
            }
        }
        for mut row in row_vec.clone() {
            let mut grid_row = GridRow::new();
            for game in row.clone() {
                grid_row = grid_row.push(game.view().map(|msg| GameListMessage::GameData(msg)));
                row.pop_front();
            }
            grid = grid.push(grid_row);
            row_vec.pop_front();
        }
        if self.games.len() <= 5 {
            container(scrollable(grid.width(Length::Shrink)).height(Length::Fill))
                .height(Length::Fill)
                .into()
        } else {
            container(
                scrollable(grid.width(Length::Fill))
                    .height(Length::Fill)
                    .spacing(15.0),
            )
            .align_right(Length::Fill)
            .height(Length::Fill)
            .into()
        }
    }
}
