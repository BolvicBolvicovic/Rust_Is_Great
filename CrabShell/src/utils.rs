use std::io::{self, Write};

pub fn quote_end(s: &wstr, mut pos: usize, quote: char) -> Option<usize> {
    loop {
        pos += 1;

        let c = s.try_char_at(pos)?;
        if c == '\\' {
            pos += 1;
        } else if c == quote ||
                // Command substitutions also end a double quoted string.  This is how we
                // support command substitutions inside double quotes.
                (quote == '"' && c == '$' && s.as_char_slice().get(pos+1) == Some(&'('))
        {
            return Some(pos);
        }
    }
}

pub fn is_wspace_but_nl(c: char) -> bool {
	match c {
		'\n' => false,
		_ => c.is_ascii_whitespace(),
	}
}

pub fn get_line() -> String {
	print!("🦀 "); io::stdout().flush().unwrap();
	let mut line = String::new();
	io::stdin().read_line(&mut line).expect("...some crab language...");
	line
}
