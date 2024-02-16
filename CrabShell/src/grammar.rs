pub mod lib;

struct program {
	has_command: Option<linebreak, complete_commands, linebreak>,
	has_no_command: Option<linebreak>,
}

struct complete_commands {
	many_command: Option<complete_commands, newline_list, complete_command>,
	one_command: Option<complete_command>,
}

struct complete_command {
	has_separator: Option<list, separtator_op>,
	has_no_sep: Option<list>,
}

struct list {
	full_list: Option<list, separator_op, and_or>,
	no_list: Option<and_or>,
}

struct and_or {
	just_pipe: Option<pipeline>,
	and_line_break: Option<and_or, OperatorToken::AND_IF, linebreak, pipeline>,
	or_line_breal: Option<and_or, OperatorToken::OR_IF, linebreak, pipeline>,
}

struct pipeline {
	no_bang: Option<pipe_sequence>,
	bang: Option<Bang, pipe_sequence>,
}

struct pipe_sequence {
	no_pipe: Option<command>,
	with_pipe: Option<pipe_sequence, OperatorToken::OR, linebreak, command>,
}

struct command {
	simple_cmd: Option<simple_command>,
	compound_cmd: Option<compound_command>,
	compound_cmd_red_list: Option<compound_command, redirect_list>,
	func_def: Option<function_definition>,
}

struct compound_command {
	brace_grp: Option<brace_group>,
	subsh: Option<subshell>,
	for_statement: Option<for_clause>,
	case_statement: Option<case_clause>,
	if_statement: Option<if_clause>,
	while_statement: Option<while_clause>,
	until_statement: Option<until_clause>,
}

struct subshell {
	is_sub: Option<PunctuationToken::Lparenth, compound_list, PunctuationToken::Rparenth>,
}

struct compound_list {
	no_sep: Option<linebreak, term>,
	sep: Option<linebreak, term, separator>,
}

struct term {
	sep: Option<term, separator, and_or>,
	no_sep: Option<and_or>,
}

struct for_clause {
	case_one: Option<ReservedTokenWord::For, BaseToken::NAME, do_group>,
	case_two: Option<ReservedTokenWord::For, BaseToken::NAME, sequential_sep, do_group>,
	case_three: Option<ReservedTokenWord::For, BaseToken::NAME, linebreak, ReservedTokenWord::In, sequential_sep, do_group>,
	case_four: Option<ReservedTokenWord::For, BaseToken::NAME, linebreak, ReservedTokenWord::In, wordlist, do_group>,
	
}

struct wordlist {
	case_one: Option<wordlist, BaseToken::WORD>,
	case_two: Option<BaseToken::WORD>,
}

struct case_clause {
	case_one: Option<ReservedTokenWord::Case, BaseToken::WORD, linebreak, ReservedTokenWord::In, linebreak, case_list, ReservedTokenWord::Esac>,
	case_two: Option<ReservedTokenWord::Case, BaseToken::WORD, linebreak, ReservedTokenWord::In, linebreak, case_list_ns, ReservedTokenWord::Esac>,
	case_three: Option <ReservedTokenWord::Case, BaseToken::WORD, linebreak, ReservedTokenWord::In, linebreak, ReservedTokenWord::Esac>,
}

struct case_list_ns {
	with_case_list: Option<case_list, case_item_ns>,
	no_case_list: Option<case_item_ns>,
}

struct case_list {
	with_case_list: Option<case_list, case_item>,
	no_case_list: Option<case_item>,
}

struct case_item_ns {
	case_one: Option<pattern, PunctuationToken::Rparenth, linebreak>,
	case_two: Option<pattern, PunctuationToken::Rparenth, compound_list>,
	case_three: Option<PunctuationToken::Lparenth, pattern, PunctuationToken::Rparenth, linebreak>,
	case_four: Option<PunctuationToken::Lparenth, pattern, PunctuationToken::Rparenth, compound_list>,
}

struct case_item {
	case_one: Option<pattern, PunctuationToken::Rparenth, linebreak, OperatorToken::DSEMI, linebreak>,
	case_two: Option<pattern, PunctuationToken::Rparenth, compound_list, OperatorToken::DSEMI, linebreak>,
	case_three: Option<PunctuationToken::Lparenth, pattern, PunctuationToken::Rparenth, linebreak, OperatorToken::DSEMI, linebreak>,
	case_four: Option<PunctuationToken::Lparenth, pattern, PunctuationToken::Rparenth, compound_list, OperatorToken::DSEMI, linebreak>,
}

struct pattern {
	rule_four: Option<BaseToken::WORD>,
	no_rule_four: Option<pattern, OperatorToken::OR, BaseToken::WORD>,
}

struct if_clause {
	else_part: Option<ReservedTokenWord::If, compound_list, ReservedTokenWord::Then, compound_list, else_part, ReservedTokenWord::Fi>,
	no_else_part: Option<ReservedTokenWord::If, compound_list, ReservedTokenWord::Then, compound_list, ReservedTokenWord::Fi>,
}

struct else_part {
	elif: Option<ReservedTokenWord::Elif, compound_list, ReservedTokenWord::Then, compound_list>,
	elif_else: Option<ReservedTokenWord::Elif, compound_list, ReservedTokenWord::Then, compound_list, else_part>,
	_else: Option<ResrvedTokenWord::Else, compound_list>,
}

struct while_clause {
	clause: Option<ReservedTokenWord::While, compound_list, do_group>,
}

struct until_clause {
	clause: Option<ReservedTokenWord::Until, compound_list, do_group>,
}

struct function_definition {
	def: Option<fname, PunctuationToken::Lparenth, PunctuationToken::Rparenth, linebreak, function_body>,
}

struct function_body {
	without_redir_list:
	with_redir_list:
} 
