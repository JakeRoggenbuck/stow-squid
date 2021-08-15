use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::ffi::OsStr;
use std::fmt::Display;
use std::fs::{copy, create_dir_all, read_dir, File};
use std::io::{stdin, Read};
use std::path::Path;
use std::{env, io};
use structopt::StructOpt;
use toml::de;

pub mod verb;

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

/// Ask a message for a dot according to the verb and get a yes or no response
/// Return a bool depending on the yes or no
fn ask(verb: &Verbs, dot: &Dot) -> Result<bool, io::Error> {
    let mut line: String = String::new();
    let message: String = get_message_from_dot(&verb, &dot);

    println!("{}", message);
    stdin().read_line(&mut line)?;
    line.pop();

    match line.as_str() {
        "Y" | "y" | "" => Ok(true),
        "N" | "n" | _ => Ok(false),
    }
}

/// Run a function for each dot file only if the ask was returned as true
fn action_for_dot(
    config: &Config,
    action: &dyn Fn(&Dot) -> Result<(), io::Error>,
    verb: &Verbs,
    dot_name: Option<String>,
) -> Result<(), io::Error> {
    for dot in &config.files {
        // If a name is provided, continue everything but the name
        if dot_name.is_some() {
            if dot_name.as_ref().unwrap() != &dot.name {
                continue;
            }
        }

        if ask(&verb, &dot).unwrap() {
            action(&dot)?;
        }
    }
    Ok(())
}

/// Move a file or directory
fn safely_copy(
    from: impl AsRef<Path> + AsRef<OsStr> + Display,
    to: impl AsRef<Path> + AsRef<OsStr>,
) -> Result<(), io::Error> {
    // Currently, there is not an implemented way to move entire directories
    // so this check if the &from is a directory
    if Path::new(&from).is_dir() {
        eprintln!(
            "🦈 Currently cannot move entire directories. Did not move {}.",
            from
        );
    } else {
        copy(&from, &to)?;
        println!("📦 Moved \"{}\"!", from);
    }

    Ok(())
}

/// Ask for each dot file to run save_inner on it
fn save(config: &Config, verb: &Verbs, dot_name: Option<String>) -> Result<(), io::Error> {
    println!("🦑 Saving move!");

    /// Copy the deployed file to the origin location
    fn save_inner(dot: &Dot) -> Result<(), io::Error> {
        safely_copy(&dot.deployed, &dot.origin)?;
        Ok(())
    }

    action_for_dot(&config, &save_inner, &verb, dot_name)?;
    Ok(())
}

/// Ask for each dot file to run deploy_inner on it
fn deploy(config: &Config, verb: &Verbs, dot_name: Option<String>) -> Result<(), io::Error> {
    println!("🦑 Deploy move!");

    /// Copy the origin file to the deployed location
    fn deploy_inner(dot: &Dot) -> Result<(), io::Error> {
        copy(&dot.origin, &dot.deployed)?;
        println!("Successfully deployed {}!", dot.name);
        Ok(())
    }

    action_for_dot(&config, &deploy_inner, &verb, dot_name)?;
    Ok(())
}

fn diff(config: &Config, _verb: &Verbs) {
    // TODO: Find the diff of dot.deployed and dot.origin
    for dot in &config.files {
        println!("{}", dot.origin);
    }
}

/// Open the configuration file as a toml struct
fn open_config() -> Result<Config, io::Error> {
    let mut config_file = File::open("/home/jake/.config/stow-squid/stow-squid.toml")?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_verb_test() {
        assert_eq!(get_verb("save"), Verbs::Save);
        assert_eq!(get_verb("deploy"), Verbs::Deploy);
        assert_eq!(get_verb("diff"), Verbs::Diff);
        assert_eq!(get_verb("something"), Verbs::None);
        assert_eq!(get_verb("something else"), Verbs::None);
        assert_eq!(get_verb("anything"), Verbs::None);
    }

    #[test]
    fn get_message_from_dot_test() {
        let dot: Dot = Dot {
            name: "bspwm".to_string(),
            origin: "this".to_string(),
            deployed: "that".to_string(),
        };
        assert_eq!(
            get_message_from_dot(&Verbs::Save, &dot),
            "Would you like to save that? "
        );
        assert_eq!(
            get_message_from_dot(&Verbs::Deploy, &dot),
            "Would you like to deploy this -> that? "
        );
        assert_eq!(get_message_from_dot(&Verbs::None, &dot), "");
        assert_eq!(get_message_from_dot(&Verbs::Diff, &dot), "");
    }
}
