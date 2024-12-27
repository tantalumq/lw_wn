mod commands;

use core::{lock, print_error, unlock};
use std::env;

use commands::{add, help, list, remove, run, update};

// TODO: Add more print
// TODO: No panics

fn main() {
    lock();

    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let args: Vec<&str> = args.iter().map(|x| x.as_str()).collect();

    let output = match args.as_slice() {
        // Show all games
        &["list"] => list::execute(),
        // Add new game
        &["add", name, path] => add::execute(name, path),
        &["add", ..] => Ok(add::help()),
        // Run game
        &["run", "-uuid", uuid] => run::execute_uuid(uuid),
        &["run", id] => run::execute(id),
        &["run", ..] => Ok(run::help()),
        // Update game data
        &["update", "--uuid", uuid, "-n", path] => update::execute_uuid(uuid, "", path),
        &["update", "--uuid", uuid, name, "-p"] => update::execute_uuid(uuid, name, ""),
        &["update", "--uuid", uuid, name, path] => update::execute_uuid(uuid, name, path),
        &["update", id, "-n", path] => update::execute(id, "", path),
        &["update", id, name, "-p"] => update::execute(id, name, ""),
        &["update", id, name, path] => update::execute(id, name, path),
        &["update", ..] => Ok(update::help()),
        // Remove game
        &["remove", "--uuid", uuid] => remove::execute_uuid(uuid),
        &["remove", id] => remove::execute(id),
        &["remove", ..] => Ok(remove::help()),
        _ => Ok(help()),
    };

    if let Err(error) = output {
        print_error(error)
    }

    unlock();
}
