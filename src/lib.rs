extern crate to_default;
use to_default::*;
use ::std::collections::HashMap;

pub fn run(_: &str) -> String {
	unimplemented!()
}

pub struct Session<'a> {
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
	c >= 'a' && c <= 'z'
}

#[test]
fn check_pass() {
	let rules = r#"
		factor = "1"
		term = factor "*" factor
	"#;
	
	let s = Session::new(rules).unwrap();
	assert!(s.check_orphan());
	assert!(s.check_root())
}


#[test]
fn check_violent_root() {
	let rules = r#"
		foo = "0"
		bar = "1"
	"#;
	
	let s = Session::new(rules).unwrap();
	assert!(s.check_orphan());
	assert!(!s.check_root())
}

#[test]
fn check_violent_orphan() {
	let rules = r#"
		foo = bar
	"#;
	
	let s = Session::new(rules).unwrap();
	assert!(!s.check_orphan());
	assert!(s.check_root())
}

#[test]
fn test() {
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

#[test]
fn test_twoway_mapping() {
	let mut id2name = HashMap::new();
	id2name.insert(0, "foo");
	let mut name2id = HashMap::new();
	name2id.insert("foo", 0);
	let ref rules = get_rules(r#"foo="1""#).unwrap();
	let mappings: TwoWayMapping = rules.into();
	assert_eq!(name2id, mappings.name2id);
	assert_eq!(id2name, mappings.id2name)
}

#[test]
fn test_generator() {
	let rules = vec![
		("factor".to_string(), vec![Factor::Terminate(1.to_string())]),
		("term".to_string(),
			vec![
				Factor::Name("factor".to_string()),
				Factor::Terminate("*".to_string()),
				Factor::Name("factor".to_string())])
	];
	
	assert_eq!(generate_decl(&rules), "enum Factor {  }\n\
		enum Term { Factor(Factor), Factor(Factor) }")
}

pub fn get_rules<'a>(source: &'a str) -> Result<Vec<(String, Vec<Factor>)>, String> {
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
						right.push(Factor::Terminate(making.to_default()));
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
			if (left.is_none() && making.is_empty()) || eq {
				return Err("`=` occured in wrong place".to_string())
			} else {
				if left.is_none() {
					left = Some(making.to_default())
				}
				assert!(right.is_empty());
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
					right.push(Factor::Name(making.to_default()))
				}
				ret.push((left.to_default().unwrap(), right.to_default()));
				eq = false
			}
		}
		else if is_whitespace(c) {
			if !making.is_empty() {
				if left.is_none() {
					left = Some(making.to_default())
				}
				else if eq {
					right.push(Factor::Name(making.to_default()))
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
	pub fn new(src: &'a str) -> Result<Session<'a>, String> {
		Ok(Session {
			source: src,
			rules: try!(get_rules(src))
		})
	}
	
	/// This function assume that all non-terminates on the right-hand side
	/// also appear on the left-hand side
	pub fn check_root(&self) -> bool {
		let mut rhs_name_set = HashMap::new();
		let mut lhs_vec = Vec::new();
		
		for &(ref left, ref items) in &self.rules {
			lhs_vec.push(left);
			for item in items {
				if let &Factor::Name(ref name) = item {
					rhs_name_set.insert(name, ());
				}
			}
		}
		println!("{:?}\n{:?}", rhs_name_set, lhs_vec);
		return 1 == lhs_vec.into_iter().filter(|x| { println!("checking {}", x); !rhs_name_set.contains_key(x)}).count()
	}
	
	pub fn check_orphan(&self) -> bool {
		let lhs_set = self.rules.iter().fold(HashMap::new(), |mut acc, x| {
			acc.insert(&x.0, ());
			acc
		});
		self.rules.iter().all(|&(_, ref items)| {
			items.iter().all(|item| if let &Factor::Name(ref name) = item {
				lhs_set.contains_key(name)
			} else {
				true
			})
		})
	}
}

struct TwoWayMapping<'a> {
	name2id: HashMap<&'a str, usize>,
	id2name: HashMap<usize, &'a str>
}

type Rules = Vec<(String, Vec<Factor>)>;

fn generate_decl<'a>(rules: &'a Rules) -> String {
	fn to_name(name: &str) -> String {
		name.chars().enumerate().map(|(idx, c)| if idx == 0 { c.to_uppercase().collect() } else { c.to_string() }).collect()
	}
	rules.iter().map(|&(ref name, ref rule)| format!("enum {} {{ {} }}", to_name(name), {
		rule.iter().flat_map(|item| if let &Factor::Name(ref name) = item {
			Some(to_name(name))
		} else {
			None
		}).map(|x| format!("{}({})", x, x)).collect::<Vec<_>>().join(", ")
	})).collect::<Vec<_>>().join("\n")
}

impl<'a> From<&'a Rules> for TwoWayMapping<'a> {
	fn from(rules: &'a Rules) -> TwoWayMapping<'a> {
		let names: HashMap<&str, _> = rules.iter().fold(HashMap::new(), |mut acc, &(ref left, ref right)| {
			acc.insert(left, ());
			for item in right {
				if let &Factor::Name(ref name) = item {
					acc.insert(name, ());
				}
			}
			acc
		});
		let mut id2name = HashMap::new();
		let mut name2id = HashMap::new();
		for (id, (name, _)) in names.into_iter().enumerate() {
			id2name.insert(id, name);
			name2id.insert(name, id);
		}
		TwoWayMapping {
			id2name: id2name,
			name2id: name2id
		}
	}
}
