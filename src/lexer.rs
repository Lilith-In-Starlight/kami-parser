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
	IDAttr,
	ClassAttr,
	NormAttr,
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
	for (_, cha) in input.char_indices() {
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
									None => panic!("Attribute found as first token, which is not meant to happen"),
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

fn parse_attr(input: &str) -> HashMap<&str, String> {
	let mut out: HashMap<&str, String> = HashMap::new();
	let mut current_content = String::new();
	let mut current_type = "none";
	let mut everything_else = String::new();
	for (_, cha) in input.char_indices() {
		match current_type {
			"none" => {
				match cha {
					'.' => current_type = "class",
					'#' => current_type = "id",
					_ => everything_else += &cha.to_string(),
				}
			},
			"class" => {
				match cha {
					' ' => {
						let mut classc = current_content.clone();
						match out.get("class") {
							Some(x) => {
								classc = current_content + &x.clone();
							},
							None => (),
						}
						out.insert("class", classc);
						current_content = String::new();
						current_type = "none";
					},
					_ => current_content += &cha.to_string(),
				}
			},
			"id" => {
				match cha {
					' ' => {
						out.insert("id", current_content);
						current_content = String::new();
						current_type = "none";
					},
					_ => current_content += &cha.to_string(),
				}
			},
			_ => panic!("Attribute parser reached undefined state"),
		}
	}
	out.insert("else", everything_else);
	out
}
