extern crate edn;

use edn::parser::Parser;

fn main() {
    let s = r#"{:name/first "John" :name/last "Doe" :hobbies ["code" "code" "code"]
    :some-num 42 :some-set #{1 2 3.5} :some-nil nil}"#;

    let mut parser = Parser::new(s);
    println!("{:?}", parser.read().unwrap().unwrap());
}
