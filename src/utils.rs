pub fn plain(s: &str, id: usize) -> String {
	format!("fn parse_plain_id_{}(s: &mut TwoWay) -> Result<(), ()> {{\
		\n    let ptr = s.ptr();\
		\n    for c in {:?}.chars() {{\
		\n        if Some(c) != s.read() {{\
		\n            s.set(ptr);\
		\n            return Err(())\
		\n        }}\
		\n    }}\
		\n    Ok(())\
		\n}}", id, s)
}

pub fn indent(s: &str) -> String {
	let mut indent = true;
	let mut ret = String::new();
	for c in s.chars() {
		if indent {
			ret.push_str("    ");
			indent = false
		}
		ret.push(c);
		if c == '\n' {
			indent = true
		}
	}
	ret
}

pub fn decl_twoway() -> String {
	let decl = "struct TwoWay { ptr: usize, strm: Vec<char> }\n";
	let set = "pub fn set(&mut self, ptr: usize) { self.ptr = ptr }\n";
	let ptr = "pub fn ptr(&self) -> usize { self.ptr }\n";
	let read = "pub fn read(&mut self) -> Option<char> {\
		\n    if self.ptr < self.strm.len() {\
		\n        let ret = self.strm[self.ptr];\
		\n        self.ptr += 1;\
		\n        Some(ret)\
		\n    } else {\
		\n        None\
		\n    }\
		\n}\
		\n";
	format!("{}\nimpl TwoWay {{\n{}\n{}\n{}}}", decl, indent(set), indent(ptr), indent(read))
}
