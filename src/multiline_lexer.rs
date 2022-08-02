use crate::lexer::{TokenType, Token, push_token, tokenize};

fn add_table(tokens: &mut Vec<Token>, table: &mut Token) {
	if !table.subtokens.is_empty() {
		tokens.push(table.clone());
		*table = Token::init(TokenType::Table, String::new());
	}
}

fn table_parse(input: &String) -> Token {
	enum CellMode {
		None,
		Column,
		Row,
		Attr,
	}
	let mut nullify = false;
	let mut out: Vec<Token> = Vec::new();
	let mut starting_cell = true;
	let mut current_cell = Token::init(TokenType::TableCell, String::new());
	let mut cell_mode = CellMode::None;
	let mut current_cell_col = String::new();
	let mut current_cell_row = String::new();
	let mut rowattr = String::new();
	for ch in input.chars() {
		if starting_cell {
			// If it's writing the cell starter token
			match ch {
				'*' => {
					match cell_mode {
						CellMode::Attr => current_cell.attributes += &ch.to_string(),
						_ => current_cell.class = TokenType::TableHeader,
					}
				},
				'r' => {
					match cell_mode {
						CellMode::Attr => current_cell.attributes += &ch.to_string(),
						_ => cell_mode = CellMode::Row,
					}
				},
				'c' => {
					match cell_mode {
						CellMode::Attr => current_cell.attributes += &ch.to_string(),
						_ => cell_mode = CellMode::Column,
					}
				},
				'0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
					match cell_mode {
						CellMode::Column => current_cell_col += &ch.to_string(),
						CellMode::Row => current_cell_row += &ch.to_string(),
						CellMode::Attr => current_cell.attributes += &ch.to_string(),
						_ => panic!("Found a digit in an unexpected position in cell token"),
					}
				},
				'{' => {
					cell_mode = CellMode::Attr;
					current_cell.attributes += &ch.to_string();
				}
				'}'=> {
					match cell_mode {
						CellMode::Attr => {
							cell_mode = CellMode::None;
						},
						_ => panic!("{}", "Found a } outside an attribute sequence"),
					}
				},
				'|' => {
					nullify = false;
					starting_cell = true;
					current_cell = Token::init(TokenType::TableCell, String::new());
					cell_mode = CellMode::None;
					current_cell_row = String::new();
					current_cell_col = String::new();
				},
				'-' => nullify = true,
				' ' => {
					match cell_mode {
						CellMode::Attr => current_cell.attributes += &ch.to_string(),
						_ => {
							starting_cell = false;
							let mut close_atter = !current_cell.attributes.is_empty();
							if !current_cell_col.is_empty() || !current_cell_row.is_empty() {
								if !close_atter {
									current_cell.attributes += "{";
									close_atter = true;
								}
								if !current_cell_row.is_empty() {
									current_cell.attributes += &(" rowspan=\"".to_owned() + &current_cell_row + "\"");
								}
								if !current_cell_col.is_empty() {
									current_cell.attributes += &(" colspan=\"".to_owned() + &current_cell_col + "\"");
								}
							}
							if close_atter { current_cell.attributes += "}" }
						}
					}
				},
				_ => {
					match cell_mode {
						CellMode::Attr => current_cell.attributes += &ch.to_string(),
						_ => panic!("Unexpected character in table cell initiation"),
					}
				},
			}
		} else {
			// If it's writing the content of the cell
			match ch {
				'|' => {
					if !nullify {
						current_cell.subtokens = tokenize(&current_cell.content.trim_end_matches('\t')).0;
						out.push(current_cell.clone());
					}
					nullify = false;
					starting_cell = true;
					current_cell = Token::init(TokenType::TableCell, String::new());
					cell_mode = CellMode::None;
					current_cell_row = String::new();
					current_cell_col = String::new();
				},
				_ => current_cell.content += &ch.to_string(),
			}
		}
	}
	if !current_cell.attributes.is_empty() && current_cell.content.is_empty() {
		current_cell.attributes += "}";
		rowattr = current_cell.attributes;
	}
	let mut outok = Token::init_sub(TokenType::TableRow, out, String::new());
	outok.attributes = rowattr;
	outok
}

