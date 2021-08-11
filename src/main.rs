use serde::{Deserialize, Serialize};
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

fn main() -> Result<(), Error> {
    let config: Config = toml::from_str(
        r#"
        [[files]]
        name = bspwm
        origin = "~/Build/dotfiles/bspwm/bspwmrc" 
        deployed = "~/.config/bspwm/bspwmrc"
    "#,
    )
    .unwrap();

    let config_structure = Config {
        files: vec![
            File {
                a: "hey".to_string(),
                b: "i".to_string(),
            },
            File {
                a: "hey".to_string(),
                b: "i".to_string(),
            },
        ],
    };

    let pretty_structure = toml::ser::to_string_pretty(&config_structure);

    println!("{:?}", a);

    Ok(())
}
