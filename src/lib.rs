pub fn run(s: &str) -> String {
	unimplemented!()
}

struct Session<'a> {
	source: &'a str,
	rules: Vec<(String, Vec<Factor>)>
}

#[derive(Eq, PartialEq, Debug)]
pub enum Factor {
	Terminate(String),
	Name(String)
}

fn is_whitespace(c: char) -> bool {
	" \r\t".chars().any(|x| x == c)
}

fn is_ident_char(c: char) -> bool {
	(b'a' .. b'z').any(|x| x as char == c)
}

#[test]
pub fn test() {
	assert_eq!(Ok(vec![("a".to_string(), vec![Factor::Terminate("*".to_string())])]), get_rules(r#"
		a = "*"
	"#));
	
	assert_eq!(Ok(vec![]), get_rules(""));
	
	assert_eq!(Ok(vec![
		("factor".to_string(), vec![Factor::Terminate(1.to_string())]),
		("term".to_string(),
			vec![
				Factor::Name("factor".to_string()),
				Factor::Terminate("*".to_string()),
				Factor::Name("factor".to_string())])
	]), get_rules(r#"
		factor = "1"
		term = factor "*" factor
	"#))
}

pub fn get_rules<'a>(source: &'a str) -> Result<Vec<(String, Vec<Factor>)>, String> {
	use std::mem::replace;
	let mut inside_string = InsideString::NonString;
	let mut left = None;
	let mut eq = false;
	let mut right = Vec::new();
	let mut making = String::new();
	let mut ret = Vec::new();
	
	// 4 cases here:
	// 1. left side ident
	// 2. `=`
	// 3. terminate on the right side
	// 4. non-terminate on the right side
	
	#[derive(Eq, PartialEq, Debug)] enum InsideString { Escape, InString, NonString }
	
	for c in source.chars() {
		if c == '"' {
			if eq {
				match inside_string {
					InsideString::NonString => inside_string = InsideString::InString,
					InsideString::InString => {
						right.push(Factor::Terminate(replace(&mut making, String::new())));
						inside_string = InsideString::NonString
					}
					InsideString::Escape => {
						making.push('"');
						inside_string = InsideString::InString
					}
				}
			} else {
				return Err("`\"` occured in wrong place".to_string())
			}
		}
		else if c == '\\' {
			match inside_string {
				InsideString::NonString => return Err("`\\` occured in wrong place".to_string()),
				InsideString::InString => {
					inside_string = InsideString::Escape
				}
				InsideString::Escape => {
					making.push('\\');
				}
			}
		}
		else if inside_string == InsideString::InString {
			making.push(c)
		}
		else if inside_string == InsideString::Escape {
			if c == 'n' {
				making.push('\n')
			} else {
				making.push(c)
			}
		}
		else if c == '=' {
			if left.is_none() || !making.is_empty() || !right.is_empty() {
				return Err("`=` occured in wrong place".to_string())
			} else {
				eq = true
			}
		}
		else if c == '\n' {
			let empty = left.is_none() && !eq && making.is_empty() && right.is_empty();
			let valid_line = left.is_some() && eq && !(making.is_empty() && right.is_empty());
			if inside_string != InsideString::NonString || !(empty || valid_line) {
				return Err("`\\n` occured in wrong place".to_string())
			} else if !empty {
				if !making.is_empty() {
					right.push(Factor::Name(replace(&mut making, String::new())))
				}
				ret.push((replace(&mut left, None).unwrap(), replace(&mut right, Vec::new())));
				eq = false
			}
		}
		else if is_whitespace(c) {
			if !making.is_empty() {
				if left.is_none() {
					left = Some(replace(&mut making, String::new()))
				}
				else if eq {
					right.push(Factor::Name(replace(&mut making, String::new())))
				}
			}
		}
		else if is_ident_char(c) {
			making.push(c)
		} else {
			return Err(format!("illegal character `{}`", c))
		}
	}
	
	let empty = left.is_none() && !eq && making.is_empty() && right.is_empty();
	let valid_line = left.is_some() && eq && !(making.is_empty() && right.is_empty());
	if !empty {
		if valid_line {
			if !making.is_empty() {
				right.push(Factor::Name(making))
			}
			ret.push((left.unwrap(), right))
		} else {
			return Err("unexpected terminating".to_string())
		}
	}
	return Ok(ret)
}

impl<'a> Session<'a> {
	fn new(_: &'a str) -> () {
		
	}
}