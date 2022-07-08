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
	ListEl,
	NumberedListEl,
	LineBreak,
	Para,
	UList,
	OList,
	ListBlock, 
	Image,
}

#[derive(Clone, Debug)]
pub(crate) struct Token {
	pub(crate) class: TokenType,
	pub(crate) content: String,
	pub(crate) subtokens: Vec<Token>,
	pub(crate) attributes: String,
}

impl Token {
	pub(crate) fn new() -> Self {
		Self { class: TokenType::Put, content: String::new(), subtokens: Vec::new(), attributes: String::new() }
	}
	pub(crate) fn n_para() -> Self {
		Self { class: TokenType::Para, content: String::new(), subtokens: Vec::new(), attributes: String::new() }
	}
	pub(crate) fn init(class: TokenType, content: String) -> Self {
		Self { class: class, content: content, subtokens: Vec::new(), attributes: String::new() }
	}
	pub(crate) fn init_sub(class: TokenType, tcontent: Vec<Self>, content: String) -> Self {
		Self { class: class, content: content, subtokens: tcontent, attributes: String::new() }
	}
	pub(crate) fn tokenize_content(self: &mut Self, borders: usize) {
		self.subtokens = tokenize(&self.content[borders..self.content.len()-borders]).0;
	}
	pub(crate) fn tokenize_unclosed(self: &mut Self, borders: usize) {
		self.subtokens = tokenize(&self.content[borders..self.content.len()]).0;
	}
}