pub(crate) fn block_lexer(lines: &Vec<Vec<Token>>) -> Vec<Token>{
	let mut blocks: Vec<Token> = Vec::new();
	let mut current_block: Token;
	let mut lists: Vec<Token> = Vec::new();
	let mut table: Token = Token::init(TokenType::Table, String::new());
	let mut next_attr: String = String::new();
	for line in lines.iter() {
		match line.first() {
			None => add_table(&mut blocks, &mut table),
			Some(ftoken) => {
				match ftoken.class {
					TokenType::TableRow => {
						if table.subtokens.is_empty() {
							table.attributes = next_attr.clone();
							next_attr = String::new();
						}
						table.subtokens.push(table_parse(&ftoken.content));
					},
					TokenType::ListEl => {
						add_table(&mut blocks, &mut table);
						let fltoken = { 
							let mut ft = ftoken.clone();
							ft.subtokens = line[1..].to_vec();
							ft
						};
						match lists.last_mut() {
							None => {
								let mut new_sublist = Token::init_sub(TokenType::UList, vec![fltoken.to_owned()], fltoken.content.to_owned());
								new_sublist.attributes = next_attr.clone();	
								lists.push(new_sublist);
								next_attr = String::new();
							},
							Some(x) => {
								match x.class {
									TokenType::UList => {
										if get_list_depth(&x) == get_list_depth(&fltoken) {
											x.subtokens.push(fltoken.to_owned());
										} else if get_list_depth(&x) < get_list_depth(&fltoken) {
											let mut new_sublist = Token::init_sub(TokenType::UList, vec![fltoken.to_owned()], fltoken.content.to_owned());
											new_sublist.attributes = next_attr.clone();	
											lists.push(new_sublist);
											next_attr = String::new();
										}
										else {
											let new_sublist = Token::init_sub(TokenType::UList, vec![fltoken.to_owned()], fltoken.content.to_owned());
											lists.push(new_sublist);
										}
									},
									TokenType::OList => {
										lists.push(Token::init_sub(TokenType::UList, vec![fltoken.to_owned()], fltoken.content.to_owned()));
									},
									_ => panic!("This wasn't supposed to be possible at all"),
								}
							},
						}
					},
					TokenType::NumberedListEl => {
						add_table(&mut blocks, &mut table);
						let fltoken = { 
							let mut ft = ftoken.clone();
							ft.subtokens = line[1..].to_vec();
							ft
						};
						match lists.last_mut() {
							None => {
								let mut new_sublist = Token::init_sub(TokenType::OList, vec![fltoken.to_owned()], fltoken.content.to_owned());
								new_sublist.attributes = next_attr.clone();
								next_attr = String::new();
								lists.push(new_sublist);
							},
							Some(x) => {
								match x.class {
									TokenType::OList => {
										if get_list_depth(&x) == get_list_depth(&fltoken) {
											x.subtokens.push(fltoken.to_owned());
										} else if get_list_depth(&x) < get_list_depth(&fltoken) {
											let mut new_sublist = Token::init_sub(TokenType::OList, vec![fltoken.to_owned()], fltoken.content.to_owned());
											new_sublist.attributes = next_attr.clone();	
											lists.push(new_sublist);
											next_attr = String::new();
										}
										else {
											let new_sublist = Token::init_sub(TokenType::OList, vec![fltoken.to_owned()], fltoken.content.to_owned());
											lists.push(new_sublist);
										}
									},
									TokenType::UList => {
										lists.push(Token::init_sub(TokenType::OList, vec![fltoken.to_owned()], fltoken.content.to_owned()));
									},
									_ => panic!("This wasn't supposed to be possible at all"),
								}
							},
						}
					},
					TokenType::Html => {
						add_table(&mut blocks, &mut table);
						if !lists.is_empty() {
							push_token(&mut blocks, &Token::init_sub(TokenType::ListBlock, lists.clone(), String::new()));
						}
						current_block = ftoken.clone();
						current_block.subtokens = line[1..].to_vec();
						push_token(&mut blocks, &current_block);
					}
					TokenType::Attr => {
						if !lists.is_empty() {
							push_token(&mut blocks, &Token::init_sub(TokenType::ListBlock, lists.clone(), String::new()));
							lists = Vec::new();
						}
						if line.len() > 1 {
							current_block = Token::n_para();
							current_block.attributes = line[0].content.to_owned();
							current_block.subtokens = line[1..].to_vec();
							push_token(&mut blocks, &current_block);
						} else {
							next_attr = ftoken.content.to_owned();
						}
					},
					TokenType::Header => {
						add_table(&mut blocks, &mut table);
						if !lists.is_empty() {
							push_token(&mut blocks, &Token::init_sub(TokenType::ListBlock, lists.clone(), String::new()));
							lists = Vec::new();
						}
						current_block = ftoken.clone();
						current_block.subtokens = line[1..].to_vec();
						push_token(&mut blocks, &current_block);
					},
					TokenType::Image => {
						add_table(&mut blocks, &mut table);
						if !lists.is_empty() {
							push_token(&mut blocks, &Token::init_sub(TokenType::ListBlock, lists.clone(), String::new()));
							lists = Vec::new();
						}
						if line.len() > 1 {
							current_block = Token::n_para();
							current_block.subtokens = line.clone();
							push_token(&mut blocks, &current_block);
						} else {
							current_block = ftoken.clone();
							push_token(&mut blocks, &current_block);
						}
					}
					_ => {
						add_table(&mut blocks, &mut table);
						if !lists.is_empty() {
							push_token(&mut blocks, &Token::init_sub(TokenType::ListBlock, lists.clone(), String::new()));
							lists = Vec::new();
						}
						current_block = Token::n_para();
						current_block.subtokens = line.clone();
						match blocks.last_mut() {
							None => push_token(&mut blocks, &current_block),
							Some(x) => {
								let first_char = current_block.subtokens.first_mut().expect("An empty string got to the paragraph parser.");
								if &first_char.content[0..1] == " " {
									x.subtokens.push(Token::init(TokenType::LineBreak, "\n".to_owned()));
									first_char.content = "\n".to_owned() + &first_char.content[1..];
									x.subtokens.append(&mut current_block.subtokens);
								} else {
									push_token(&mut blocks, &current_block);
								}
							},
						}
					}
				}
			}
		}
	}
	add_table(&mut blocks, &mut table);
	if !lists.is_empty() {
		push_token(&mut blocks, &Token::init_sub(TokenType::ListBlock, lists.clone(), String::new()));
	}
	blocks
}

pub(crate) fn get_list_depth(token: &Token) -> usize {
	match token.class {
		TokenType::UList | TokenType::ListEl | TokenType::OList | TokenType::NumberedListEl => token.content.len() - 1,
		_ => panic!("Passed a non-list token to get_list_depth"),
	}
}
