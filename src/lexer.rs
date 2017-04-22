pub enum Keyword {
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
  Not
}

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

pub struct Symbol {
  token: String,
  type: Type,
  line: u32
}