use super::musical_notation;

pub mod error {
    use std::error::Error;
    use std::fmt;

    #[derive(Debug)]
    pub struct RepresentationError {
        message: String,
    }

    impl RepresentationError {
        pub fn new(message: &str) -> RepresentationError {
            RepresentationError {
                message: message.to_string(),
            }
        }
    }

    impl fmt::Display for RepresentationError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "There was an Error with the Representation of an L-System Element: {}",
                self.message
            )
        }
    }

    impl Error for RepresentationError {}
}

use error::RepresentationError;

pub trait ActionState {}

pub trait Action {
    fn get_state(&self) -> Option<Box<dyn ActionState>>;
    fn get_symbol(&self) -> char;
    fn gen_next_musical_element(&mut self) -> Option<musical_notation::MusicalElement>;
}

pub struct NoAction {
    symbol: char,
}

impl Action for NoAction {
    fn get_state(&self) -> Option<Box<dyn ActionState>> {
        Option::None
    }

    fn get_symbol(&self) -> char {
        self.symbol
    }

    fn gen_next_musical_element(&mut self) -> Option<musical_notation::MusicalElement> {
        Option::None
    }
}

pub struct Atom {
    action: Box<dyn Action>,
    symbol: char,
}

impl Atom {
    fn from_string(string_representation: &str) -> Result<Atom, RepresentationError> {
        let mut i = string_representation.chars();

        if let Some(first) = i.next() {
            if None != i.next() {
                Err(RepresentationError::new(
                    "Atom contains more that one character.",
                ))
            } else {
                Ok(Atom::from_char(first))
            }
        } else {
            Err(RepresentationError::new("Atom is empty."))
        }
    }

    fn from_char(char_representation: char) -> Atom {
        Atom {
            action: Box::new(NoAction {
                symbol: char_representation,
            }),
            symbol: char_representation,
        }
    }
}

pub struct Axiom {
    atom_list: Vec<Atom>,
}

impl Axiom {
    fn from(string_representation: &str) -> Result<Axiom, RepresentationError> {
        if string_representation.is_empty() {
            return Err(RepresentationError::new("Axiom is empty"));
        }

        let mut iter = string_representation.chars();
        let mut axiom = Axiom { atom_list: vec![] };

        for character in iter {
            axiom.atom_list.push(Atom::from_char(character));
        }

        return Ok(axiom);
    }
}

pub struct Rule {
    lhs: Atom,
    rhs: Axiom,
}

impl Rule {
    fn from(string_representation: &str) -> Result<Rule, RepresentationError> {
        match string_representation.split_once("->") {
            None => Err(RepresentationError::new("Rule didn't contain a '->'.")),
            Some((lhs_str, rhs_str)) => Ok(Rule {
                lhs: Atom::from_string(lhs_str.trim())?,
                rhs: Axiom::from(rhs_str.trim())?,
            }),
        }
    }
}
