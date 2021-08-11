use serde::{Deserialize, Serialize};
use std::env;
use toml::{de::Error, Value};

#[derive(Debug, Deserialize, Serialize)]
struct File {
    origin: String,
    deployed: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    files: Vec<File>,
}

enum Verbs {
    Deploy,
    Save,
    None,
}

fn get_verb(verb) -> Verbs {
    match verb {
        "deploy" => Verbs::Deploy,
        "save" => Verbs::Save,
        _ => Verbs::None,
    }
}

fn main() -> Result<(), Error> {
    let mut argv = env::args();
    let argc = argv.len();

    if argc == 1 {
        println!("One arg");
    } else if argc >= 2 {
        let verb = get_verb(argv.nth(1).unwrap().as_str());
    }

    Ok(())
}
