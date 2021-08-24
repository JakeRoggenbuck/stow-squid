use super::Dot;
use super::PartialEq;

#[derive(PartialEq, Debug)]
pub enum Verbs {
    Deploy,
    Save,
    Diff,
    List,
    None,
}

/// Return the verb enum from the string passed in
pub fn get_verb(verb: &str) -> Verbs {
    match verb {
        "deploy" => Verbs::Deploy,
        "save" => Verbs::Save,
        "diff" => Verbs::Diff,
        "list" => Verbs::List,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_verb_test() {
        assert_eq!(get_verb("save"), Verbs::Save);
        assert_eq!(get_verb("deploy"), Verbs::Deploy);
        assert_eq!(get_verb("diff"), Verbs::Diff);
        assert_eq!(get_verb("list"), Verbs::List);
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
