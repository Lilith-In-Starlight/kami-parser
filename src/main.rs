mod lexer;
mod syntax;
use std::fs;

fn main() {
	let input = fs::read_to_string("example.kami").unwrap();
	fs::write("example.html", lexer::parse_multiline(&input)).unwrap();
}
