use std::fs;
use pest::iterators::Pair;
use pest::Parser;
use crate::{EraBasicParser, Rule};
use insta::assert_debug_snapshot;

fn read(unparsed_file: &str) -> Pair<Rule> {
    let file = EraBasicParser::parse(Rule::file, unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next().unwrap();
    file
}

#[test]
fn functions() {
    let unparsed_file = fs::read_to_string("example/functions.erb").expect("cannot read file");
    let result = read(&unparsed_file);
    assert_debug_snapshot!(result);
}