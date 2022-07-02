mod syntax;
mod lexer;

use std::fs;

fn main() {
	println!("{}", syntax::parse(&fs::read_to_string("example.km").unwrap())); 
}
