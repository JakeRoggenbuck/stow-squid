use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use toml::de;

#[derive(Debug, Deserialize, Serialize)]
struct Dot {
    name: String,
    origin: String,
    deployed: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    files: Vec<Dot>,
}

#[derive(PartialEq)]
enum Verbs {
    Deploy,
    Save,
    None,
}

fn get_verb(verb: &str) -> Verbs {
    match verb {
        "deploy" => Verbs::Deploy,
        "save" => Verbs::Save,
        _ => Verbs::None,
    }
}

fn save(config: &Config) {
    // TODO: move dot.deployed -> dot.origin
    for dot in &config.files {
        println!("{}", dot.deployed);
    }
}

fn deploy(config: &Config) {
    // TODO: move dot.origin -> dot.deployed
    for dot in &config.files {
        println!("{}", dot.origin);
    }
}

fn diff() {
    // TODO: Find the diff of dot.deployed and dot.origin
}

fn open_config() -> Result<Config, io::Error> {
    let mut config_file = File::open("/home/jake/.config/dotsin/dotsin.toml")?;
    let mut config = String::new();
    config_file.read_to_string(&mut config)?;
    let config_toml: Config = toml::from_str(config.as_str()).unwrap();
    Ok(config_toml)
}

fn main() -> Result<(), de::Error> {
    let mut argv = env::args();
    let argc = argv.len();

    if argc == 1 {
        println!("One arg");
    } else if argc >= 2 {
        let config = open_config().unwrap();

        let verb: Verbs = get_verb(argv.nth(1).unwrap().as_str());
        if verb == Verbs::Save {
            save(&config);
        } else if verb == Verbs::Deploy {
            deploy(&config);
        }
    }

    Ok(())
}
