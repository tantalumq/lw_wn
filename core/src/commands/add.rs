use super::FILENAME;

use core::{Config, Data, Error};
use serde_yaml;

use std::{
    fs::{metadata, File, OpenOptions},
    io::Write,
};

pub fn execute(name: &str, path: &str) -> Result<(), Error> {
    let mut file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(FILENAME)
        .map_err(|err| Error::FileOpen(err.to_string()))?;

    if metadata(FILENAME)
        .map_err(|err| Error::Metadata(err.to_string()))?
        .len()
        == 0
    {
        file.write_all("data: Null".as_bytes())
            .map_err(|err| Error::FileWrite(err.to_string()))?;
    }

    let mut config: Config =
        serde_yaml::from_reader(&file).map_err(|err| Error::FromReader(err.to_string()))?;

    let file = File::create(FILENAME).map_err(|err| Error::FileOpen(err.to_string()))?;

    let config = Config {
        data: if let Some(data) = &mut config.data {
            data.push(Data::new(name, path));
            Some(data.to_vec())
        } else {
            Some(vec![Data::new(name, path)])
        },
    };
    serde_yaml::to_writer(file, &config).map_err(|err| Error::ToWriter(err.to_string()))
}

pub fn help() {
    println!("[ERROR] Incorrect arguments.\n[HINT] launcher add 'name' 'path'")
}
