use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::env;
use std::fs::{copy, File};
use std::io;
use std::io::{stdin, Read};
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
    Diff,
    None,
}

fn get_verb(verb: &str) -> Verbs {
    match verb {
        "deploy" => Verbs::Deploy,
        "save" => Verbs::Save,
        "diff" => Verbs::Diff,
        _ => Verbs::None,
    }
}

fn ask(message: &str) -> Result<bool, io::Error> {
    let mut line = String::new();
    println!("{}", message);
    stdin().read_line(&mut line)?;
    line.pop();

    match line.as_str() {
        "Y" | "y" | "" => Ok(true),
        "N" | "n" | _ => Ok(false),
    }
}

fn action_for_dot(config: &Config, message: &str, action: &dyn Fn()) {
    for dot in &config.files {
        if ask(message).unwrap() {
            action();
        }
    }
}

fn save(config: &Config) -> Result<(), io::Error> {
    let message = "Would you like to copy {} -> {}? [Y/n]";
    fn save_inner() {}
    action_for_dot(&config, &message, &save_inner);

    Ok(())
}

fn deploy(config: &Config) {}

fn diff(config: &Config) {
    // TODO: Find the diff of dot.deployed and dot.origin
    for dot in &config.files {
        println!("{}", dot.origin);
    }
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
        } else if verb == Verbs::Diff {
            diff(&config);
        }
    }

    Ok(())
}
