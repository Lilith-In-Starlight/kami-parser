#[derive(Clone, Debug)]
pub(crate) enum TokenType {
	Put,
	Bold,
	Strong,
	Italic,
	Emphasis,
}

#[derive(Clone, Debug)]
pub(crate) struct Token {
	pub(crate) class: TokenType,
	pub(crate) content: String,
}

impl Token {
	fn new() -> Self {
		Self { class: TokenType::Put, content: String::new() }
	}
	fn init(class: TokenType, content: String) -> Self {
		Self { class: class, content: content }
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
					match cha {
						'*' => {
							push_token(&mut tokens, &current_token);
							current_token = Token::init(TokenType::Bold, cha.to_string());
						},
						'_' => {
							push_token(&mut tokens, &current_token);
							current_token = Token::init(TokenType::Italic, cha.to_string());
						},
						_ => current_token.content += &cha.to_string(),
					}
				},
				TokenType::Bold => {
					current_token.content += &cha.to_string();
					match cha {
						'*' => {
							if current_token.content == "**" && !escaping { current_token.class = TokenType::Strong; }
							else if !escaping {
								current_token.class = TokenType::Put;
								push_token(&mut tokens, &Token::init(TokenType::Bold, String::from("*")));
								push_token(&mut tokens, &Token::init(TokenType::Put, current_token.content[1..current_token.content.len()-1].to_string()));
								push_token(&mut tokens, &Token::init(TokenType::Bold, String::from("*")));
								current_token = Token::init(TokenType::Put, String::new());
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
								push_token(&mut tokens, &Token::init(TokenType::Strong, String::from("**")));
								push_token(&mut tokens, &Token::init(TokenType::Put, current_token.content[2..current_token.content.len()-2].to_string()));
								push_token(&mut tokens, &Token::init(TokenType::Strong, String::from("**")));
								current_token = Token::init(TokenType::Put, String::new());
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
								current_token.class = TokenType::Put;
								push_token(&mut tokens, &Token::init(TokenType::Italic, String::from("_")));
								push_token(&mut tokens, &Token::init(TokenType::Put, current_token.content[1..current_token.content.len()-1].to_string()));
								push_token(&mut tokens, &Token::init(TokenType::Italic, String::from("_")));
								current_token = Token::init(TokenType::Put, String::new());
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
								push_token(&mut tokens, &Token::init(TokenType::Emphasis, String::from("__")));
								push_token(&mut tokens, &Token::init(TokenType::Put, current_token.content[2..current_token.content.len()-2].to_string()));
								push_token(&mut tokens, &Token::init(TokenType::Emphasis, String::from("__")));
								current_token = Token::init(TokenType::Put, String::new());
								strong_wait = false;
							} else { strong_wait = false; }
						},
						' ' => if current_token.content == "__ " && !escaping { current_token.class = TokenType::Put },
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
	list.push(token.clone());
}
