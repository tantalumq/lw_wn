use core::{lock, print_error, unlock};
use std::env;

mod commands;

fn main() {
    _ = lock().inspect_err(|err| print_error(err));
    let mut args: Vec<String> = env::args()
        .map(|el| match el.to_lowercase().as_str() {
            "core" | "exit" | "list" | "add" | "run" | "update" | "remove" => {
                el.trim().to_lowercase()
            }
            _ => el.trim().to_string(),
        })
        .collect();
    args.remove(0);

    let output = commands::execute(&mut args, false);

    if let Err(error) = output {
        print_error(error)
    }

    _ = unlock().inspect_err(|err| print_error(err));
}
