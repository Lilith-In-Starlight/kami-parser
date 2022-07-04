use crate::lexer::{TokenType, Token, push_token};
use crate::syntax::parse_attr;

pub(crate) fn block_lexer(lines: &Vec<Vec<Token>>) -> Vec<Token>{
	let mut blocks: Vec<Token> = Vec::new();
	let mut current_block = Token::n_para();
	let mut lists: Vec<Token> = Vec::new();
	let mut next_attr: String = String::new();
	for line in lines.iter() {
		match line.first() {
			None => (),
			Some(ftoken) => {
				match ftoken.class {
					TokenType::ListEl => {
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
					TokenType::Attr => {
						if !lists.is_empty() {
							push_token(&mut blocks, &Token::init_sub(TokenType::ListBlock, lists, String::new()));
							lists = Vec::new();
						}
						if line.len() > 1 {
							current_block = Token::n_para();
							current_block.attributes = parse_attr(&line[1].content);
							current_block.subtokens = line[1..].to_vec();
							push_token(&mut blocks, &current_block);
						} else {
							next_attr = ftoken.content.to_owned();
						}
					},
					TokenType::Header => {
						if !lists.is_empty() {
							push_token(&mut blocks, &Token::init_sub(TokenType::ListBlock, lists, String::new()));
							lists = Vec::new();
						}
						current_block = ftoken.clone();
						current_block.subtokens = line[1..].to_vec();
						push_token(&mut blocks, &current_block);
					},
					TokenType::Image => {
						if !lists.is_empty() {
							push_token(&mut blocks, &Token::init_sub(TokenType::ListBlock, lists, String::new()));
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
						if !lists.is_empty() {
							push_token(&mut blocks, &Token::init_sub(TokenType::ListBlock, lists, String::new()));
							lists = Vec::new();
						}
						current_block = Token::n_para();
						current_block.subtokens = line.clone();
						push_token(&mut blocks, &current_block);
					},
				}
			}
		}
	}
	if !lists.is_empty() {
		push_token(&mut blocks, &Token::init_sub(TokenType::ListBlock, lists, String::new()));
	}
	blocks
}

pub(crate) fn get_list_depth(token: &Token) -> usize {
	match token.class {
		TokenType::UList | TokenType::ListEl | TokenType::OList | TokenType::NumberedListEl => token.content.len() - 1,
		_ => panic!("Passed a non-list token to get_list_depth"),
	}
}
