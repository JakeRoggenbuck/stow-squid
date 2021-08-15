use super::Dot;
use super::PartialEq;

#[derive(PartialEq, Debug)]
pub enum Verbs {
    Deploy,
    Save,
    Diff,
    None,
}

/// Return the verb enum from the string passed in
pub fn get_verb(verb: &str) -> Verbs {
    match verb {
        "deploy" => Verbs::Deploy,
        "save" => Verbs::Save,
        "diff" => Verbs::Diff,
        _ => Verbs::None,
    }
}

/// Return the message that corresponds to the verb
pub fn get_message_from_dot(verb: &Verbs, dot: &Dot) -> String {
    match verb {
        Verbs::Deploy => format!(
            "Would you like to deploy {} -> {}? ",
            dot.origin, dot.deployed
        ),
        Verbs::Save => format!("Would you like to save {}? ", dot.deployed),
        _ => "".to_string(),
    }
}
