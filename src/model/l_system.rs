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
	
	impl From<RepresentationError> for String {
		fn from(error: RepresentationError) -> Self { 
			format!("{}", error)
		}
	}
}

use error::RepresentationError;
use super::musical_notation;
use std::fmt;

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

// #--- Atom ---#

#[derive(Copy, Clone)]
pub struct Atom {
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
            symbol: char_representation,
        }
    }
}

impl fmt::Debug for Atom {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
		write!(f, "{}", self.symbol)
	}
}

// #--- Axiom ---#

pub struct Axiom {
    atom_list: Vec<Atom>,
}

impl Axiom {
    pub fn from(string_representation: &str) -> Result<Axiom, RepresentationError> {
        if string_representation.is_empty() {
            return Err(RepresentationError::new("Axiom is empty"));
        }

        let iter = string_representation.chars();
        let mut axiom = Axiom { atom_list: vec![] };

        for character in iter {
            axiom.atom_list.push(Atom::from_char(character));
        }

        return Ok(axiom);
    }
	
	pub fn apply(&mut self, rule: &Rule) {
		let mut new_atom_list: Vec<Atom> = vec![];
		
		for atom in &self.atom_list {
			if rule.lhs.symbol == atom.symbol {
				for atom in &rule.rhs.atom_list {
					new_atom_list.push(*atom);
				}
			} else {
				new_atom_list.push(*atom);
			}
		}
		
		self.atom_list = new_atom_list;
	}
}

impl fmt::Debug for Axiom {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
		for atom in &self.atom_list {
			match write!(f, "{:?}", atom) {
				Err(e) => return Err(e),
				Ok(_) => {},
			};
		}
		
		Ok(())
	}
}

// #--- Rule ---#

pub struct Rule {
    lhs: Atom,
    rhs: Axiom,
}

impl Rule {
    pub fn from(string_representation: &str) -> Result<Rule, RepresentationError> {
        match string_representation.split_once("->") {
            None => Err(RepresentationError::new("Rule didn't contain a '->'.")),
            Some((lhs_str, rhs_str)) => Ok(Rule {
                lhs: Atom::from_string(lhs_str.trim())?,
                rhs: Axiom::from(rhs_str.trim())?,
            }),
        }
    }
}

impl fmt::Debug for Rule {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
		write!(f, "{:?} -> {:?}", self.lhs, self.rhs)
	}
}

#[cfg(test)]
mod tests {
	use super::{Atom, Axiom, Rule};
	
	#[test]
	fn create_and_display_atom_test() -> Result<(), String> {
		assert_eq!(format!("{:?}", Atom::from_string("A")?), "A");
		assert_eq!(format!("{:?}", Atom::from_char('A')), "A");
		Ok(())
	}
	
	#[test]
	fn create_empty_atom_test() {
		match Atom::from_string("") {
			Err(e) => assert_eq!(format!("{}", e), "There was an Error with the Representation of an L-System Element: Atom is empty."),
			Ok(_) => panic!("Created empty atom."),
		}
	}
	
	#[test]
	fn create_overfull_atom_test() {
		match Atom::from_string("AABB") {
			Err(e) => assert_eq!(format!("{}", e), "There was an Error with the Representation of an L-System Element: Atom contains more that one character."),
			Ok(_) => panic!("Created overfull atom."),
		}
		
		match Atom::from_string("AC") {
			Err(e) => assert_eq!(format!("{}", e), "There was an Error with the Representation of an L-System Element: Atom contains more that one character."),
			Ok(_) => panic!("Created overfull atom."),
		}
		
		match Atom::from_string("CCC") {
			Err(e) => assert_eq!(format!("{}", e), "There was an Error with the Representation of an L-System Element: Atom contains more that one character."),
			Ok(_) => panic!("Created overfull atom."),
		}
	}
	
	#[test]
	fn create_and_display_axiom_test() -> Result<(), String> {
		assert_eq!(format!("{:?}", Axiom::from("ABA")?), "ABA");
		Ok(())
	}

	#[test]
	fn create_and_display_rule_test() -> Result<(), String> {
		assert_eq!(format!("{:?}", Rule::from("A->ABA")?), "A -> ABA");
		Ok(())
	}
	
	#[test]
	fn apply_rule_to_axiom_test() -> Result<(), String> {
		let mut axiom: Axiom = Axiom::from("ABA")?;
		let rule: Rule = Rule::from("A->ABA")?;
		axiom.apply(&rule);
		
		assert_eq!(format!("{:?}", axiom), "ABABABA");
		
		Ok(())
	}
}