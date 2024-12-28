use core::{print_hint, print_success, Config, Error};
use std::{fs::File, str::FromStr};

use uuid::Uuid;

use super::{file_open, GAME_DATA};

pub fn execute(id: &str) -> Result<(), Error> {
    let id = id
        .parse::<usize>()
        .map_err(|err| Error::Parse(err.to_string()))?;
    let file = file_open()?;
    let mut config: Config =
        serde_yaml::from_reader(&file).map_err(|err| Error::FromReader(err.to_string()))?;

    if let Some(ref mut data) = config.data {
        let uuid = data.get(id).ok_or(Error::Index)?.uuid();
        let name = data
            .remove(
                data.iter()
                    .position(|el| el.uuid() == uuid)
                    .ok_or(Error::GameIsNotExists)?,
            )
            .name();
        print_success(format!("Game '{name}' was removed").as_str());
    }
    let file = File::create(GAME_DATA).map_err(|err| Error::FileOpen(err.to_string()))?;
    serde_yaml::to_writer(&file, &config).map_err(|err| Error::ToWriter(err.to_string()))
}

pub fn execute_uuid(uuid: &str) -> Result<(), Error> {
    let uuid = Uuid::from_str(uuid).map_err(|err| Error::UuidFrom(err.to_string()))?;
    let file = file_open()?;
    let mut config: Config =
        serde_yaml::from_reader(&file).map_err(|err| Error::FromReader(err.to_string()))?;

    let file = File::create(GAME_DATA).map_err(|err| Error::FileOpen(err.to_string()))?;

    if let Some(ref mut data) = config.data {
        let index = data
            .iter()
            .position(|el| el.uuid() == uuid)
            .ok_or(Error::GameIsNotExists)?;
        let name = data.remove(index).name();
        print_success(format!("Game '{name}' was removed").as_str());
    }
    serde_yaml::to_writer(&file, &config).map_err(|err| Error::ToWriter(err.to_string()))
}

pub fn help() {
    print_hint("launcher remove 'uuid' or launcher remove -i 'index'");
}
