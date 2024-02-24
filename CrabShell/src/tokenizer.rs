pub enum TokenType {
	error,
	string,
	pipe,
	andand,
	oror,
	semicol,
	redirect,
	comment,
}

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
	offset: u32,
	lenght: u32,
	error_offset_within_token: u32,
	error: TokenizerError,
	type_: TokenType,
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
