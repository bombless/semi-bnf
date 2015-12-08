extern crate bnf;
fn main() {
	bnf::get_rules(r#"
		a = "*"
	"#);
}
