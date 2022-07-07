mod syntax;
mod multiline_lexer;
mod lexer;

use std::fs;

fn main() {
	let out = syntax::parse(&fs::read_to_string("example.km").unwrap());
	out.0.lines().map(|x| print!("{}\n", x)).for_each(drop);
	println!("\n");
	out.1.lines().map(|x| println!("{:?}", x)).for_each(drop);
}
