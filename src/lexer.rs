use std::collections::HashMap;

#[derive(Clone, Debug)]
pub(crate) enum TokenType {
	Put,
	Bold,
	Strong,
	Italic,
	Emphasis,
	LinkName,
	LinkDir,
	Attr,
	Sub,
	Sup,
	Span,
	Code,
	Strike,
	Under,
	Header,
}

#[derive(Clone, Debug)]
pub(crate) struct Token {
	pub(crate) class: TokenType,
	pub(crate) content: String,
	pub(crate) subtokens: Vec<Token>,
	pub(crate) attributes: String,
}

impl Token {
	fn new() -> Self {
		Self { class: TokenType::Put, content: String::new(), subtokens: Vec::new(), attributes: String::new() }
	}
	fn init(class: TokenType, content: String) -> Self {
		Self { class: class, content: content, subtokens: Vec::new(), attributes: String::new() }
	}
	fn init_sub(class: TokenType, content: Vec<Self>) -> Self {
		Self { class: class, content: String::new(), subtokens: content, attributes: String::new() }
	}
	fn tokenize_content(self: &mut Self, borders: usize) {
		self.subtokens = tokenize(&self.content[borders..self.content.len()-borders]);
	}
}

pub(crate) fn tokenize(input: &str) -> Vec<Token> {
	let mut tokens:Vec<Token> = vec![];
	let mut current_token: Token = Token::new();
	let mut escaping = false;

	let mut strong_wait = false; // Variable used for closing a STRONG token
	for (pos, cha) in input.char_indices() {
		if cha == '\\'{
			if escaping {
				escaping = false;
				current_token.content += &cha.to_string();
			} else { escaping = true; }
		} else {
			match current_token.class {
				TokenType::Put => {
					if !escaping {
						match cha {
							'*' => {
								push_token(&mut tokens, &current_token);
								current_token = Token::init(TokenType::Bold, cha.to_string());
							},
							'_' => {
								push_token(&mut tokens, &current_token);
								current_token = Token::init(TokenType::Italic, cha.to_string());
							},
							'[' => {
								push_token(&mut tokens, &current_token);
								current_token = Token::init(TokenType::LinkName, cha.to_string());
							},
							'~' => {
								push_token(&mut tokens, &current_token);
								current_token = Token::init(TokenType::Sub, cha.to_string());
							},
							'^' => {
								push_token(&mut tokens, &current_token);
								current_token = Token::init(TokenType::Sup, cha.to_string());
							},
							'`' => {
								push_token(&mut tokens, &current_token);
								current_token = Token::init(TokenType::Code, cha.to_string());
							},
							'@' => {
								push_token(&mut tokens, &current_token);
								current_token = Token::init(TokenType::Span, cha.to_string());
							},
							'-' => {
								push_token(&mut tokens, &current_token);
								current_token = Token::init(TokenType::Strike, cha.to_string());
							},
							'#' => {
								if pos == 0 { current_token = Token::init(TokenType::Header, cha.to_string()); }
								else { current_token.content += &cha.to_string(); }
							},
							'(' => {
								match tokens.last() {
									None => current_token.content += &cha.to_string(),
									Some(last_token) => {
										match last_token.class {
											TokenType::LinkName => current_token = Token::init(TokenType::LinkDir, cha.to_string()),
											_ => current_token.content += &cha.to_string(),
										}
									}
								}
							},
							'{' => {
								push_token(&mut tokens, &current_token);
								match tokens.last() {
									None => current_token.content += &cha.to_string(),
									Some(last_token) => {
										match last_token.class {
											TokenType::Put => {
												tokens.pop();
												current_token.content += &cha.to_string();
											},
											_ => current_token = Token::init(TokenType::Attr, cha.to_string()),
										}
									}
								}
							},
							_ => current_token.content += &cha.to_string(),
						}
					} else { current_token.content += &cha.to_string(); }
				},
				TokenType::Bold => {
					current_token.content += &cha.to_string();
					match cha {
						'*' => {
							if current_token.content == "**" && !escaping { current_token.class = TokenType::Strong; }
							else if !escaping {
								current_token.tokenize_content(1);
								push_token(&mut tokens, &current_token);
								current_token = Token::new();
							}
						},
						' ' => if current_token.content == "* " && !escaping { current_token.class = TokenType::Put },
						_ => (),
					}
				},
				TokenType::Strong => {
					current_token.content += &cha.to_string();
					match cha {
						'*' => {
							if !escaping && !strong_wait { strong_wait = true; }
							else if !escaping && strong_wait {
								current_token.tokenize_content(2);
								push_token(&mut tokens, &current_token);
								current_token = Token::new();
								strong_wait = false;
							} else { strong_wait = false; }
						},
						' ' => if current_token.content == "** " && !escaping { current_token.class = TokenType::Put },
						_ => (),
					}
				},
				TokenType::Italic => {
					current_token.content += &cha.to_string();
					match cha {
						'_' => {
							if current_token.content == "__" && !escaping { current_token.class = TokenType::Emphasis; }
							else if !escaping {
								current_token.tokenize_content(1);
								push_token(&mut tokens, &current_token);
								current_token = Token::new();
							}
						},
						' ' => if current_token.content == "_ " && !escaping { current_token.class = TokenType::Put },
						_ => (),
					}
				},
				TokenType::Emphasis => {
					current_token.content += &cha.to_string();
					match cha {
						'_' => {
							if !escaping && !strong_wait { strong_wait = true; }
							else if !escaping && strong_wait {
								current_token.tokenize_content(2);
								push_token(&mut tokens, &current_token);
								current_token = Token::new();
								strong_wait = false;
							} else { strong_wait = false; }
						},
						' ' => if current_token.content == "__ " && !escaping { current_token.class = TokenType::Put },
						_ => (),
					}
				},
				TokenType::Sub => {
					current_token.content += &cha.to_string();
					match cha {
						'~' => {
							if !escaping {
								current_token.tokenize_content(1);
								push_token(&mut tokens, &current_token);
								current_token = Token::new();
							} 
						},
						' ' => if current_token.content == "~ " && !escaping { current_token.class = TokenType::Put },
						_ => (),
					}
				},
				TokenType::Sup => {
					current_token.content += &cha.to_string();
					match cha {
						'^' => {
							if !escaping {
								current_token.tokenize_content(1);
								push_token(&mut tokens, &current_token);
								current_token = Token::new();
							} 
						},
						' ' => if current_token.content == "^ " && !escaping { current_token.class = TokenType::Put },
						_ => (),
					}
				},
				TokenType::Span => {
					current_token.content += &cha.to_string();
					match cha {
						'@' => {
							if !escaping {
								current_token.tokenize_content(1);
								push_token(&mut tokens, &current_token);
								current_token = Token::new();
							} 
						},
						' ' => if current_token.content == "@ " && !escaping { current_token.class = TokenType::Put },
						_ => (),
					}
				},
				TokenType::Code => {
					current_token.content += &cha.to_string();
					match cha {
						'`' => {
							if !escaping {
								current_token.tokenize_content(1);
								push_token(&mut tokens, &current_token);
								current_token = Token::new();
							} 
						},
						_ => (),
					}
				},
				TokenType::Strike => {
					current_token.content += &cha.to_string();
					match cha {
						'-' => {
							if current_token.content == "--" && !escaping { current_token.class = TokenType::Under; }
							else if !escaping {
								current_token.tokenize_content(1);
								push_token(&mut tokens, &current_token);
								current_token = Token::new();
							}
						},
						' ' => if current_token.content == "- " && !escaping { current_token.class = TokenType::Put },
						_ => (),
					}
				},
				TokenType::Under => {
					current_token.content += &cha.to_string();
					match cha {
						'-' => {
							if !escaping && !strong_wait { strong_wait = true; }
							else if !escaping && strong_wait {
								current_token.tokenize_content(2);
								push_token(&mut tokens, &current_token);
								current_token = Token::new();
								strong_wait = false;
							} else { strong_wait = false; }
						},
						' ' => if current_token.content == "-- " && !escaping { current_token.class = TokenType::Put },
						_ => (),
					}
				},
				TokenType::LinkName => {
					current_token.content += &cha.to_string();
					match cha {
						']' => {
							if !escaping {
								current_token.tokenize_content(1);
								push_token(&mut tokens, &current_token);
								current_token = Token::new();
							}
						},
						_ => (),
					}
				},
				TokenType::LinkDir => {
					current_token.content += &cha.to_string();
					match cha {
						')' => {
							if !escaping {
								push_token(&mut tokens, &current_token);
								current_token = Token::new();
							}
						},
						_ => (),
					}
				},
				TokenType::Attr => {
					current_token.content += &cha.to_string();
					match cha {
						'}' => {
							if !escaping {
								match tokens.last_mut() {
									None => {
										push_token(&mut tokens, &current_token);
									},
									Some(last_token) =>	{
										last_token.attributes = current_token.content.clone();
									},
								}
								current_token = Token::new();
							}
						},
						_ => (),
					}
				},
				TokenType::Header => {
					match cha {
						'#' => (),
						_ => {
							push_token(&mut tokens, &current_token);
							current_token = Token::new();
						}
					}
					current_token.content += &cha.to_string()
				}
				_ => panic!("Reached undefined token type {:?}", current_token.class),
			}
		}
		if escaping && cha != '\\' { escaping = false; }
	}
	if !current_token.content.is_empty() {
		current_token.class = TokenType::Put;
		push_token(&mut tokens, &current_token);
	}
	tokens
}

