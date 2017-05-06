use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;
use std::iter::Peekable;
use std::str::Chars;
use spec::Type;
use spec::Token;

pub struct Symbol {
  token: Token,
  category: Type,
  line: u32
}

pub struct Scanner {
  deque_token: VecDeque<Symbol>,
}

impl Scanner {
  pub fn new() -> Scanner {
    Scanner { deque_token: VecDeque::new() }
  }
  pub fn build_token(&mut self, p: &str) {
    let mut reader = BufReader::new(File::open(p).expect("Open failed!"));
    let mut count = 1;
    let mut flag_next;

    for line in reader.lines() {
      let mut iter = line.as_ref().unwrap().chars().peekable();
      flag_next = true;
      
      while flag_next {
        if let Some(c) = iter.next() {
          if c == ' ' { continue; }
          
          let (token, category) = match c {
            '+' | '-' | '/' | '*' | '=' | '<' | '>' => self.operators(c, &mut iter),
            _ => unimplemented!()
          };
          println!("{}", token);
          self.deque_token.push_back(Symbol{ token: token, category: category, line: count });
          flag_next = true;
        } else {
          flag_next = false;
        }
      }
      count += 1;
    }
  }

  fn operators(&self, c: char, iter:&mut Peekable<Chars>) -> (Token, Type) {
    match c {
      '+' => (Token::Add, Type::AddOperator),
      '-' => (Token::Sub, Type::AddOperator),
      '/' => (Token::Div, Type::MulOperator),
      '*' => (Token::Mult, Type::MulOperator),
      '=' => (Token::Equal, Type::RelOperator),
      '<' => {
        if iter.peek().unwrap() == &'=' {
          iter.next();
          (Token::LessThanOrEqual, Type::RelOperator)
        } else if iter.peek().unwrap() == &'>' {
          iter.next();
          (Token::NotEqual, Type::RelOperator)
        } else {
          (Token::LessThan, Type::RelOperator)
        }
      },
      '>' => {
        if iter.peek().unwrap() == &'=' {
          iter.next();
          (Token::GreaterThanOrEqual, Type::RelOperator)
        } else {
          (Token::GreaterThan, Type::RelOperator)
        }
      }
      _ => unimplemented!()
    }
  }

  pub fn next_token(&mut self) -> Symbol {
    return self.deque_token.pop_front().unwrap();
  }

  fn error(&self, s: String, abort: bool) {
    println!("Erro lexico");

    if abort { process::exit(0); }
  }
}

#[test]
fn test_token_operator1() {
  let mut s: Scanner = Scanner::new();
  s.build_token("files/program1.txt");
  assert_eq!(s.next_token().token, Token::GreaterThanOrEqual);
  assert_eq!(s.next_token().token, Token::Mult);
  assert_eq!(s.next_token().token, Token::Add);
  assert_eq!(s.next_token().token, Token::Div);
}