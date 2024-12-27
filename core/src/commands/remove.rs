use core::{Config, Error};
use std::{
    fs::{File, OpenOptions},
    str::FromStr,
};

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
        data.remove(
            data.iter()
                .position(|el| el.uuid() == data.get(id).expect("[ERROR] Incorrect index").uuid())
                .expect("[ERROR] Game not found"),
        );
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
            .expect("[ERROR] Game not found");
        data.remove(index);
    }
    serde_yaml::to_writer(&file, &config).map_err(|err| Error::ToWriter(err.to_string()))
}

pub fn help() {
    println!(
        "[ERROR] Incorrect arguments.\n[HINT] launcher remove 'uuid' or launcher remove -i 'index'"
    )
}
