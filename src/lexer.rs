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
	Html,
	List,
	NumberedList,
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

	let mut nlist_wait_space = false;

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
							'<' => {
								push_token(&mut tokens, &current_token);
								current_token = Token::init(TokenType::Html, cha.to_string());
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
									None => current_token = Token::init(TokenType::Attr, cha.to_string()),
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
						' ' => {
							if current_token.content == "* " && !escaping {
								if pos != 1 { current_token.class = TokenType::Put;	}
								else {
									current_token.class = TokenType::List;
									push_token(&mut tokens, &current_token);
									current_token = Token::new();
								}
							}
						},
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
				TokenType::Html => {
					current_token.content += &cha.to_string();
					match cha {
						'>' => {
							if !escaping {
								current_token.tokenize_content(1);
								push_token(&mut tokens, &current_token);
								current_token = Token::new();
							}
						},
						' ' => if current_token.content == "< " && !escaping { current_token.class = TokenType::Put },
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
						'#' => {
							nlist_wait_space = false;
							current_token.content += &cha.to_string();
						},
						'{' => {
							nlist_wait_space = false;
							push_token(&mut tokens, &current_token);
							current_token = Token::init(TokenType::Attr, cha.to_string());
						},
						' ' => {
							if !nlist_wait_space {
								push_token(&mut tokens, &current_token);
								current_token = Token::new();
							} else {
								current_token.class = TokenType::NumberedList;
								push_token(&mut tokens, &current_token);
								current_token = Token::new();
							}
							nlist_wait_space = false;
						},
						'.' => {
							if pos == 1 { nlist_wait_space = true; }
							else { 
								push_token(&mut tokens, &current_token);
								current_token = Token::new();
								current_token.content += &cha.to_string();
								nlist_wait_space = false;
							}
						}
						_ => {
							nlist_wait_space = false;
							push_token(&mut tokens, &current_token);
							current_token = Token::new();
							current_token.content += &cha.to_string();
						}
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


