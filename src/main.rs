use pest_derive::Parser;

#[cfg(test)]
mod snapshot_test;

#[derive(Parser)]
#[grammar = "erabasic.pest"]
pub struct EraBasicParser;

fn main() {
    println!("Hello, world!");
}
