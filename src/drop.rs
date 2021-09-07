use super::get_message_from_dot;
use super::{copy, env};
use super::{io, stdin};
use super::{Config, Dot, Verbs};
use super::{Display, OsStr, Path};
use std::process::Command;

fn ask_yes_or_no() -> Result<bool, io::Error> {
    let mut line: String = String::new();
    stdin().read_line(&mut line)?;
    line.pop();

    match line.as_str() {
        "Y" | "y" | "" => Ok(true),
        "N" | "n" | _ => Ok(false),
    }
}

/// Ask a message for a dot according to the verb and get a yes or no response
/// Return a bool depending on the yes or no
fn ask_copy(verb: &Verbs, dot: &Dot) -> Result<bool, io::Error> {
    let message: String = get_message_from_dot(&verb, &dot);
    println!("{}", message);
    return ask_yes_or_no();
}

/// Run a function for each dot file only if the ask was returned as true
fn action_for_dot(
    config: &Config,
    action: &dyn Fn(&Dot) -> Result<(), io::Error>,
    verb: &Verbs,
    dot_name: Option<String>,
) -> Result<(), io::Error> {
    let mut found: bool = false;
    for dot in &config.files {
        // If a name is provided, continue everything but the name
        if dot_name.is_some() {
            if dot_name.as_ref().unwrap() == &dot.name {
                found = true;
            } else {
                continue;
            }
        }

        if ask_copy(&verb, &dot).unwrap() {
            action(&dot)?;
        }
    }

    // If a dotfile name is provided
    if dot_name.is_some() {
        // And it was not found in the config
        if !found {
            println!(
                "{} is not a name of a dotfile in the config.",
                dot_name.unwrap()
            );
        }
    }

    Ok(())
}

/// Move a file or directory
fn safely_copy(
    from: impl AsRef<Path> + AsRef<OsStr> + Display,
    to: impl AsRef<Path> + AsRef<OsStr>,
) -> Result<bool, io::Error> {
    if !Path::new(&from).exists() {
        eprintln!(
            "🦈 The file stow-squid is copying from does not exist: {}",
            from
        );
        return Ok(false);
    }

    // TODO: Currently, there is not an implemented way to move entire directories
    // so this check if the &from is a directory
    if Path::new(&from).is_dir() {
        eprintln!(
            "🦈 Currently cannot move entire directories. Did not move {}.",
            from
        );
    } else {
        copy(&from, &to)?;
    }

    Ok(true)
}

/// Ask for each dot file to run save_inner on it
pub fn save(config: &Config, verb: &Verbs, dot_name: Option<String>) -> Result<(), io::Error> {
    println!("🦑 Saving mode!");

    /// Copy the deployed file to the origin location
    fn save_inner(dot: &Dot) -> Result<(), io::Error> {
        if safely_copy(&dot.deployed, &dot.origin)? {
            println!("📦 Saved \"{}\"!", &dot.name);
        }
        Ok(())
    }

    action_for_dot(&config, &save_inner, &verb, dot_name)?;

    if config.gitpath.is_some() {
        println!("Add and commit changes? ");
        let can_commit = ask_yes_or_no()?;

        if can_commit {
            let path_source: &str = config.gitpath.as_ref().unwrap();
            let path = Path::new(&path_source);

            if env::set_current_dir(path).is_ok() {
                println!("Moved to {}", path.display());

                let add = Command::new("git")
                    .args(&["add", "-u"])
                    .output()
                    .expect("failed to add files");

                // TODO: Add -m to stow-squid for message
                // and or user provided message
                let commit_message = "stow-squid updated";

                let commit = Command::new("git")
                    .args(&["commit", "-m", commit_message])
                    .output()
                    .expect("failed to commit");

                if commit.status.success() {
                    // A successful add is only relevant if a commit is successful
                    if add.status.success() {
                        println!("Successfully added!");
                    }

                    println!("Successfully committed '{}'", commit_message);
                }
            }
        }
    }

    Ok(())
}

/// Ask for each dot file to run deploy_inner on it
pub fn deploy(config: &Config, verb: &Verbs, dot_name: Option<String>) -> Result<(), io::Error> {
    println!("🦑 Deploy mode!");

    /// Copy the origin file to the deployed location
    fn deploy_inner(dot: &Dot) -> Result<(), io::Error> {
        if safely_copy(&dot.origin, &dot.deployed)? {
            println!("🐬 Successfully deployed {}!", dot.name);
        }
        Ok(())
    }

    action_for_dot(&config, &deploy_inner, &verb, dot_name)?;
    Ok(())
}

pub fn diff(config: &Config, _verb: &Verbs) {
    // TODO: Find the diff of dot.deployed and dot.origin
    for dot in &config.files {
        println!("{}", dot.origin);
    }
}

pub fn list(config: &Config, _verb: &Verbs) {
    for dot in &config.files {
        println!("{: <8}\t{} -> {}", dot.name, dot.origin, dot.deployed);
    }
}
