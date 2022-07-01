use crate::lexer::tokenize;
use crate::lexer::Token;
use crate::lexer::TokenType;

fn parse_attr(inp: &str) -> String {
	if inp == "{}" || inp == "" { 
		return String::new()
	}
	let input = &inp[1..&inp.len()-1];
	let mut id = String::new();
	let mut class = String::new();
	let mut current_type = "none";
	let mut everything_else = String::new();
	let mut out = String::new();
	for (_, cha) in input.char_indices() {
		match current_type {
			"none" => {
				match cha {
					'.' => current_type = "class",
					'#' => {
						id = String::new();
						current_type = "id";
					},
					_ => everything_else += &cha.to_string(),
				}
			},
			"class" => {
				match cha {
					' ' => {
						class += &cha.to_string();
						current_type = "none";
					},
					_ => class += &cha.to_string(),
				}
			},
			"id" => {
				match cha {
					' ' => {
						current_type = "none";
					},
					_ => id += &cha.to_string(),
				}
			},
			_ => panic!("Attribute parser reached undefined state"),
		}
	}
	if id != "" {
		out += &("id=\"".to_string() + &id + "\" ");
	}
	if class != "" {
		out += &("class=\"".to_string() + &class + "\" ");
	}
	out + &everything_else
}


pub fn parse(input: &str) -> String {
	let mut out = String::new();
	let mut list_type = "";
	let mut expect_block:String = String::new();
	for line in input.lines() {
		let tokens = tokenize(line);
		match tokens.first() {
			None => {
				if list_type == "" { out += "\n"; }
			},
			Some(first_token) => {
				match list_type {
					"ul" => {
						match first_token.class {
							TokenType::List => (),
							_ => {
								list_type = "";
								out += "</ul>\n";
								expect_block = String::new();
							}
						}
					},
					"ol" => {
						match first_token.class {
							TokenType::NumberedList => (),
							_ => {
								list_type = "";
								out += "</ol>\n";
								expect_block = String::new();
							}
						}
					},
					_ => (),
				}
				match first_token.class {
					TokenType::Header => out += &("<h".to_owned() + &first_token.content.len().to_string() + " " + &parse_attr(&first_token.attributes) + ">" + &parse_line(&tokens[1..].to_vec()) + "</h" + &first_token.content.len().to_string() + ">\n"),
					TokenType::Attr => {
						if tokens.len() != 1 {
							out += &("<p ".to_owned() + &parse_attr(&first_token.content) + ">" + &parse_line(&tokens[1..].to_vec()) + "</p>\n");
						} else {
							expect_block = parse_attr(&first_token.content);
						}
					},
					TokenType::List => {
						let mut attr = String::new();
						if expect_block != "" {
							attr = expect_block.clone();
							expect_block = String::new();
						}
						if list_type != "ul" { out += &("\n<ul ".to_owned() + &attr + ">\n"); }
						list_type = "ul";
						out += &("<li ".to_owned() + &parse_attr(&first_token.attributes) + ">" + &parse_line(&tokens[1..].to_vec()) + "</li>\n");
					},
					TokenType::NumberedList => {
						let mut attr = String::new();
						if expect_block != "" {
							attr = expect_block.clone();
							expect_block = String::new();
						}
						if list_type != "ol" { out += &("\n<ol ".to_owned() + &attr + ">\n"); }
						list_type = "ol";
						out += &("<li ".to_owned() + &parse_attr(&first_token.attributes) + ">" + &parse_line(&tokens[1..].to_vec()) + "</li>\n");
					},
					TokenType::Html => out += &first_token.content,
					_ => out += &("<p>".to_owned() + &parse_line(&tokens) + "</p>\n"),
				}
			}
		}
	}
	match list_type {
		"ul" => out += "</ul>\n",
		"ol" => out += "</ol>\n",
		_ => (),
	}
	out
}

fn parse_line(input: &Vec<Token>) -> String {
	let mut out = String::new();
	let mut iter: usize = 0;
	for i in input.iter() {
		match i.class {
			TokenType::Put => out += &i.content,
			TokenType::Bold => out += &("<b ".to_owned() + &parse_attr(&i.attributes) + ">" + &parse_line(&i.subtokens) + "</b>"),
			TokenType::Italic => out += &("<i ".to_owned() + &parse_attr(&i.attributes) + ">" + &parse_line(&i.subtokens) + "</i>"),
			TokenType::Emphasis => out += &("<em ".to_owned() + &parse_attr(&i.attributes) + ">" + &parse_line(&i.subtokens) + "</em>"),
			TokenType::Strong => out += &("<strong ".to_owned() + &parse_attr(&i.attributes) + ">" + &parse_line(&i.subtokens) + "</strong>"),
			TokenType::Sub => out += &("<sub ".to_owned() + &parse_attr(&i.attributes) + ">" + &parse_line(&i.subtokens) + "</sub>"),
			TokenType::Sup => out += &("<sup ".to_owned() + &parse_attr(&i.attributes) + ">" + &parse_line(&i.subtokens) + "</sup>"),
			TokenType::Span => out += &("<span ".to_owned() + &parse_attr(&i.attributes) + ">" + &parse_line(&i.subtokens) + "</span>"),
			TokenType::Code => out += &("<code ".to_owned() + &parse_attr(&i.attributes) + ">" + &parse_line(&i.subtokens) + "</code>"),
			TokenType::LinkName => {
				let parsed_name = parse_line(&i.subtokens);
				if iter < input.len() - 1 {
					let next = &input[iter + 1];
					match next.class {
						TokenType::LinkDir => out += &("<a href=\"".to_owned() + &next.content[1..next.content.len()-1] + "\" " + &parse_attr(&next.attributes) + ">" + &parsed_name + "</a>"),
						_ => out += &("<a href=\"".to_owned() + &parsed_name + "\" "  + &parse_attr(&i.attributes) + ">" + &parsed_name + "</a>"),
					}
				} else { out += &("<a href=\"".to_owned() + &parsed_name + "\" "  + &parse_attr(&i.attributes) + ">" + &parsed_name + "</a>"); }
			},
			TokenType::LinkDir => (),
			_ => out += &i.content,
		}
		iter += 1;
	}
	out
}