fn push_token(list: &mut Vec<Token>, token: &Token) {
	if token.content != "" { list.push(token.clone()); }
}

fn parse_attr(input: &str) -> String {
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
						id += &cha.to_string();
						current_type = "none";
					},
					_ => id += &cha.to_string(),
				}
			},
			_ => panic!("Attribute parser reached undefined state"),
		}
	}
	if id != "" {
		out += &("id=".to_string() + &id + "\"");
	}
	if class != "" {
		out += &("class=".to_string() + &id + "\"");
	}
	out + &everything_else
}


pub(crate) fn parse_multiline(input: &str) -> String {
	let mut out = String::new();
	for line in input.lines() {
		let tokens = tokenize(line);
		match tokens.first() {
			None => out += "\n",
			Some(first_token) => {
				match first_token.class {
					TokenType::Header => out += &("<h".to_owned() + &first_token.content.len().to_string() + ">" + &parse_line(&tokens[1..].to_vec()) + "</h" + &first_token.content.len().to_string() + ">\n"),
					_ => out += &("<p>".to_owned() + &parse_line(&tokens) + "</p>\n"),
				}
			}
		}
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
						TokenType::LinkDir => out += &("<a href=\"".to_owned() + &next.content[1..next.content.len()-1] + "\" " + &parse_attr(&i.attributes) + ">" + &parsed_name + "</a>"),
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
