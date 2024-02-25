#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenType {
	error,
	string,
	pipe,
	andand,
	oror,
	semicol,
	redirect,
	comment,
	end,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenizerError {
	none,
	unterminated_quote,
	unterminated_subshell,
	invalid_redirect,
	invalid_pipe,
	invalid_pipe_ampersand,
	closing_unopened_subshell,
	closing_unopened_brace,
	unterminated_brace,
	expected_pclose_found_bclose,
    expected_bclose_found_pclose,
}

pub struct Token {
	pub offset: u32,
	pub lenght: u32,
	pub error_offset_within_token: u32,
	pub error: TokenizerError,
	pub type_: TokenType,
}

pub struct PipeOrRedir {
	fd: i16,
	is_pipe: bool,
//	Should create a redir mod in a redir file \\
//	mode: RedirectionMode,
	stderr_merge: bool,
	chars_consumed: usize,
}

pub enum MoveWordStyle {
	Punctuation,
	PathComponents,
	Whitespace,
}

pub struct MoveWordStateMachine {
	state: u8,
	style: MoveWordStyle,
}

pub struct Tokenizer {
	token_cursor: usize,
	start: String,
	has_next: bool,
	accept_unfinished: bool,
	show_blank_line: bool,
	show_comment: bool,
	continue_after_error: bool,
	continue_after_comment: bool,
}

impl Iterator for Tokenizer {
	fn next(&mut self) -> Option<Token> {
		if !self.has_next {
			return None;
		}
		///Consume whitespaces at the start.
		loop {
			let i = self.token_cursor;
			if self.start.get(i..i + 2) == Some(L!("\\\n")) {
                self.token_cursor += 2;
                self.continue_line_after_comment = true;
			} else if i < self.start(len) && is_wspace_but_nl(self.start.char_at(i)) {
				self.token_cursor += 1;
			} else { break; }
		}
		///Handle comment.
		while self.start.char_at(self.token_cursor) == '#' {
			let comment_start = self.token_cursor;
			loop {
				if self.token_cursor == self.start.len() || self.start.char_at(self.token_cursor) == '\n' {
					break ;
				}
				self.token_cursor += 1;
			}
			let comment_len = self.token_cursor - comment_start;
			if continue_after_comment && self.start.char_at(self.token_cursor) == '\n' {
				self.token_cursor += 1;
			}
			if self.show_comments {
				let mut result = Token::new(TokenType::comment);
				result.offset = comment_start as u32;
				result.len = comment_len as u32;
				return Some(result);
			}
			while self.token_cursor < self.start.len() && iswspace_but_nl(self.start.char_at(self.token_cursor)) {
				self.token_cursor += 1;
			}
		}
		self.continue_after_comment = false;
		let start_pos = self.token_cursor;
		let current_char = self.start.char_at(self.token_cursor);
		///Here the next char is wrapped in an option to handle the case there is no next_char.
		let next_char = self.start.as_char_slice().get(self.token_cursor + 1).copied();
		///And a buffer that contains the part of the string starting where the cursor is.
		let buffer = &self.start[self.token_cursor..];
		match current_char {
			'\0' => { self.has_next = false; None }
			'\n' | '\r' | ';' => {
				let result = Token::new(TokenType::end);
				result.offset = start_pos as u32;
				result.len = 1;
				self.current_token += 1;
				///Here we are compressing the newline into one single newline (as a token) in case there're many.
				if !self.show_blank_line {
					while self.token_cursor < self.start.len() {
						let c = self.start.char_at(self.token_cursor);
						if !c.is_ascii_whitespace {
							break ;
						}
						self.token_cursor += 1;
					}
				}
				Some(result);
			}
		}
	}
}
