use core::{Config, Error};
use std::{fs::File, path::PathBuf, process::Command, str::FromStr, time::Instant, usize};

use uuid::Uuid;

use super::FILENAME;

pub fn execute(uuid: &str) -> Result<(), Error> {
    let uuid = Uuid::from_str(uuid).map_err(|err| Error::UuidFrom(err.to_string()))?;
    let file = File::open(FILENAME).map_err(|err| Error::FileOpen(err.to_string()))?;
    let mut config: Config =
        serde_yaml::from_reader(&file).map_err(|err| Error::FromReader(err.to_string()))?;

    let file = File::create(FILENAME).map_err(|err| Error::FileOpen(err.to_string()))?;

    let path: String = if let Some(ref data) = config.data {
        let data = data
            .iter()
            .find(|el| el.uuid() == uuid)
            .expect("[ERROR] Game not found");
        data.path()
    } else {
        return Err(Error::DataGet("[ERROR] Games not added".to_string()));
    };

    let path = PathBuf::from_str(&path).map_err(|err| Error::PathFrom(err.to_string()))?;
    if let Some(extension) = path.extension() {
        if extension != "exe" {
            return Err(Error::FileNotExe);
        }
    };

    let now = Instant::now();

    let output = Command::new(path)
        .output()
        .map_err(|err| Error::CommandRun(err.to_string()))?;

    let elapsed = now.elapsed().as_secs_f32();
    config
        .data
        .as_mut()
        .unwrap()
        .iter_mut()
        .find(|el| el.uuid() == uuid)
        .unwrap()
        .add_time(elapsed);
    serde_yaml::to_writer(&file, &config).map_err(|err| Error::ToWriter(err.to_string()))?;

    Ok(println!("[STATUS] {}\n{:?}", output.status, elapsed))
}

pub fn execute_id(id: &str) -> Result<(), Error> {
    let id = id
        .parse::<usize>()
        .map_err(|err| Error::Parse(err.to_string()))?;
    let file = File::open(FILENAME).map_err(|err| Error::FileOpen(err.to_string()))?;
    let config: Config =
        serde_yaml::from_reader(&file).map_err(|err| Error::FromReader(err.to_string()))?;

    return if let Some(data) = config.data {
        let uuid = data
            .get(id)
            .expect("[ERROR] Incorrect index")
            .uuid()
            .to_string();
        execute(&uuid)
    } else {
        return Err(Error::DataGet("[ERROR] Games not added".to_string()));
    };
}

pub fn help() {
    println!("[ERROR] Incorrect arguments.\n[HINT] launcher run 'uuid' or launcher run -i 'index'")
}
