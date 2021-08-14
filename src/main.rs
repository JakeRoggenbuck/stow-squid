use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::env;
use std::ffi::OsStr;
use std::fs::{copy, create_dir_all, read_dir, File};
use std::io;
use std::io::{stdin, Read};
use std::path::Path;
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

#[derive(PartialEq, Debug)]
enum Verbs {
    Deploy,
    Save,
    Diff,
    None,
}

/// Return the verb enum from the string passed in
fn get_verb(verb: &str) -> Verbs {
    match verb {
        "deploy" => Verbs::Deploy,
        "save" => Verbs::Save,
        "diff" => Verbs::Diff,
        _ => Verbs::None,
    }
}

/// Return the message that corresponds to the verb
fn get_message_from_dot(verb: &Verbs, dot: &Dot) -> String {
    match verb {
        Verbs::Deploy => format!(
            "Would you like to deploy {} -> {}? ",
            dot.origin, dot.deployed
        ),
        Verbs::Save => format!("Would you like to save {}? ", dot.deployed),
        _ => "".to_string(),
    }
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
) -> Result<(), io::Error> {
    for dot in &config.files {
        if ask(&verb, &dot).unwrap() {
            action(&dot)?;
        }
    }
    Ok(())
}

/// Move a file or directory
fn safely_copy(
    from: impl AsRef<Path>,
    to: impl AsRef<Path> + AsRef<OsStr>,
) -> Result<(), io::Error> {
    if !Path::new(&to).exists() {
        create_dir_all(&to)?;
    }

    // for entry in read_dir(from)? {
    //     let entry = entry?;
    //     if entry.file_type()?.is_dir() {
    //         let next_to: impl AsRef<Path> + AsRef<OsStr> = to.as_ref().join(entry.file_name());
    //         safely_copy(entry.path(), next_to)?;
    //     } else {
    //         copy(entry.path(), to.as_ref().join(entry.file_name()))?;
    //     }
    // }

    Ok(())
}

/// Ask for each dot file to run save_inner on it
fn save(config: &Config, verb: &Verbs) -> Result<(), io::Error> {
    /// Copy the deployed file to the origin location
    fn save_inner(dot: &Dot) -> Result<(), io::Error> {
        safely_copy(&dot.deployed, &dot.origin)?;
        Ok(())
    }
    action_for_dot(&config, &save_inner, &verb)?;
    Ok(())
}

/// Ask for each dot file to run deploy_inner on it
fn deploy(config: &Config, verb: &Verbs) -> Result<(), io::Error> {
    /// Copy the origin file to the deployed location
    fn deploy_inner(dot: &Dot) -> Result<(), io::Error> {
        copy(&dot.origin, &dot.deployed)?;
        println!("Successfully deployed {}!", dot.name);
        Ok(())
    }
    action_for_dot(&config, &deploy_inner, &verb)?;
    Ok(())
}

fn diff(config: &Config, verb: &Verbs) {
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
    let mut argv = env::args();
    let argc: usize = argv.len();

    if argc == 1 {
        println!("One arg");
    } else if argc >= 2 {
        let config: Config = open_config().unwrap();

        let verb: Verbs = get_verb(argv.nth(1).unwrap().as_str());
        if verb == Verbs::Save {
            save(&config, &verb).unwrap();
        } else if verb == Verbs::Deploy {
            deploy(&config, &verb).unwrap();
        } else if verb == Verbs::Diff {
            diff(&config, &verb);
        }
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
