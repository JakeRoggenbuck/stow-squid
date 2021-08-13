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

fn ask(verb: &Verbs, dot: &Dot) -> Result<bool, io::Error> {
    let mut line = String::new();
    let message = match verb {
        Verbs::Deploy => format!(
            "Would you like to deploy {} -> {}? ",
            dot.origin, dot.deployed
        ),
        _ => "".to_string(),
    };
    println!("{}", message);
    stdin().read_line(&mut line)?;
    line.pop();

    match line.as_str() {
        "Y" | "y" | "" => Ok(true),
        "N" | "n" | _ => Ok(false),
    }
}

fn action_for_dot(config: &Config, action: &dyn Fn(), verb: &Verbs) {
    for dot in &config.files {
        if ask(&verb, &dot).unwrap() {
            action();
        }
    }
}

fn save(config: &Config, verb: &Verbs) -> Result<(), io::Error> {
    fn save_inner() {}
    action_for_dot(&config, &save_inner, &verb);

    Ok(())
}

fn deploy(config: &Config, verb: &Verbs) {}

fn diff(config: &Config, verb: &Verbs) {
    // TODO: Find the diff of dot.deployed and dot.origin
    for dot in &config.files {
        println!("{}", dot.origin);
    }
}

fn open_config() -> Result<Config, io::Error> {
    let mut config_file = File::open("/home/jake/.config/stow-squid/stow-squid.toml")?;
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
            save(&config, &verb).unwrap();
        } else if verb == Verbs::Deploy {
            deploy(&config, &verb);
        } else if verb == Verbs::Diff {
            diff(&config, &verb);
        }
    }

    Ok(())
}