pub(crate) fn tokenize(input: &str) -> (Vec<Token>, String) {
	let mut tokens:Vec<Token> = vec![];
	let mut current_token: Token = Token::new();
	let mut escaping = false;

	let mut warnings = String::new();

	let mut nlist_wait_space = false;

	let mut strong_wait = false; // Variable used for closing a STRONG token
	for (pos, cha) in input.char_indices() {
		if cha == '\\'{
			if escaping {
				escaping = false;
				current_token.content += &cha.to_string();
			} else {
				escaping = true;
				match current_token.class {
					TokenType::Bold | TokenType::Italic | TokenType::Strong | TokenType::Emphasis | TokenType::LinkName | TokenType::Sub | TokenType::Sup | TokenType::Code | TokenType::Span | TokenType::Under | TokenType::Strike => current_token.content +=  &cha.to_string(),
					_ => (),
				}
			}
		} else {
			match current_token.class {
				TokenType::Put => {
					if !escaping {
						match cha {
							'*' => {
								if !escaping {
									push_token(&mut tokens, &current_token);
									current_token = Token::init(TokenType::Bold, cha.to_string());
								} else { current_token.content += &cha.to_string(); }
							},
							'_' => {
								if !escaping {
									push_token(&mut tokens, &current_token);
									current_token = Token::init(TokenType::Italic, cha.to_string());
								} else { current_token.content += &cha.to_string(); }
							},
							'[' => {
								if !escaping {
									push_token(&mut tokens, &current_token);
									current_token = Token::init(TokenType::LinkName, cha.to_string());
								} else { current_token.content += &cha.to_string(); }
							},
							'~' => {
								if !escaping {
									push_token(&mut tokens, &current_token);
									current_token = Token::init(TokenType::Sub, cha.to_string());
								} else { current_token.content += &cha.to_string(); }
							},
							'^' => {
								if !escaping {
									push_token(&mut tokens, &current_token);
									current_token = Token::init(TokenType::Sup, cha.to_string());
								} else { current_token.content += &cha.to_string(); }
							},
							'!' => {
								if !escaping {
									push_token(&mut tokens, &current_token);
									current_token = Token::init(TokenType::Image, cha.to_string());
								} else { current_token.content += &cha.to_string(); }
							},
							'`' => {
								if !escaping {
									push_token(&mut tokens, &current_token);
									current_token = Token::init(TokenType::Code, cha.to_string());
								} else { current_token.content += &cha.to_string(); }
							},
							'@' => {
								if !escaping {
									push_token(&mut tokens, &current_token);
									current_token = Token::init(TokenType::Span, cha.to_string());
								} else { current_token.content += &cha.to_string(); }
							},
							'-' => {
								if !escaping {
									push_token(&mut tokens, &current_token);
									current_token = Token::init(TokenType::Under, cha.to_string());
								} else { current_token.content += &cha.to_string(); }
							},
							'#' => {
								if pos == 0 { current_token = Token::init(TokenType::Header, cha.to_string()); }
								else { current_token.content += &cha.to_string(); }
							},
							'<' => {
								if !escaping {
									push_token(&mut tokens, &current_token);
									current_token = Token::init(TokenType::Html, cha.to_string());
								} else { current_token.content += &cha.to_string(); }
							},
							'(' => {
								if !escaping {
									match tokens.last() {
										None => current_token.content += &cha.to_string(),
										Some(last_token) => {
											match last_token.class {
												TokenType::LinkName => current_token = Token::init(TokenType::LinkDir, cha.to_string()),
												_ => current_token.content += &cha.to_string(),
											}
										}
									}
								} else { current_token.content += &cha.to_string(); }
							},
							'{' => {
								if !escaping {
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
								} else { current_token.content += &cha.to_string(); }
							},
							'n' => {
								if escaping {
									push_token(&mut tokens, &current_token);
									current_token = Token::init(TokenType::LineBreak, String::from("BR"));
									push_token(&mut tokens, &current_token);
									current_token = Token::new();
								} else {
									current_token.content += &cha.to_string();
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
									current_token.class = TokenType::ListEl;
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
							if !(pos == 2 && current_token.content == "***") {
								if !escaping && !strong_wait { strong_wait = true; }
								else if !escaping && strong_wait {
									current_token.tokenize_content(2);
									push_token(&mut tokens, &current_token);
									current_token = Token::new();
									strong_wait = false;
								} else { strong_wait = false; }
							} else {
								current_token.class = TokenType::ListEl;
							}
						},
						' ' => {
							if current_token.content == "** " && !escaping {
								if pos != 2 { current_token.class = TokenType::Put; }
								else {
									current_token.class = TokenType::ListEl;
									push_token(&mut tokens, &current_token);
									current_token = Token::new();
								}
							}
						},
						_ => (),
					}
				},
				TokenType::ListEl => {
					current_token.content += &cha.to_string();
					match cha {
						'*' => (),
						' ' => {
							push_token(&mut tokens, &current_token);
							current_token = Token::new();
						},
						_ => current_token.class = TokenType::Put,
					}
				}
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
								if current_token.content == "~~" {
									current_token.class = TokenType::Strike;
								} else {
									current_token.tokenize_content(1);
									push_token(&mut tokens, &current_token);
									current_token = Token::new();
								}
							} 
						},
						' ' => if current_token.content == "~ " && !escaping { current_token.class = TokenType::Put },
						_ => (),
					}
				},
				TokenType::Image => {
					current_token.content += &cha.to_string();
					match cha {
						'!' => {
							if !escaping {
								if current_token.content == "!!" {
									current_token.class = TokenType::Put;
								} else {
									push_token(&mut tokens, &current_token);
									current_token = Token::new();
								}
							}
						},
						' ' => {
							if !escaping {
								if current_token.content == "! " {
									current_token.class = TokenType::Put;
								}
							}
						},
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
						'~' => {
							if !escaping && !strong_wait { strong_wait = true; }
							else if !escaping && strong_wait {
								current_token.tokenize_content(2);
								push_token(&mut tokens, &current_token);
								current_token = Token::new();
								strong_wait = false;
							} else { strong_wait = false; }
						},
						' ' => if current_token.content == "~~ " && !escaping { current_token.class = TokenType::Put },
						_ => (),
					}
				},
				TokenType::Under => {
					if current_token.content == "-" {
						current_token.content += &cha.to_string();
						match cha {
							'-' => (),
							_ => current_token.class = TokenType::Put,
						}
					}
					else {
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
								current_token.content += " ";
								current_token.class = TokenType::NumberedListEl;
								push_token(&mut tokens, &current_token);
								current_token = Token::new();
							}
							nlist_wait_space = false;
						},
						'.' => {
							if !nlist_wait_space { nlist_wait_space = true; }
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
		match current_token.class {
			TokenType::Put => (),
			_ => warnings += &format!("WARNING: Unclosed {:?} token at {}\n", current_token.class, current_token.content),
		}
		match current_token.class {
			TokenType::Bold | TokenType::Italic | TokenType::Sub | TokenType::Sup | TokenType::LinkName | TokenType::LinkDir | TokenType::Attr | TokenType::Image | TokenType::Html | TokenType::Code | TokenType::Span => {
				push_token(&mut tokens, &Token::init(TokenType::Put, current_token.content[0..1].to_string()));
				current_token.tokenize_unclosed(1);
				tokens.append(&mut current_token.subtokens);
			},
			TokenType::Strong | TokenType::Emphasis | TokenType::Under | TokenType::Strike => {
				push_token(&mut tokens, &Token::init(TokenType::Put, current_token.content[0..2].to_string()));
				current_token.tokenize_unclosed(2);
				tokens.append(&mut current_token.subtokens);
			}
			TokenType::Put => push_token(&mut tokens, &current_token),
			_ => { 
				push_token(&mut tokens, &current_token);
				warnings += "The unclosing of the last token was impossible to handle for Kami, so the raw text has been outputted. Please contact the project maintainer about this.\n";
			}
		}
	}
	(tokens, warnings)
}

pub(crate) fn push_token(list: &mut Vec<Token>, token: &Token) {
	if token.content != "" || !token.subtokens.is_empty() { list.push(token.clone()); }
}


