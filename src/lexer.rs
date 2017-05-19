use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Peekable;
use std::str::Chars;
use spec::*;

#[derive(Debug)]
pub struct Symbol {
  pub token: Token,
  pub category: Type,
  pub line: u32
}

pub struct Scanner {
  deque_token: VecDeque<Symbol>
}

impl Scanner {
  pub fn new() -> Scanner {
    Scanner { deque_token: VecDeque::new() }
  }
  
  pub fn build_token(&mut self, p: &str) {
    let reader = BufReader::new(File::open(p).expect("Open failed!"));
    let mut count = 1;
    let mut block_comment = false;

    for line in reader.lines() {
      let mut iter = line.as_ref().unwrap().chars().peekable();

      loop {
        if let Some(c) = iter.next() {
          if c == ' ' { 
            while iter.peek() == Some(&' ') { iter.next(); }
          } else if c == '{' || block_comment {
            block_comment = true;
            while iter.peek() != Some(&'}') && iter.peek() != None { iter.next(); }
            if iter.next() == Some('}') || c == '}' { block_comment = false; }
          } else {
            let (token, category) = match c {
              '+' | '-' | '/' | '*' | '=' | '<' | '>' | '^' => self.operators(c, &mut iter),
              ';' | '.' | ':' | '(' | ')' | ',' => self.delimiters(c, &mut iter),
              _ => self.literal(c, &mut iter)
            };
            self.deque_token.push_back(Symbol{ token: token, category: category, line: count });
          }
        } else {
          break;
        }
      }
      count += 1;
    }
    if block_comment { panic!("Error: Unterminated comment"); }
  }

  fn operators(&self, c: char, iter:&mut Peekable<Chars>) -> (Token, Type) {
    match c {
      '+' => (Token::Add, Type::AddOperator),
      '-' => (Token::Sub, Type::AddOperator),
      '/' => (Token::Div, Type::MulOperator),
      '^' => (Token::Power, Type::MulOperator),
      '*' => {
        if iter.peek() == Some(&'*'){
          iter.next();
          (Token::Power, Type::MulOperator)  
        } else {
          (Token::Mult, Type::MulOperator)
        }
      },
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
      '.' => (Token::Period, Type::Delimiter),
      '(' => (Token::LParentheses, Type::Delimiter),
      ')' => (Token::RParentheses, Type::Delimiter),
      ',' => (Token::Comma, Type::Delimiter),
      ':' => {
        if iter.peek() == Some(&'=') {
          iter.next();
          (Token::Assign, Type::Command)
        } else {
          (Token::Colon, Type::Delimiter) 
        }
      },
      _ => unimplemented!()
    }
  }

  fn literal(&self, c: char, iter: &mut Peekable<Chars>) -> (Token, Type) {
    if c.is_digit(10) {
      self.literal_num(c, iter)
    } else if c.is_alphabetic() {
      self.literal_str(c, iter)
    } else {
      panic!("Error: Unexpected Symbol => {}", c)
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
      (Token::LitInt(num.parse().unwrap()), Type::IntLiteral)
    }
  }

  fn literal_str(&self, c: char, iter: &mut Peekable<Chars>) -> (Token, Type) {
    let mut word = c.to_string();

    while self.is_alphanumeric_or_underl(iter) {
      word.push(iter.next().unwrap());
    }
    match &*word {
      "program" => (Token::Program, Type::Keyword),
      "var"  => (Token::Var, Type::Keyword),
      "integer" => (Token::Integer, Type::Keyword),
      "real" => (Token::Real, Type::Keyword),
      "boolean" => (Token::Boolean, Type::Keyword),
      "procedure" => (Token::Procedure, Type::Keyword),
      "begin" => (Token::Begin, Type::Keyword),
      "end" => (Token::End, Type::Keyword),
      "if" => (Token::If, Type::Keyword),
      "then" => (Token::Then, Type::Keyword),
      "else" => (Token::Else, Type::Keyword),
      "while" => (Token::While, Type::Keyword),
      "do" => (Token::Do, Type::Keyword),
      "not" => (Token::Not, Type::Keyword),
      "or" => (Token::Or, Type::AddOperator),
      "and" => (Token::And, Type::MulOperator),
      _ => (Token::LitStr(word), Type::Identifier)
    }
  }

  pub fn next_symbol(&mut self) -> Symbol {
    self.deque_token.pop_front().expect("Error: Deque token is empty!")
  }

  fn is_digit(&self, iter:&mut Peekable<Chars>) -> bool {
    match iter.peek() {
      Some(c) => c.is_digit(10),
      _ => false
    }
  }

  fn is_alphanumeric_or_underl(&self, iter: &mut Peekable<Chars>) -> bool {
    match iter.peek() {
      Some(c) => if c.is_alphanumeric() || c==&'_' { true } else { false },
      _ => false
    }
  }
}

#[test]
fn test_print_vecdeque() {
  let mut s: Scanner = Scanner::new();
  s.build_token("files/program8.txt");
  for num in s.deque_token.iter() {
    println!("{:?}", num);
  }
  assert!(true);
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
  assert_eq!(s.next_symbol().token, Token::And);
  assert_eq!(s.next_symbol().token, Token::Or);
}

