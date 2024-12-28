use core::{print_hint, print_success, Config, Error};
use std::{
    fs::File,
    path::PathBuf,
    process::Command,
    str::FromStr,
    sync::{Arc, Mutex},
    thread::{self, sleep},
    time::Duration,
    usize,
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

    let path: String = if let Some(ref data) = config.data {
        let data = data.get(id).ok_or(Error::GameIsNotExists)?;
        data.path()
    } else {
        return Err(Error::DataGet("Games not added".to_string()));
    };

    let path = PathBuf::from_str(&path).map_err(|err| Error::PathFrom(err.to_string()))?;
    if let Some(extension) = path.extension() {
        if extension != "exe" {
            return Err(Error::FileNotExe);
        }
    };
    if !path.exists() {
        return Err(Error::PathIsNotExists);
    }

    let is_game_running = Arc::new(Mutex::new(true));
    let is_game_running_clone = Arc::clone(&is_game_running);

    let name = config.clone().data.unwrap().get(id).unwrap().name();

    print_success(format!("Game '{name}' launched").as_str());
    thread::spawn(move || {
        let mut child = Command::new(path).spawn().unwrap();
        let _ = child.wait();
        *is_game_running_clone.lock().unwrap() = false;
    });

    while *is_game_running.lock().unwrap() {
        config
            .data
            .as_mut()
            .unwrap()
            .get_mut(id)
            .ok_or(Error::GameIsNotExists)?
            .add_time(1);
        sleep(Duration::from_secs(1));
        let file = File::create(GAME_DATA).map_err(|err| Error::FileOpen(err.to_string()))?;
        serde_yaml::to_writer(&file, &config).map_err(|err| Error::ToWriter(err.to_string()))?;
    }
    Ok(print_success(format!("Game '{name}' was exited").as_str()))
}

pub fn execute_uuid(uuid: &str) -> Result<(), Error> {
    let uuid = Uuid::from_str(uuid).map_err(|err| Error::UuidFrom(err.to_string()))?;
    let file = file_open()?;

    let mut config: Config =
        serde_yaml::from_reader(&file).map_err(|err| Error::FromReader(err.to_string()))?;

    let path: String = if let Some(ref data) = config.data {
        let data = data
            .iter()
            .find(|el| el.uuid() == uuid)
            .ok_or(Error::GameIsNotExists)?;
        data.path()
    } else {
        return Err(Error::DataGet("Games not added".to_string()));
    };

    let path = PathBuf::from_str(&path).map_err(|err| Error::PathFrom(err.to_string()))?;

    if let Some(extension) = path.extension() {
        if extension != "exe" {
            return Err(Error::FileNotExe);
        }
    };
    if !path.exists() {
        return Err(Error::PathIsNotExists);
    }

    let is_game_running = Arc::new(Mutex::new(true));
    let is_game_running_clone = Arc::clone(&is_game_running);

    let name = config
        .clone()
        .data
        .unwrap()
        .iter()
        .find(|el| el.uuid() == uuid)
        .unwrap()
        .name();

    print_success(format!("Game '{name}' launched").as_str());
    thread::spawn(move || {
        let mut child = Command::new(path).spawn().unwrap();
        let _ = child.wait();
        *is_game_running_clone.lock().unwrap() = false;
    });

    while *is_game_running.lock().unwrap() {
        config
            .data
            .as_mut()
            .unwrap()
            .iter_mut()
            .find(|el| el.uuid() == uuid)
            .ok_or(Error::GameIsNotExists)?
            .add_time(1);
        sleep(Duration::from_secs(1));
        let file = File::create(GAME_DATA).map_err(|err| Error::FileOpen(err.to_string()))?;
        serde_yaml::to_writer(&file, &config).map_err(|err| Error::ToWriter(err.to_string()))?;
    }
    Ok(print_success(format!("Game '{name}' exited").as_str()))
}

pub fn help() {
    print_hint("core run 'id' or core run --uuid 'uuid''");
}
