extern crate bnf;

fn main() {
	println!("{}", bnf::utils::decl_twoway());
	println!("{}", bnf::utils::plain("hello", 0));
	println!("{}", "fn main() {\
		\n    let mut stream = TwoWay { strm: \"hello\".chars().collect(), ptr: 0 };\
		\n    assert!(parse_plain_id_0(&mut stream).is_ok());\
		\n    assert!(parse_plain_id_0(&mut stream).is_err())\
		\n}")
}