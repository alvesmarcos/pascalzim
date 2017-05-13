use lexer::*;
use spec::*;

pub struct Parser {
  scanner: Scanner,
  buff: Symbol
} 

impl Parser {
  pub fn new() -> Parser {
    Parser { scanner: Scanner::new(), buff: Symbol { token: Token::Empty, category: Type::Eof, line: 0 } }
  }
  pub fn build_ast(&mut self, p: &str) -> bool {
    self.scanner.build_token(p);

    self.buff = self.scanner.next_symbol();
    self.parse_program()
  }

  fn parse_program(&mut self) -> bool {    
    if self.buff.token == Token::Program {
      self.buff = self.scanner.next_symbol();

      if self.buff.category == Type::Identifier {
        self.buff = self.scanner.next_symbol();

        if self.buff.token == Token::Semicolon {
           self.buff = self.scanner.next_symbol();
          // self.parse_declare_var();
          
          // self.buff = self.scanner.next_symbol();
          // call declare_subprograms
          // call compound command

          if self.buff.token == Token::Colon  {
             true
          } else {
            panic!("Expected delimiter `.` found `{}` => line {}", self.buff.token, self.buff.line);
          }
        } else {
          panic!("Expected delimiter `;`  found `{}` => line {}", self.buff.token, self.buff.line);
        }  
      } else {
        panic!("Expected identifier  found `{:?}` => line {}", self.buff.category, self.buff.line);
      }
    } else {
      panic!("Expected keyword `program`  found `{}` => line {}", self.buff.token, self.buff.line);
    }
  }

  fn parse_declare_var(&mut self) {
    if self.buff.token == Token::Var {
      // call parse_list_declare_var
    }
  }
}

#[test]
fn test_parser_program() {
  let mut p1: Parser = Parser::new();
  let res = p1.build_ast("files/program7.txt");

  assert!(res);
}