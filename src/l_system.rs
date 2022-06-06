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
                "There was an Error with the Representation of an L-System Element: {}.",
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
use std::collections::HashMap;
use std::fmt;

// #--- Atom ---#

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Atom {
    pub symbol: char,
}

impl Atom {
    fn from_string(string_representation: &str) -> Result<Atom, RepresentationError> {
        let mut i = string_representation.chars();

        if let Some(first) = i.next() {
            if None != i.next() {
                Err(RepresentationError::new(
                    "Atom contains more that one character",
                ))
            } else {
                Ok(Atom::from_char(first))
            }
        } else {
            Err(RepresentationError::new("Atom is empty"))
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
    pub atom_list: Vec<Atom>,
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

    pub fn apply_ruleset(&mut self, ruleset: &RuleSet) {
        let mut new_atom_list: Vec<Atom> = vec![];

        for atom in &self.atom_list {
            match ruleset.rules.get(&atom) {
                Some(axiom) => {
                    for atom in &axiom.atom_list {
                        new_atom_list.push(*atom);
                    }
                }
                None => new_atom_list.push(*atom),
            };
        }

        self.atom_list = new_atom_list;
    }

    pub fn atoms(&self) -> std::slice::Iter<Atom> {
        self.atom_list.iter()
    }
}

impl fmt::Debug for Axiom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for atom in &self.atom_list {
            match write!(f, "{:?}", atom) {
                Err(e) => return Err(e),
                Ok(_) => {}
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
            None => Err(RepresentationError::new("Rule didn't contain a '->'")),
            Some((lhs_str, rhs_str)) => Ok(Rule {
                lhs: Atom::from_string(lhs_str.trim())?,
                rhs: Axiom::from(rhs_str.trim())?,
            }),
        }
    }
}

impl fmt::Debug for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}->{:?}", self.lhs, self.rhs)
    }
}

pub struct RuleSet {
    rules: HashMap<Atom, Axiom>,
}

impl RuleSet {
    pub fn from(rule_list: Vec<Rule>) -> Result<RuleSet, RepresentationError> {
        let mut rules: HashMap<Atom, Axiom> = HashMap::new();

        for rule in rule_list {
            match rules.insert(rule.lhs, rule.rhs) {
                Some(_) => {
                    return Err(RepresentationError::new(&format!(
                        "RuleSet contains two Rules with the lhs-Atom '{:?}'",
                        &rule.lhs
                    )));
                }
                None => {}
            }
        }

        return Ok(RuleSet { rules });
    }
}

