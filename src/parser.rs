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
    self.set_next_symbol();
    self.parse_program()
  }

  fn parse_program(&mut self) -> bool {    
    if self.symbol.token == Token::Program {
      self.set_next_symbol();

      if self.symbol.category == Type::Identifier {
        self.set_next_symbol();

        if self.symbol.token == Token::Semicolon {
          self.set_next_symbol();
          self.parse_declare_var();
          self.parse_declare_subprograms();
          self.parse_compound_command();
         
          if self.symbol.token == Token::Period  {
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
      self.set_next_symbol();
      self.parse_list_declare_var(false);
    }
  }

  fn parse_list_declare_var(&mut self, ep_closure: bool) {
    self.parse_list_identfiers(ep_closure);
    
    if self.symbol.token == Token::Colon {
      self.set_next_symbol();  
      self.parse_types();

      if self.symbol.token == Token::Semicolon {
        self.set_next_symbol();
        self.parse_list_declare_var(true);
      }
    }  else if !ep_closure {
      panic!("Expected delimiter `:` found `{}` => line {}", self.symbol.token, self.symbol.line);
    }
  }

  fn parse_list_identfiers(&mut self, ep_closure: bool) {
    if self.symbol.category == Type::Identifier {
      self.set_next_symbol();  
      self.parse_list_identfiers_recursive();
    } else if !ep_closure {
      panic!("Expected identifier  found `{:?}` => line {}", self.symbol.category, self.symbol.line);
    }
  }

  fn parse_list_identfiers_recursive(&mut self) {
    if self.symbol.token == Token::Comma {
      self.set_next_symbol();
      
      if self.symbol.category == Type::Identifier {
        self.set_next_symbol();
        self.parse_list_identfiers_recursive();
      } else {
        panic!("Expected identifier  found `{:?}` => line {}", self.symbol.category, self.symbol.line);
      } 
    }
  }

  fn parse_types(&mut self) {
    if self.symbol.token == Token::Integer || self.symbol.token == Token::Real || self.symbol.token == Token::Boolean {
      self.set_next_symbol();
    } else {
      panic!("Expected type `boolean` or `integer` or `real`  found `{}` => line {}", self.symbol.token, self.symbol.line); 
    }
  }

  fn parse_declare_subprograms(&mut self) {
    if self.symbol.token == Token::Procedure {
      self.parse_declare_subprogram(true);
      
      if self.symbol.token == Token::Semicolon {
        self.set_next_symbol();
        self.parse_declare_subprograms();
      }
    }
  }

  fn parse_declare_subprogram(&mut self, ep_closure: bool) {
    if self.symbol.token == Token::Procedure {
      self.set_next_symbol();

      if self.symbol.category == Type::Identifier {
        self.set_next_symbol();
        self.parse_args();
        
        if self.symbol.token == Token::Semicolon {
          self.set_next_symbol();
          self.parse_declare_var();
          self.parse_declare_subprograms();
          self.parse_compound_command();
        }
      } else {
        panic!("Expected identifier  found `{:?}` => line {}", self.symbol.category, self.symbol.line);
      }
    } else if !ep_closure {
      panic!("Expected keyword `procedure`  found `{}` => line {}", self.symbol.token, self.symbol.line);  
    }
  }

  fn parse_args(&mut self) {
    if self.symbol.token == Token::LParentheses {
      self.set_next_symbol();
      self.parse_list_params();

      if self.symbol.token == Token::RParentheses {
        self.set_next_symbol();
      } else {
        panic!("Expected delimiter `)`  found `{}` => line {}", self.symbol.token, self.symbol.line);
      }
    }
  }

  fn parse_list_params(&mut self) {
    self.parse_list_identfiers(false);

    if self.symbol.token == Token::Colon {
      self.set_next_symbol();
      self.parse_types();
      self.parse_list_params_recursive();

    } else {
      panic!("Expected delimiter `:`  found `{}` => line {}", self.symbol.token, self.symbol.line);
    }
  }

  fn parse_list_params_recursive(&mut self) {
    if self.symbol.token == Token::Semicolon {
      self.set_next_symbol();
      self.parse_list_identfiers(false);
    
      if self.symbol.token == Token::Colon {
        self.set_next_symbol();
        self.parse_types();
        self.parse_list_params_recursive();

      } else {
        panic!("Expected delimiter `:`  found `{}` => line {}", self.symbol.token, self.symbol.line);
      }
    }
  }

  fn parse_compound_command(&mut self) {
    if self.symbol.token == Token::Begin {
      self.set_next_symbol();
      self.parse_list_command(true);
     
      if self.symbol.token == Token::End {
        self.set_next_symbol();
      } else {
        panic!("Expected keyword `end`  found `{}` => line {}", self.symbol.token, self.symbol.line);
      }
    } else {
      panic!("Expected keyword `begin`  found `{}` => line {}", self.symbol.token, self.symbol.line);
    }
  }

  fn parse_list_command(&mut self, ep_closure: bool) {
    self.parse_command(ep_closure); 
    self.parse_list_command_recursive();
  }

  fn parse_list_command_recursive(&mut self) {
    if self.symbol.token == Token::Semicolon {
      self.set_next_symbol(); 
      self.parse_command(true);

      self.parse_list_command_recursive();
    }
  }

  fn parse_command(&mut self, ep_closure: bool) {
    if self.symbol.category == Type::Identifier {
      self.set_next_symbol();
      
      if self.symbol.token == Token::Assign {
        self.set_next_symbol();
        self.parse_expr();
      } else {
        self.parse_active_procedure();
      }
    } else if self.symbol.token == Token::Begin {
      self.parse_compound_command();

    } else if self.symbol.token == Token::If {
      self.set_next_symbol();
      self.parse_expr();
      
      if self.symbol.token == Token::Then {
        self.set_next_symbol();
        self.parse_command(false);
      
        self.parse_else();
      } else {
        panic!("Expected keyword `then`  found `{}` => line {}", self.symbol.token, self.symbol.line);
      }
    } else if self.symbol.token == Token::While {
      self.set_next_symbol();
      self.parse_expr();

      if self.symbol.token == Token::Do {
        self.set_next_symbol();
        self.parse_command(false);

      } else {
         panic!("Expected keyword `do`  found `{}` => line {}", self.symbol.token, self.symbol.line);        
      }
    } else if !ep_closure {
       panic!("Expected identifier found `{}` => line {}", self.symbol.token, self.symbol.line);
    }
  }

  fn parse_active_procedure(&mut self) {
    if self.symbol.token == Token::LParentheses {
      self.set_next_symbol();
      self.parse_list_expr();

      if self.symbol.token == Token::RParentheses {
        self.set_next_symbol();
      } else {
        panic!("Expected delimiter `)`  found `{}` => line {}", self.symbol.token, self.symbol.line);
      }
    }
  }

  fn parse_else(&mut self) {
    if self.symbol.token == Token::Else {
      self.set_next_symbol();
      self.parse_command(false);
    }
  } 

  fn parse_list_expr(&mut self) {
    self.parse_expr();
    self.parse_list_expr_recursive();
  }

  fn parse_list_expr_recursive(&mut self) {
    if self.symbol.token == Token::Comma {
      self.set_next_symbol();
      self.parse_expr();
      self.parse_list_expr_recursive();
    }
  }

  fn parse_expr(&mut self) {
    self.parse_simple_expr();

    if self.symbol.category == Type::RelOperator {
      self.set_next_symbol(); 
      self.parse_simple_expr();
    }
  }

  fn parse_simple_expr(&mut self) {
    if self.symbol.token == Token::Add || self.symbol.token == Token::Sub {
      self.set_next_symbol();
      self.parse_term();
      self.parse_simple_expr_recursive();
    } else {
      self.parse_term();
      self.parse_simple_expr_recursive();
    }
  }

  fn parse_simple_expr_recursive(&mut self) {
    if self.symbol.category == Type::AddOperator {
      self.set_next_symbol();
      self.parse_term();
      self.parse_simple_expr_recursive();
    }
  }

  fn parse_term(&mut self) {
    self.parse_factor();
    self.parse_term_recursive();
  }

  fn parse_term_recursive(&mut self) {
    if self.symbol.category == Type::MulOperator {
      self.set_next_symbol();
      self.parse_factor();
      self.parse_term_recursive();
    }
  }

  fn parse_factor(&mut self) {
    if self.symbol.category == Type::Identifier {
      self.set_next_symbol();
      self.parse_active_procedure();

    } else if self.symbol.token == Token::LParentheses {
      self.set_next_symbol();
      self.parse_expr();

      if self.symbol.token == Token::RParentheses {
        self.set_next_symbol();
      }
    } else if self.symbol.category == Type::RealLiteral || self.symbol.category == Type::IntLiteral || 
              self.symbol.token == Token::LitStr("true".to_string()) || self.symbol.token == Token::LitStr("false".to_string()) ||
              self.symbol.token == Token::LitStr("not".to_string()) {
      self.set_next_symbol();
    } else {
      panic!("Expected Factor `id` or `real` or `integer` or `true` or false` or `(` or `not` found `{}` => line {}",
             self.symbol.token, self.symbol.line)
    }   
  }

  #[inline]
  fn set_next_symbol(&mut self) {
    self.symbol = self.scanner.next_symbol();
  }
}

#[test]
fn test_parser_program6() {
  let mut p1: Parser = Parser::new();
  let res = p1.build_ast("files/program6.txt");

  assert!(res);
}

#[test]
fn test_parser_program7() {
  let mut p1: Parser = Parser::new();
  let res = p1.build_ast("files/program7.txt");

  assert!(res);
}

#[test]
fn test_parser_program9() {
  let mut p1: Parser = Parser::new();
  let res = p1.build_ast("files/program9.txt");

  assert!(res);
}

#[test]
fn test_parser_program10() {
  let mut p1: Parser = Parser::new();
  let res = p1.build_ast("files/program10.txt");

  assert!(res);
}

#[test]
fn test_parser_program11() {
  let mut p1: Parser = Parser::new();
  let res = p1.build_ast("files/program11.txt");

  assert!(res);
}

#[test]
fn test_parser_program12() {
  let mut p1: Parser = Parser::new();
  let res = p1.build_ast("files/program12.txt");

  assert!(res);
}

#[test]
fn test_parser_program13() {
  let mut p1: Parser = Parser::new();
  let res = p1.build_ast("files/program13.txt");

  assert!(res);
}

#[test]
fn test_parser_program14() {
  let mut p1: Parser = Parser::new();
  let res = p1.build_ast("files/program14.txt");
  
  assert!(res);
}

#[test]
fn test_parser_program15() {
  let mut p1: Parser = Parser::new();
  let res = p1.build_ast("files/program15.txt");
  
  assert!(res);
}