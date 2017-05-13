use lexer::*;
use spec::*;

pub struct Parser {
  scanner: Scanner,
  symbol: Symbol
} 

impl Parser {
  pub fn new() -> Parser {
    Parser { scanner: Scanner::new(), symbol: Symbol { token: Token::Empty, category: Type::Eof, line: 0 } }
  }
  pub fn build_ast(&mut self, p: &str) -> bool {
    self.scanner.build_token(p);

    self.symbol = self.scanner.next_symbol();
    self.parse_program()
  }

  fn parse_program(&mut self) -> bool {    
    if self.symbol.token == Token::Program {
      self.symbol = self.scanner.next_symbol();

      if self.symbol.category == Type::Identifier {
        self.symbol = self.scanner.next_symbol();

        if self.symbol.token == Token::Semicolon {
          self.symbol = self.scanner.next_symbol();
          self.parse_declare_var();
          // self.symbol = self.scanner.next_symbol();
          // call declare_subprograms
          // self.symbol = self.scanner.next_symbol();
          // call compound command
     
          if self.symbol.token == Token::Colon  {
            true
          } else {
            panic!("Expected delimiter `.` found `{}` => line {}", self.symbol.token, self.symbol.line);
          }
        } else {
          panic!("Expected delimiter `;`  found `{}` => line {}", self.symbol.token, self.symbol.line);
        }  
      } else {
        panic!("Expected identifier  found `{:?}` => line {}", self.symbol.category, self.symbol.line);
      }
    } else {
      panic!("Expected keyword `program`  found `{}` => line {}", self.symbol.token, self.symbol.line);
    }
  }

  fn parse_declare_var(&mut self) {
    if self.symbol.token == Token::Var {
      self.symbol = self.scanner.next_symbol();
      self.parse_list_declare_var(false);
    }
  }

  fn parse_list_declare_var(&mut self, ep_closure: bool) {
    self.parse_list_identfiers(ep_closure);
    
    if self.symbol.token == Token::Period {
      self.symbol = self.scanner.next_symbol();  
      self.parse_types();
     
      self.symbol = self.scanner.next_symbol();
      
      if self.symbol.token == Token::Semicolon {
        self.symbol = self.scanner.next_symbol();
        self.parse_list_declare_var(true);
      }
    }  else if !ep_closure {
      panic!("Expected delimiter `:` found `{}` => line {}", self.symbol.token, self.symbol.line);
    }
  }

  fn parse_list_identfiers(&mut self, ep_closure: bool) {
    if self.symbol.category == Type::Identifier {
      self.symbol = self.scanner.next_symbol();  
      self.parse_list_identfiers_recursive();
    } else if !ep_closure {
      panic!("Expected identifier  found `{:?}` => line {}", self.symbol.category, self.symbol.line);
    }
  }

  fn parse_list_identfiers_recursive(&mut self) {
    if self.symbol.token == Token::Comma {
      self.symbol = self.scanner.next_symbol();
      
      if self.symbol.category == Type::Identifier {
        self.symbol = self.scanner.next_symbol();
        self.parse_list_identfiers_recursive();
      } else {
        panic!("Expected identifier  found `{:?}` => line {}", self.symbol.category, self.symbol.line);
      } 
    }
  }

  fn parse_types(&mut self) {
    if self.symbol.token != Token::Integer && self.symbol.token != Token::Real && self.symbol.token != Token::Boolean {
      panic!("Expected type `boolean` or `integer` or `real`  found `{}` => line {}", self.symbol.token, self.symbol.line);      
    }
  }
}

#[test]
fn test_parser_program7() {
  let mut p1: Parser = Parser::new();
  let res = p1.build_ast("files/program7.txt");

  assert!(res);
}