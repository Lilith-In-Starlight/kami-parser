mod syntax;
mod lexer;

use std::fs;
use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	match args.len() {
		1 => panic!("No input file was given."),
		2 => {
			let input = fs::read_to_string(&args[1]).unwrap();
			let outpath = args[1].replace(".km", ".html");
			if outpath != args[1] {
				fs::write(outpath, syntax::parse(&input)).unwrap();
			} else {
				panic!("Output path was the same as input path ({})", outpath);
			}
		},
		3 => {
			let input = fs::read_to_string(&args[1]).unwrap();
			let outpath = args[2];
			if outpath != args[1] {
				fs::write(outpath, syntax::parse(&input)).unwrap();
			} else {
				panic!("Output path was the same as input path ({})", outpath);
			}
		},
		_ => {
			panic!("Too many arguments!");
		}
	}
	if args.len() == 3 {
		outpath = args[2].clone();
		set_out = true;
	}
	if args.len() == 2 {
		let input = fs::read_to_string(&args[1]).unwrap();
		if !set_out { outpath = args[1].replace(".km", ".html"); }
		if outpath != args[1] {
			fs::write(outpath, syntax::parse(&input)).unwrap();
		} else {
			panic!("Output path was the same as input path ({})", outpath);
		}
	} else {
		panic!("No input file was given.");
	}
}
