use core::{Config, Error};
use std::{
    fs::{File, OpenOptions},
    str::FromStr,
};

use uuid::Uuid;

use super::{file_open, GAME_DATA};

pub fn execute(id: &str, new_name: &str, new_path: &str) -> Result<(), Error> {
    let id = id
        .parse::<usize>()
        .map_err(|err| Error::Parse(err.to_string()))?;

    let new_name: Option<String> = if new_name.trim().len() == 0 {
        None
    } else {
        Some(new_name.to_string())
    };
    let new_path: Option<String> = if new_path.trim().len() == 0 {
        None
    } else {
        Some(new_path.to_string())
    };

    let file = file_open()?;

    let mut config: Config =
        serde_yaml::from_reader(&file).map_err(|err| Error::FromReader(err.to_string()))?;

    if let Some(ref mut data) = config.data {
        data.get_mut(id)
            .expect("[ERROR] Incorrect index")
            .update(new_name, new_path);
    }
    let file = File::create(GAME_DATA).map_err(|err| Error::FileOpen(err.to_string()))?;
    serde_yaml::to_writer(&file, &config).map_err(|err| Error::ToWriter(err.to_string()))
}

pub fn execute_uuid(uuid: &str, new_name: &str, new_path: &str) -> Result<(), Error> {
    let uuid: Uuid = Uuid::from_str(uuid).map_err(|err| Error::UuidFrom(err.to_string()))?;

    let new_name: Option<String> = if new_name.trim().len() == 0 {
        None
    } else {
        Some(new_name.to_string())
    };
    let new_path: Option<String> = if new_path.trim().len() == 0 {
        None
    } else {
        Some(new_path.to_string())
    };

    let file = file_open()?;

    let mut config: Config =
        serde_yaml::from_reader(&file).map_err(|err| Error::FromReader(err.to_string()))?;

    if let Some(ref mut data) = config.data {
        data.iter_mut()
            .find(|el| el.uuid() == uuid)
            .expect("[ERROR] Game not found")
            .update(new_name, new_path);
    }
    let file = File::create(GAME_DATA).map_err(|err| Error::FileOpen(err.to_string()))?;
    serde_yaml::to_writer(&file, &config).map_err(|err| Error::ToWriter(err.to_string()))
}

pub fn help() {
    println!("[ERROR] Incorrect arguments.\n[HINT] launcher update --uuid 'uuid' ['name'|-n] ['path'|-p]\n[HINT] launcher update 'index' ['name'|-n] ['path'|-p]")
}
