use core::{print_success, Config, Error};

use crate::commands::parse_time;

use super::file_open;

pub fn execute() -> Result<(), Error> {
    let file = file_open()?;
    let config: Config =
        serde_yaml::from_reader(&file).map_err(|err| Error::FromReader(err.to_string()))?;
    if let Some(data) = config.data {
        for id in 0..data.len() {
            let game = data.get(id).unwrap();
            print_success(
                format!(
                    "{id}. {}: {} ({})\n\
                    \t  ['{}']",
                    game.name(),
                    parse_time(game.time()),
                    game.uuid(),
                    game.path(),
                )
                .as_str(),
            )
        }
        Ok(())
    } else {
        return Err(Error::FromReader("No games".into()));
    }
}
