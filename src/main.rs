mod syntax;
mod multiline_lexer;
mod lexer;

use std::fs;

fn main() {
	let out = syntax::parse(&fs::read_to_string("example.km").unwrap());
	for i in out.lines() {
		println!("{}", i);
	}
	print!("{:?}", out); 
}
