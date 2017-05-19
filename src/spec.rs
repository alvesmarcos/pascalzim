use std::fmt;

#[derive(PartialEq, Debug)]
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
  Power,
  // literal
  LitInt(i32),
  LitReal(f32),
  LitStr(String),
  Empty
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let result = match *self {
      Token::Program => "program".to_string(),
      Token::Var => "var".to_string(),
      Token::Integer => "integer".to_string(),
      Token::Real => "real".to_string(),
      Token::Boolean => "boolean".to_string(),
      Token::Procedure => "procedure".to_string(),
      Token::Begin => "begin".to_string(),
      Token::End => "end".to_string(),
      Token::If => "if".to_string(),
      Token::Then => "then".to_string(),
      Token::Else => "else".to_string(),
      Token::While => "while".to_string(),
      Token::Do => "do".to_string(),
      Token::Not => "not".to_string(),
      Token::Semicolon => ";".to_string(),
      Token::Colon => ":".to_string(),
      Token::Period => ".".to_string(),
      Token::LParentheses => "(".to_string(),
      Token::RParentheses => ")".to_string(),
      Token::Comma => ",".to_string(),
      Token::Assign => ":=".to_string(),
      Token::Equal => "=".to_string(),
      Token::NotEqual => "<>".to_string(),
      Token::GreaterThan => ">".to_string(),
      Token::LessThan => "<".to_string(),
      Token::GreaterThanOrEqual => ">=".to_string(), 
      Token::LessThanOrEqual => "<=".to_string(),
      Token::And => "and".to_string(),
      Token::Or => "or".to_string(),
      Token::Add =>  "+".to_string(),
      Token::Sub => "-".to_string(),
      Token::Mult => "*".to_string(),
      Token::Div => "/".to_string(),
      Token::Power => "** | ^".to_string(),
      Token::LitInt(i) => i.to_string(),
      Token::LitReal(f) => f.to_string(),
      Token::LitStr(ref s) => s.to_string(),
      Token::Empty => "EOF".to_string()
    };
    write!(f, "{}", result)
  }
}

#[derive(PartialEq, Debug)]
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
