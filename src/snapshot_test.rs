use std::fs;
use pest::Parser;
use crate::{EraBasicParser, Rule};
use insta::assert_debug_snapshot;

#[test]
fn functions() {
    let unparsed_file = fs::read_to_string("example/functions.erb").expect("cannot read file");
    let file = EraBasicParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse");
    assert_debug_snapshot!(file);
}

#[test]
fn if_statement() {
    let unparsed_file = fs::read_to_string("example/if_statement.erb").expect("cannot read file");
    let file = EraBasicParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse");
        
    assert_debug_snapshot!(file);
}