mod syntax;
mod lexer;

use std::fs;
use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() == 2 {
		let input = fs::read_to_string(&args[1]).unwrap();
		let outpath = &args[1].replace(".km", ".html");
		if outpath != &args[1] {
			fs::write(&args[1].replace(".kami", ".html"), syntax::parse(&input)).unwrap();
		} else {
			panic!("Output path was the same as input path, are you sure the extension is .km?");
		}
	}
}
