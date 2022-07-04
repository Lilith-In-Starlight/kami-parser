use crate::lexer::tokenize;
use crate::multiline_lexer::block_lexer;
use crate::lexer::Token;
use crate::lexer::TokenType;
use crate::multiline_lexer::get_list_depth;
use htmlentity::entity::*;

pub(crate) fn parse_attr(inp: &str) -> String {
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
	let mut tokvec:Vec<Vec<Token>> = Vec::new();
	let mut out = String::new();
	for i in input.lines() {
		tokvec.push(tokenize(i));
	}
	let blocks = block_lexer(&tokvec);
	for block in blocks {
		match block.class {
			TokenType::Para => out += &("<p ".to_owned() + &parse_attr(&block.attributes) + ">" + &parse_line(&block.subtokens) + "</p>\n"),
			TokenType::Header => out += &("<h".to_owned() + &block.content.len().to_string() + " " + &parse_attr(&block.attributes) + ">" + &parse_line(&block.subtokens) + "</h" + &block.content.len().to_string() + ">\n"),
			TokenType::ListBlock => {
				let mut list_types: Vec<&str> = Vec::new();
				let mut last_level = 0;
				for i in block.subtokens.iter() {
					match i.class {
						TokenType::UList => {
							match list_types.last(){
								None => {
									out += &("<ul ".to_owned() + &parse_attr(&i.attributes) + ">\n");
									last_level = 1;
									list_types.push("ul");
									out += &parse_line(&i.subtokens);
								},
								Some(x) => {
									let last_depth = get_list_depth(&i);
									if x == &"ul" {
										if last_depth > last_level {
											out += &("<ul ".to_owned() + &parse_attr(&i.attributes) + ">\n");
											list_types.push("ul");
										} else if last_depth < last_level {
											for _ in last_depth..last_level {
												out += &("</".to_string() + list_types.pop().unwrap() + ">\n"); // This should close the latest opened list type
											}
										}
										last_level = last_depth;
										out += &parse_line(&i.subtokens);
									} else {
										if last_depth < last_level {
											for _ in last_depth..last_level {
												out += &("</".to_string() + list_types.pop().unwrap() + ">\n"); // This should close the latest opened list type
											}
											if let Some(x) = list_types.last() {
												if x == &"ol" {
													out += &("</".to_string() + list_types.pop().unwrap() + ">\n"); // This should close the latest opened list type
													last_level -= 1;
												}
											}
										}
										out += &("<ul ".to_owned() + &parse_attr(&i.attributes) + ">\n");
										last_level += 1;
										list_types.push("ul");
										out += &parse_line(&i.subtokens);
									}
								}
							}
						},
						TokenType::OList => {
							match list_types.last(){
								None => {
									out += &("<ol ".to_owned() + &parse_attr(&i.attributes) + ">\n");
									last_level = 1;
									list_types.push("ol");
									out += &parse_line(&i.subtokens);
								},
								Some(x) => {
									let last_depth = get_list_depth(&i);
									if x == &"ol" {
										if last_depth > last_level {
											out += &("<ol ".to_owned() + &parse_attr(&i.attributes) + ">\n");
											list_types.push("ol");
										} else if last_depth < last_level {
											for _ in last_depth..last_level {
												out += &("</".to_string() + list_types.pop().unwrap() + ">\n"); // This should close the latest opened list type
											}
										}
										last_level = last_depth;
										out += &parse_line(&i.subtokens);
									} else {
										if last_depth <= last_level {
											for _ in last_depth..last_level {
												out += &("</".to_string() + list_types.pop().unwrap() + ">\n"); // This should close the latest opened list type
											}
											if let Some(x) = list_types.last() {
												if x == &"ul" {
													out += &("</".to_string() + list_types.pop().unwrap() + ">\n"); // This should close the latest opened list type
													last_level -= 1;
												}
											}
										}
										out += &("<ol ".to_owned() + &parse_attr(&i.attributes) + ">\n");
										last_level += 1;
										list_types.push("ol");
										out += &parse_line(&i.subtokens);
									}
								}
							}
						}
						_ => ()
					}
				}
				for _ in 0..last_level {
					match list_types.pop() {
						None => (),
						Some(x) => out += &("</".to_string() + x + ">\n"),
					}// This should close the latest opened list type
				}
			}
			_ => (),
		}
	}
	out
}


fn parse_line(input: &Vec<Token>) -> String {
	let mut out = String::new();
	let mut iter: usize = 0;
	for i in input.iter() {
		match i.class {
			TokenType::Put => out += &encode(&i.content, EntitySet::SpecialCharsAndNoASCII, EncodeType::NamedOrHex).iter().collect::<String>(),
			TokenType::Bold => out += &("<b ".to_owned() + &parse_attr(&i.attributes) + ">" + &parse_line(&i.subtokens) + "</b>"),
			TokenType::Italic => out += &("<i ".to_owned() + &parse_attr(&i.attributes) + ">" + &parse_line(&i.subtokens) + "</i>"),
			TokenType::Emphasis => out += &("<em ".to_owned() + &parse_attr(&i.attributes) + ">" + &parse_line(&i.subtokens) + "</em>"),
			TokenType::Strong => out += &("<strong ".to_owned() + &parse_attr(&i.attributes) + ">" + &parse_line(&i.subtokens) + "</strong>"),
			TokenType::Sub => out += &("<sub ".to_owned() + &parse_attr(&i.attributes) + ">" + &parse_line(&i.subtokens) + "</sub>"),
			TokenType::Sup => out += &("<sup ".to_owned() + &parse_attr(&i.attributes) + ">" + &parse_line(&i.subtokens) + "</sup>"),
			TokenType::Span => out += &("<span ".to_owned() + &parse_attr(&i.attributes) + ">" + &parse_line(&i.subtokens) + "</span>"),
			TokenType::Code => out += &("<code ".to_owned() + &parse_attr(&i.attributes) + ">" + &encode(&i.content[1..i.content.len()-1], EntitySet::SpecialCharsAndNoASCII, EncodeType::NamedOrHex).iter().collect::<String>() + "</code>"),
			TokenType::LineBreak => out += "<br>",
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
			TokenType::ListEl | TokenType::NumberedListEl => out += &("<li ".to_owned() + &parse_attr(&i.attributes) + ">" + &parse_line(&i.subtokens) + "</li>\n"),
			TokenType::LinkDir => (),
			_ => out += &i.content,
		}
		iter += 1;
	}
	out
}