#[test]
fn test_token_delimiter() {
  let mut s: Scanner = Scanner::new();
  s.build_token("files/program2.txt");

  assert_eq!(s.next_symbol().token, Token::Semicolon);
  assert_eq!(s.next_symbol().token, Token::Period);
  assert_eq!(s.next_symbol().token, Token::Colon);
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

#[test]
fn test_token_literal_str() {
  let mut s: Scanner = Scanner::new();
  s.build_token("files/program5.txt");
 
  assert_eq!(s.next_symbol().token, Token::LitStr("qualquer".to_string()));
  assert_eq!(s.next_symbol().token, Token::LitStr("coisa".to_string()));
  assert_eq!(s.next_symbol().token, Token::LitStr("aqui".to_string()));
  assert_eq!(s.next_symbol().token, Token::LitStr("pega".to_string()));
  assert_eq!(s.next_symbol().token, Token::LitStr("literal".to_string())); 
  assert_eq!(s.next_symbol().token, Token::LitStr("string".to_string()));   
}

#[test]
fn test_token_keywords() {
  let mut s: Scanner = Scanner::new();
  s.build_token("files/program4.txt");
 
  assert_eq!(s.next_symbol().token, Token::Var);
  assert_eq!(s.next_symbol().token, Token::End);
  assert_eq!(s.next_symbol().token, Token::If);
  assert_eq!(s.next_symbol().token, Token::Then);
  assert_eq!(s.next_symbol().token, Token::Integer);
  assert_eq!(s.next_symbol().token, Token::Real);
  assert_eq!(s.next_symbol().token, Token::Boolean);
  assert_eq!(s.next_symbol().token, Token::Procedure);
  assert_eq!(s.next_symbol().token, Token::Begin);
  assert_eq!(s.next_symbol().token, Token::Else);
  assert_eq!(s.next_symbol().token, Token::While);
  assert_eq!(s.next_symbol().token, Token::Do);
  assert_eq!(s.next_symbol().token, Token::Not);
  assert_eq!(s.next_symbol().token, Token::Program);  
}

#[test]
fn test_token_program() {
  let mut s: Scanner = Scanner::new();
  s.build_token("files/program6.txt");

  assert_eq!(s.next_symbol().token, Token::Program);
  assert_eq!(s.next_symbol().token, Token::LitStr("teste".to_string()));
  assert_eq!(s.next_symbol().token, Token::Semicolon);
  assert_eq!(s.next_symbol().token, Token::Var);
  assert_eq!(s.next_symbol().token, Token::LitStr("valor1".to_string()));
  assert_eq!(s.next_symbol().token, Token::Colon);
  assert_eq!(s.next_symbol().token, Token::Integer);
  assert_eq!(s.next_symbol().token, Token::Semicolon);
  assert_eq!(s.next_symbol().token, Token::LitStr("valor2".to_string()));
  assert_eq!(s.next_symbol().token, Token::Colon);
  assert_eq!(s.next_symbol().token, Token::Real);
  assert_eq!(s.next_symbol().token, Token::Semicolon);
  assert_eq!(s.next_symbol().token, Token::Begin);
  assert_eq!(s.next_symbol().token, Token::LitStr("valor1".to_string()));
  assert_eq!(s.next_symbol().token, Token::Assign);
  assert_eq!(s.next_symbol().token, Token::LitInt(10));
  assert_eq!(s.next_symbol().token, Token::Semicolon);
  assert_eq!(s.next_symbol().token, Token::End);
  assert_eq!(s.next_symbol().token, Token::Period);
}

#[test]
fn test_token_power() {
  let mut s: Scanner = Scanner::new();
  s.build_token("files/program8.txt");

  assert_eq!(s.next_symbol().token, Token::Program);
  assert_eq!(s.next_symbol().token, Token::LitStr("teste".to_string()));
  assert_eq!(s.next_symbol().token, Token::Semicolon);
  assert_eq!(s.next_symbol().token, Token::Var);
  assert_eq!(s.next_symbol().token, Token::LitStr("valor1".to_string()));
  assert_eq!(s.next_symbol().token, Token::Colon);
  assert_eq!(s.next_symbol().token, Token::Integer);
  assert_eq!(s.next_symbol().token, Token::Semicolon);
  assert_eq!(s.next_symbol().token, Token::LitStr("valor2".to_string()));
  assert_eq!(s.next_symbol().token, Token::Colon);
  assert_eq!(s.next_symbol().token, Token::Real);
  assert_eq!(s.next_symbol().token, Token::Semicolon);
  assert_eq!(s.next_symbol().token, Token::Begin);
  assert_eq!(s.next_symbol().token, Token::LitStr("valor1".to_string()));
  assert_eq!(s.next_symbol().token, Token::Assign);
  assert_eq!(s.next_symbol().token, Token::LitInt(10));
  assert_eq!(s.next_symbol().token, Token::Power);
  assert_eq!(s.next_symbol().token, Token::LitInt(8));
  assert_eq!(s.next_symbol().token, Token::Semicolon);

  //  valor2 := 10^8;
  assert_eq!(s.next_symbol().token, Token::LitStr("valor2".to_string()));
  assert_eq!(s.next_symbol().token, Token::Assign);
  assert_eq!(s.next_symbol().token, Token::LitInt(10));
  assert_eq!(s.next_symbol().token, Token::Power);
  assert_eq!(s.next_symbol().token, Token::LitInt(8));
  assert_eq!(s.next_symbol().token, Token::Semicolon);

  //  end.
  assert_eq!(s.next_symbol().token, Token::End);
  assert_eq!(s.next_symbol().token, Token::Period);
}