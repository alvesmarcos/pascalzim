use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process;
use spec::Type;
use spec::Token;

pub struct Symbol {
  token: Token,
  category: Type,
  line: u32
}

pub struct Scanner {
  deque_token: VecDeque<Symbol>,
  buff: char
}

impl Scanner {
  pub fn new(p: String) -> Scanner {
    Scanner { deque_token: VecDeque::new(), buff: '\0' }
  }
  pub fn build_token(&mut self, p: String) {
    let mut reader = BufReader::new(File::open(p).expect("Open failed!"));
    let mut count = 0;

    for line in reader.lines() {
  
      count += 1;
    }
  }

  fn next_char() {

  }

  pub fn next_token(&mut self) -> Symbol {
    return self.deque_token.pop_front().unwrap();
  }

  fn error(self, s: String, abort: bool) {
    println!("Erro lexico");

    if abort { process::exit(0); }
  }
}