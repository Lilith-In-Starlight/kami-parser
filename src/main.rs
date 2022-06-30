mod syntax;

use std::fs;
use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() == 2 {
		let input = fs::read_to_string(&args[1]).unwrap();
		fs::write(&args[1].replace(".kami", ".html"), syntax::parse(&input)).unwrap();
	}
}
