use nom::IResult;

enum BaseToken {
	WORD(IResult<&str, &str>),
	ASSIGNMENT_WORD(IResult<&str, &str>),
	NAME(IResult<&str, &str>),
	NEWLINE(IResult<&str, &str>),
	IO_NUMBER(IResult<&str, &str>),
}

enum OperatorToken {
	AND_IF("&&"),
	OR_IF("||"),
	DSEMI(";;"),
	DLESS("<<"),
	DGREAT(">>"),
	LESSAND("<&"),
	GREATAND(">&"),
	LESSGREAT("<>"),
	DLESSDASH("<<-"),
	CLOBBER(">|"),
}

enum ReservedWordToken {
	If("if"),
	Then("then"),
	Else("else"),
	Elif("elif"),
	Fi("fi"),
	Do("do"),
	Done("done"),
	Case("case"),
	Esac("esac"),
	While("while"),
	Until("until"),
	For("for"),
	Lbrace("{"),
	Rbrace("}"),
	Bang("!"),
	In("in")
}
