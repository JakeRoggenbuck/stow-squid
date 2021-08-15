use super::copy;
use super::get_message_from_dot;
use super::{io, stdin};
use super::{Config, Dot, Verbs};
use super::{Display, OsStr, Path};

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
    // TODO: Currently, there is not an implemented way to move entire directories
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
pub fn save(config: &Config, verb: &Verbs, dot_name: Option<String>) -> Result<(), io::Error> {
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
pub fn deploy(config: &Config, verb: &Verbs, dot_name: Option<String>) -> Result<(), io::Error> {
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
