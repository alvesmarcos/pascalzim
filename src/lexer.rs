use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
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

    for line in reader.lines() {
      let mut iter = line.as_ref().unwrap().chars().peekable();

      loop {
        if let Some(c) = iter.next() {
          if c == ' ' { continue; }        
          let (token, category) = match c {
            '+' | '-' | '/' | '*' | '=' | '<' | '>' => self.operators(c, &mut iter),
            ';' | '.' | ':' | '(' | ')' | ',' => self.delimiters(c, &mut iter),
            _ => self.literal_num(c, &mut iter)
          };
          self.deque_token.push_back(Symbol{ token: token, category: category, line: count });
        } else {
          break;
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
        if iter.peek() == Some(&'=') {
          iter.next();
          (Token::LessThanOrEqual, Type::RelOperator)
        } else if iter.peek() == Some(&'>') {
          iter.next();
          (Token::NotEqual, Type::RelOperator)
        } else {
          (Token::LessThan, Type::RelOperator)
        }
      },
      '>' => {
        if iter.peek() == Some(&'=') {
          iter.next();
          (Token::GreaterThanOrEqual, Type::RelOperator)
        } else {
          (Token::GreaterThan, Type::RelOperator)
        }
      }
      _ => unimplemented!()
    }
  }

  fn delimiters(&self, c: char, iter: &mut Peekable<Chars>) -> (Token, Type) {
    match c {
      ';' => (Token::Semicolon, Type::Delimiter),
      '.' => (Token::Colon, Type::Delimiter),
      '(' => (Token::LParentheses, Type::Delimiter),
      ')' => (Token::RParentheses, Type::Delimiter),
      ',' => (Token::Comma, Type::Delimiter),
      ':' => {
        if iter.peek() == Some(&'=') {
          iter.next();
          (Token::Assign, Type::Command)
        } else {
          (Token::Period, Type::Delimiter) 
        }
      },
      _ => unimplemented!()
    }
  }

  fn literal_num(&self, c: char, iter: &mut Peekable<Chars>) -> (Token, Type) {
    let mut num = c.to_string();

    while self.is_digit(iter) {
      num.push(iter.next().unwrap());
    }
    
    if iter.peek() == Some(&'.') {
      num.push(iter.next().unwrap());
      
      while self.is_digit(iter) {
        num.push(iter.next().unwrap());
      } 
      (Token::LitReal(num.parse().unwrap()), Type::RealLiteral)
    } else {
      println!("{}", num);
      (Token::LitInt(num.parse().unwrap()), Type::IntLiteral)
    }
  }

  pub fn next_symbol(&mut self) -> Symbol {
    return self.deque_token.pop_front().expect("Deque token is empty!");
  }

  fn is_digit(&self, iter:&mut Peekable<Chars>) -> bool {
    match iter.peek() {
      Some(c) => c.is_digit(10),
      _ => false
    }
  }
}

#[test]
fn test_token_operator() {
  let mut s: Scanner = Scanner::new();
  s.build_token("files/program1.txt");
 
  assert_eq!(s.next_symbol().token, Token::Add);
  assert_eq!(s.next_symbol().token, Token::Sub);
  assert_eq!(s.next_symbol().token, Token::Mult);
  assert_eq!(s.next_symbol().token, Token::Div);
  assert_eq!(s.next_symbol().token, Token::Equal);
  assert_eq!(s.next_symbol().token, Token::LessThan);
  assert_eq!(s.next_symbol().token, Token::GreaterThan);
  assert_eq!(s.next_symbol().token, Token::LessThanOrEqual);
  assert_eq!(s.next_symbol().token, Token::GreaterThanOrEqual);
  assert_eq!(s.next_symbol().token, Token::NotEqual);
}

#[test]
fn test_token_delimiter() {
  let mut s: Scanner = Scanner::new();
  s.build_token("files/program2.txt");

  assert_eq!(s.next_symbol().token, Token::Semicolon);
  assert_eq!(s.next_symbol().token, Token::Colon);
  assert_eq!(s.next_symbol().token, Token::Period);
  assert_eq!(s.next_symbol().token, Token::LParentheses);
  assert_eq!(s.next_symbol().token, Token::RParentheses);
  assert_eq!(s.next_symbol().token, Token::Comma);
  assert_eq!(s.next_symbol().token, Token::Assign);
}

#[test]
fn test_token_literal_num() {
  let mut s: Scanner = Scanner::new();
  s.build_token("files/program3.txt");
 
  assert_eq!(s.next_symbol().token, Token::LitInt(22));
  assert_eq!(s.next_symbol().token, Token::LitInt(19));
  assert_eq!(s.next_symbol().token, Token::LitReal(11.2));
  assert_eq!(s.next_symbol().token, Token::LitReal(932.2));
  assert_eq!(s.next_symbol().token, Token::LitInt(1));
  assert_eq!(s.next_symbol().token, Token::LitReal(1.0));
}
