// Autor: Marcos Alves
// Data: 22/04/2017
// Sobre: Espceficição reduzida da linguagem pascal
use std::fmt;

pub enum Token {
  // keywords
  Program,
  Var,
  Integer,
  Real,
  Boolean,
  Procedure,
  Begin,
  End,
  If,
  Then,
  Else,
  While,
  Do,
  Not,
  // delimiters
  Semicolon,
  Period,
  Colon,
  LParentheses,
  RParentheses,
  LBrace,
  RBrace,
  Comma,
  // operators
  Assign,
  Equal,
  NotEqual,
  GreaterThan,
  LessThan,
  GreaterThanOrEqual,
  LessThanOrEqual,
  And,
  Or,
  Add, 
  Sub,
  Mult,
  Div,
  // literal
  Int(i32),
  Float(f32),
  Str(String)
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let result = match *self {
      Token::Program => "program",
      Token::Var => "var",
      Token::Integer => "integer",
      Token::Real => "real",
      Token::Boolean => "boolean",
      Token::Procedure => "procedure",
      Token::Begin => "begin",
      Token::End => "end",
      Token::If => "if",
      Token::Then => "then",
      Token::Else => "else",
      Token::While => "while",
      Token::Do => "do",
      Token::Not => "not",
      Token::Semicolon => ";",
      Token::Period => ":",
      Token::Colon => ".",
      Token::LParentheses => "(",
      Token::RParentheses => ")",
      Token::LBrace => "{",
      Token::RBrace => "}",
      Token::Comma => ",",
      Token::Assign => ":",
      Token::Equal => "=",
      Token::NotEqual => "<>",
      Token::GreaterThan => ">",
      Token::LessThan => "<",
      Token::GreaterThanOrEqual => ">=", 
      Token::LessThanOrEqual => "<=",
      Token::And => "and",
      Token::Or => "or",
      Token::Add =>  "+",
      Token::Sub => "-",
      Token::Mult => "*",
      Token::Div => "/",
      Token::Int(i) => "2",
      Token::Float(f) => "2",
      Token::Str(ref s) => "s.to_string()"
    };
    write!(f, "{}", result)
  }
}

#[derive(Debug)]
pub enum Type {
  Keyword,
  Identifier,
  IntLiteral,
  RealLiteral,
  Delimiter,
  Command,
  RelOperator,
  AddOperator,
  MulOperator,
  Eof
}