impl fmt::Debug for RuleSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let mut set_of_rules: Vec<(&Atom, &Axiom)> = self.rules.iter().collect();
        set_of_rules.sort_by(|(lhs_1, _), (lhs_2, _)| lhs_1.cmp(lhs_2));

        write!(
            f,
            "{}",
            set_of_rules
                .iter()
                .map(|(key, val)| format!("{:?}->{:?}", key, val))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{Atom, Axiom, Rule, RuleSet};

    #[test]
    fn create_and_display_atom_test() -> Result<(), String> {
        assert_eq!(format!("{:?}", Atom::from_string("A")?), "A");
        assert_eq!(format!("{:?}", Atom::from_char('A')), "A");
        Ok(())
    }

    #[test]
    fn create_empty_atom_test() {
        match Atom::from_string("") {
            Err(e) => assert_eq!(
                format!("{}", e),
                "There was an Error with the Representation of an L-System Element: Atom is empty."
            ),
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
    fn create_empty_axiom_test() {
        match Axiom::from("") {
			Err(e) => assert_eq!(format!("{}", e), "There was an Error with the Representation of an L-System Element: Axiom is empty."),
			Ok(_) => panic!("Created empty axiom."),
		}
    }

    #[test]
    fn create_and_display_rule_test() -> Result<(), String> {
        assert_eq!(format!("{:?}", Rule::from("A->ABA")?), "A->ABA");
        Ok(())
    }

    #[test]
    fn create_rule_without_seperator() {
        const EXPECTED_ERROR_MESSAGE: &str = "There was an Error with the Representation of an L-System Element: Rule didn't contain a '->'.";

        match Rule::from("") {
            Err(e) => assert_eq!(format!("{}", e), EXPECTED_ERROR_MESSAGE),
            Ok(_) => panic!("Created rule without seperator."),
        }

        match Rule::from("A ABA") {
            Err(e) => assert_eq!(format!("{}", e), EXPECTED_ERROR_MESSAGE),
            Ok(_) => panic!("Created rule without seperator."),
        }

        match Rule::from("AABA") {
            Err(e) => assert_eq!(format!("{}", e), EXPECTED_ERROR_MESSAGE),
            Ok(_) => panic!("Created rule without seperator."),
        }

        match Rule::from("A=>ABA") {
            Err(e) => assert_eq!(format!("{}", e), EXPECTED_ERROR_MESSAGE),
            Ok(_) => panic!("Created rule without seperator."),
        }
    }

    #[test]
    fn create_rule_with_empty_side_test() {
        match Rule::from("->ABA") {
            Err(e) => assert_eq!(
                format!("{}", e),
                "There was an Error with the Representation of an L-System Element: Atom is empty."
            ),
            Ok(_) => panic!("Created rule with empty side."),
        }

        match Rule::from("->") {
            Err(e) => assert_eq!(
                format!("{}", e),
                "There was an Error with the Representation of an L-System Element: Atom is empty."
            ),
            Ok(_) => panic!("Created rule with empty side."),
        }

        match Rule::from("A->") {
			Err(e) => assert_eq!(format!("{}", e), "There was an Error with the Representation of an L-System Element: Axiom is empty."),
			Ok(_) => panic!("Created rule with empty side."),
		}
    }

    #[test]
    fn create_rule_with_overfull_atom() {
        match Rule::from("AB->ABA") {
			Err(e) => assert_eq!(format!("{}", e), "There was an Error with the Representation of an L-System Element: Atom contains more that one character."),
			Ok(_) => panic!("Created rule with overfull atom."),
		}

        match Rule::from("ABA->ABA") {
			Err(e) => assert_eq!(format!("{}", e), "There was an Error with the Representation of an L-System Element: Atom contains more that one character."),
			Ok(_) => panic!("Created rule with overfull atom."),
		}
    }

    #[test]
    fn create_and_display_ruleset_test() -> Result<(), String> {
        assert_eq!(
            format!("{:?}", RuleSet::from(vec![Rule::from("A->ABA")?])?),
            "A->ABA"
        );
        assert_eq!(
            format!(
                "{:?}",
                RuleSet::from(vec![Rule::from("A->ABA")?, Rule::from("B->BAB")?])?
            ),
            "A->ABA, B->BAB"
        );
        Ok(())
    }

    #[test]
    fn create_ruleset_with_same_axioms_test() {
        match RuleSet::from(vec![Rule::from("A->ABA").unwrap(), Rule::from("A->BAB").unwrap()]) {
            Err(e) => assert_eq!(
                format!("{}", e),
                "There was an Error with the Representation of an L-System Element: RuleSet contains two Rules with the lhs-Atom 'A'."
            ),
            Ok(_) => panic!("Created ruleset with same axioms side."),
        }
    }

    #[test]
    fn apply_rule_to_axiom_test() -> Result<(), String> {
        let mut axiom: Axiom = Axiom::from("ABA")?;
        let rule: Rule = Rule::from("A->ABA")?;
        axiom.apply(&rule);

        assert_eq!(format!("{:?}", axiom), "ABABABA");

        Ok(())
    }

    #[test]
    fn apply_ruleset_to_axiom_test() -> Result<(), String> {
        let mut axiom: Axiom = Axiom::from("ABA")?;
        let ruleset: RuleSet = RuleSet::from(vec![Rule::from("A->ABA")?, Rule::from("B->BAB")?])?;
        axiom.apply_ruleset(&ruleset);

        assert_eq!(format!("{:?}", axiom), "ABABABABA");

        Ok(())
    }

    #[test]
    fn dragon_curve_test() -> Result<(), String> {
        let mut axiom: Axiom = Axiom::from("FL")?;
        let ruleset: RuleSet = RuleSet::from(vec![Rule::from("L->L+KF")?, Rule::from("K->FL-K")?])?;

        axiom.apply_ruleset(&ruleset);
        assert_eq!(format!("{:?}", axiom), "FL+KF");

        axiom.apply_ruleset(&ruleset);
        assert_eq!(format!("{:?}", axiom), "FL+KF+FL-KF");

        axiom.apply_ruleset(&ruleset);
        assert_eq!(format!("{:?}", axiom), "FL+KF+FL-KF+FL+KF-FL-KF");

        Ok(())
    }
}
