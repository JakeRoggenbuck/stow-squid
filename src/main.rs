use home::home_dir;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::ffi::OsStr;
use std::fmt::Display;
use std::fs::{copy, File};
use std::io;
use std::io::{stdin, Read};
use std::path::Path;
use structopt::StructOpt;
use toml::de;

pub mod drop;
pub mod verb;

use drop::{deploy, diff, save};
use verb::{get_message_from_dot, get_verb, Verbs};

#[derive(Debug, Deserialize, Serialize)]
pub struct Dot {
    pub name: String,
    pub origin: String,
    pub deployed: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub files: Vec<Dot>,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "🦑 stow-squid", about = "Stow your dotfiles")]
struct Opt {
    #[structopt()]
    verb: String,

    #[structopt()]
    dot: Option<String>,
}

/// Get the path where the config should be
fn get_config_path() -> String {
    let config_path = "stow-squid/stow-squid.toml";
    let home_path = home_dir().unwrap().display().to_string();

    let user_config = home_path + &"/.config/".to_string() + config_path;

    if Path::new(&user_config).exists() {
        return user_config;
    } else {
        eprintln!("Config file not found at {}", user_config);
        eprintln!("An example can be found at https://github.com/JakeRoggenbuck/stow-squid#config-example");
    }

    String::new()
}

/// Open the configuration file as a toml struct
fn open_config() -> Result<Config, io::Error> {
    let mut config_file = File::open(get_config_path())?;
    let mut config: String = String::new();
    config_file.read_to_string(&mut config)?;
    let config_toml: Config = toml::from_str(config.as_str()).unwrap();
    Ok(config_toml)
}

fn main() -> Result<(), de::Error> {
    let opt: Opt = Opt::from_args();

    let config: Config = open_config().unwrap();
    let verb: Verbs = get_verb(&opt.verb);

    match verb {
        Verbs::Save => save(&config, &verb, opt.dot).unwrap(),
        Verbs::Deploy => deploy(&config, &verb, opt.dot).unwrap(),
        Verbs::Diff => diff(&config, &verb),
        _ => (),
    }

    Ok(())
}
