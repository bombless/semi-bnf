extern crate bnf;
fn main() {
	let rules = r#"
		foo = "0"
		bar = "1"
		baz = foo "*" bar
	"#;
	bnf::Session::new(rules).unwrap().check_root();
}
