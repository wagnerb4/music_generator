mod l_system;
mod musical_notation;

use musical_notation::volume;
use l_system::{Axiom, Rule};

pub fn test() {
	println!("{:?}", Rule::from("A->ABA").unwrap());
	println!("{:?}", Axiom::from("ABA").unwrap());
}
