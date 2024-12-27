use core::{Config, Error};

use crate::commands::parse_time;

use super::file_open;

pub fn execute() -> Result<(), Error> {
    let file = file_open()?;
    let config: Config =
        serde_yaml::from_reader(&file).map_err(|err| Error::FromReader(err.to_string()))?;
    if let Some(data) = config.data {
        for id in 0..data.len() {
            let game = data.get(id).unwrap();
            println!(
                "{id}. {}: {} ({})\n[{}]",
                game.name(),
                parse_time(game.time()),
                game.uuid(),
                game.path()
            )
        }
        Ok(())
    } else {
        return Err(Error::FromReader("[ERRPR] No games".into()));
    }
}
