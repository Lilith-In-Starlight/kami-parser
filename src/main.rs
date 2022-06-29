mod lexer;
mod syntax;
use std::fs;

fn main() {
	let input = fs::read_to_string("example.kami").unwrap();
	println!("{:#?}", lexer::tokenize(&input));
}
