extern crate lib;
use lib::parser::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut p1: Parser = Parser::new();
    p1.build_ast(&args[1]);
}