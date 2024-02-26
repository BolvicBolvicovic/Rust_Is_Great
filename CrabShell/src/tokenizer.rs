pub const SOURCE_OFFSET_INVALID: usize = u32::MAX as _;

const TOK_MODE_REGULAR_TEXT: TokModes = TokModes(0); // regular text
const TOK_MODE_SUBSHELL: TokModes = TokModes(1 << 0); // inside of subshell parentheses
const TOK_MODE_ARRAY_BRACKETS: TokModes = TokModes(1 << 1); // inside of array brackets
const TOK_MODE_CURLY_BRACES: TokModes = TokModes(1 << 2);
const TOK_MODE_CHAR_ESCAPE: TokModes = TokModes(1 << 3);

impl BitAnd for TokModes {
    type Output = bool;
    fn bitand(self, rhs: Self) -> Self::Output {
        (self.0 & rhs.0) != 0
    }
}
impl BitAndAssign for TokModes {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}
impl BitOrAssign for TokModes {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}
impl Not for TokModes {
    type Output = TokModes;
    fn not(self) -> Self::Output {
        TokModes(!self.0)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenType {
	error,
	string,
	pipe,
	andand,
	oror,
	background,
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

impl Token {
	fn new(t_type: TokenType) -> Token {
		Token {
			offset: 0,
			length: 0,
			error_offset_within_token:
			error_length: 0,
			error: TokenizerError::none,
			type_: t_type,
		}
	}
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
		//Consume whitespaces at the start.
		loop {
			let i = self.token_cursor;
			if self.start.get(i..i + 2) == Some(L!("\\\n")) {
                self.token_cursor += 2;
                self.continue_line_after_comment = true;
			} else if i < self.start(len) && is_wspace_but_nl(self.start.char_at(i)) {
				self.token_cursor += 1;
			} else { break; }
		}
		//Handle comment.
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
		//Here the next char is wrapped in an option to handle the case there is no next_char.
		let next_char = self.start.as_char_slice().get(self.token_cursor + 1).copied();
		//And a buffer that contains the part of the string starting where the cursor is.
		let buffer = &self.start[self.token_cursor..];
		match current_char {
			'\0' => { self.has_next = false; None }
			'\n' | '\r' | ';' => {
				let result = Token::new(TokenType::end);
				result.offset = start_pos as u32;
				result.len = 1;
				self.current_token += 1;
				//Here we are compressing the newline into one single newline (as a token) in case there're many.
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
			'&' => {
				if next_char == Some('&') {
					let mut result = Token::new(TokenType::andand);
					result.offset = start_pos as u32;
					result.len = 2;
					self.token_cursor += 2;
					Some(result)
				} else if next_char == Some('>') || next_char == Some('|') {
					//Handle the redirections with PipeOrRedir::try_from
					let redir = PipeOrRedir::try_from(buff).expect("redir &| &> should work, fix your code");
					let mut result = Token::new(redir.token_type());
					result.offset = start_pos as u32;
					result.len = 2;
					self.token_cursor += 2;
					Some(result)
				} else {
					let mut result = Token::new(TokenType::background);
					result.offset = start_pos as u32;
					result.length = 1;
					Some(result)
				}
			}
			'|' => {
				if next_char == Some('|') {
					let mut result = Token::new(TokenType::oror);
					result.offset = start_pos as u32;
					result.len = 2;
					self.token_cursor += 2;
					Some(result)
				} else if next_char == Some('&') {
				// |& is in bash but it is not logic. If you want to add a stderr and stdout to your pipe, do &|.
					Some(self.call_error(
						TokenizerError::invalid_pipe_ampersand,
						self.token_cursor, self.token_cursor, Some(2), 2))
				} else {
					let pipe = PipeOrRedir::try_from(buff).expect("pipe parse should work, fix your code");
					let mut result = Token::new(pipe.token_type());
					result.offset = start_pos as u32;
					result.pipe = pipe.consumed as u32;
					self.token_cursor += pipe.consumed;
					Some(result)
				}
			}
			'>' || '<' => {
				match PipeOrRedir::try_from(buff) {
					Ok(redir_or_pipe) => {
						if redir_or_pipe.fd < 0 {*
							Some(self.call_error(
								TokenizerError::invalid_redirect,
								self.token_cursor, self.token_cursor,
								Some(redir_or_pipe.consumed), redir_or_pipe.consumed))
						} else {
							let mut result = Token::new(redir_or_pipe.token_type());
							result.offset = start_pos as u32;
							result.len = redir_or_pipe.consumed as u32;
							self.token_cursor += redir_or_pipe.consumed as u32;
							Some(result)
						}
					}
					Err(()) => {
						Some(self.call_error(
							TokenizerError::invalid_redirect,
							self.token_cursor, self.token_cursor, Some(0), 0))
					}
				}
			}
			_ => {
				let error_location = self.token_cursor;
				let redir_or_pipe = if this_char.is_ascii_digit() {
					PipeorRedir::try_from(buff).ok()
				} else {
					None
				};
				match redir_or_pipe {
					Some(redir_or_pipe) => {
						if redir_or_pipe.is_pipe && redir_or_pipe.fd == 0 {
							Some(self.call_error(TokenizerError::invalid_pipe,
								error_location, error_location,
								Some(redir_or_pipe.consumed), redir_or_pipe.consumed))
						} else {
							let mut result = Token::new(redir_or_pipe.token_type());
							result.offset = start_pos as u32;
							result.len = redir_or_pipe.consumed as u32;
							self.token_cursor += redir_or_pipe.consumed;
							Some(result)
						}
					}
					None => { Some(self.read_string()) }
				}
			}
		}
	}
}

impl Tokenizer {
	fn call_error(
		&mut self,
		error_type: TokenizerError,
		token_start: usize,
		error_loc: usize,
		token_length: Option<usize>,
		error_len: usize) -> Token {
		//If you have some problem do some assert!() here.
		match token_length {
			Some(token_length) if self.continue_after_error => {
				assert!(self.token_cursor < error_loc + token_length, "Unable to continue past error.");
				self.token_cursor = error_loc + token_length;
			}
			_ => { self.has_next = false; }
		}
		Token {
			offset: token_start as u32,
			length: token_length.unwrap_or(self.token_cursor - token_start) as u32,
			error_offset_within_token: (error_loc - token_start) as u32,
			error: error_type,
			type_:TokenType::error,
		}
	}
}
