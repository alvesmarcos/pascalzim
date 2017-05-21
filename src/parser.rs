use lexer::*;
use spec::*;


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Category {
  Integer,
  Real,
  Boolean,
  Procedure,
  Program,
  Sentinel,
  Undefined
}

#[derive(Debug)]
pub struct Identifier {
  name: String,
  category: Category
}

pub struct Parser {
  scanner: Scanner,
  symbol: Symbol,
  stack: Vec<Identifier>,
  // temporary buffer to store identifiers before pushing to stack
  // used to bind the types
  identifiers_buffer: Vec<Identifier>,
  // temporary buffer to store the acceptable types of a category 
  acceptable_categories: Vec<Category>,
  expression : Vec<Symbol>
} 

impl Parser {
  pub fn new() -> Parser {
    Parser { 
      scanner: Scanner::new(), 
      stack: Vec::new(),
      acceptable_categories: Vec::new(),
      identifiers_buffer: Vec::new(), 
      expression : Vec::new(),
      symbol: Symbol { 
        token: Token::Empty, 
        category: Type::Eof, 
        line: 0 } 
      }
  }
  pub fn build_ast(&mut self, p: &str) -> bool {
    self.scanner.build_token(p);
    self.set_next_symbol();
    self.parse_program()
  }

/*
programa →
	program id;
	declarações_variáveis
	declarações_de_subprogramas
	comando_composto
	.
*/

  fn parse_program(&mut self) -> bool {   

    //	program 
    if self.symbol.token == Token::Program {
      
      // pushing
      self.stack.push(
        Identifier {
          name: "$".to_string(), 
          category: Category::Sentinel
        });

      self.set_next_symbol();

      // id
      if self.symbol.category == Type::Identifier {
        
      //TODO: encapsular
      // pushing
      self.stack.push(
        Identifier {
          name: match self.symbol.token {
            Token::LitStr(ref s) => s.to_string(),
            _ => unimplemented!()
          }, 
          category: Category::Program
        });
        
        self.set_next_symbol();

        //;
        if self.symbol.token == Token::Semicolon {
          self.set_next_symbol();
          // declarações_variáveis
          self.parse_declare_var();
          // declarações_de_subprogramas
          self.parse_declare_subprograms();
          //comando_composto
          self.parse_compound_command();
         
         // .
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


/*
declarações_variáveis →
	var lista_declarações_variáveis | ε
*/
  fn parse_declare_var(&mut self) {

    //  var
    if self.symbol.token == Token::Var {
      self.set_next_symbol();
      //  lista_declarações_variáveis
      self.parse_list_declare_var(false); //nao pode ser vazio
    }
  }

/*
lista_declarações_variáveis →
	lista_de_identificadores: tipo; lista_declarações_variáveis'
*/
  fn parse_list_declare_var(&mut self, ep_closure: bool) {
    
    //  lista_de_identificadores
    self.parse_list_identfiers(ep_closure);
  
    // :
    if self.symbol.token == Token::Colon {
      self.set_next_symbol();  
      // tipo
      self.parse_types();

      // ;
      if self.symbol.token == Token::Semicolon {
        self.set_next_symbol();
        // lista_declarações_variáveis'
        self.parse_list_declare_var(true); //pode ser vazio
      }
    }  else if !ep_closure {
      panic!("Expected delimiter `:` found `{}` => line {}", self.symbol.token, self.symbol.line);
    }
  }

/*
  lista_de_identificadores →
	id lista_de_identificadores'
 
*/
  fn parse_list_identfiers(&mut self, ep_closure: bool) {
    // id
    if self.symbol.category == Type::Identifier {
      let name: String = match self.symbol.token {
                Token::LitStr(ref s) => s.to_string(),
                _ => unimplemented!() };
      if !self.search_scope(&name) {
        
        // pushing
        self.identifiers_buffer.push(
          Identifier {
            name: name, 
            category: Category::Undefined
          });
        
        self.set_next_symbol();  
      } else {
        panic!("Identifier `{}` already declared", name);
      }
      // lista_de_identificadores'
      self.parse_list_identfiers_recursive();
    } else if !ep_closure {
      panic!("Expected identifier  found `{:?}` => line {}", self.symbol.category, self.symbol.line);
    }
  }

/*
lista_de_identificadores' →
	, id lista_de_identificadores'
	| ε
*/
  fn parse_list_identfiers_recursive(&mut self) {

    // ,
    if self.symbol.token == Token::Comma {
      self.set_next_symbol();
      // id
      if self.symbol.category == Type::Identifier {
        // pushing
      self.stack.push(
        Identifier {
          name: match self.symbol.token {
            Token::LitStr(ref s) => s.to_string(),
            _ => unimplemented!()
          }, 
          category: Category::Integer
        });
        self.set_next_symbol();
        // lista_de_identificadores'
        self.parse_list_identfiers_recursive();
      } else {
        panic!("Expected identifier  found `{:?}` => line {}", self.symbol.category, self.symbol.line);
      } 
    }
  }
/*
tipo →
	integer | real | boolean
*/
  fn parse_types(&mut self) {

    //integer | real | boolean
    if self.symbol.token == Token::Integer || self.symbol.token == Token::Real || self.symbol.token == Token::Boolean {
      
      self.bind_type_and_erase();

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
        
        //pushing
        self.stack.push(
          Identifier {
            name: match self.symbol.token {
              Token::LitStr(ref s) => s.to_string(),
              _ => unimplemented!()
            }, 
            category: Category::Procedure
          }
        );

        self.stack.push(
          Identifier {
            name: "$".to_string(), 
            category: Category::Sentinel
          }
        );
        

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
        self.clear_scope();
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
      
      let name: String = match self.symbol.token {
                              Token::LitStr(ref s) => s.to_string(),
                              _ => unimplemented!() 
                              };

      let category = self.search_stack(&name); 

      if category != Category::Undefined {
        
        self.acceptable_types(category); //refresh the acceptable_categories vector
        self.set_next_symbol();
        
        if self.symbol.token == Token::Assign {
          self.set_next_symbol();
          self.parse_expr();
          
          self.evaluate_expr();
        } else {
          self.parse_active_procedure();
        }
      } else {
        panic!("Identifier `{}` not declared => line {}", name, self.symbol.line);
      }
    } else if self.symbol.token == Token::Begin {
      self.parse_compound_command();

    } else if self.symbol.token == Token::If {
      self.set_next_symbol();
      self.parse_expr();
      
      self.acceptable_types(Category::Boolean);
      self.evaluate_expr();

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

      self.acceptable_types(Category::Boolean);
      self.evaluate_expr();

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
      self.expression.push(self.symbol.clone());
      self.set_next_symbol();
      self.parse_list_expr();

      if self.symbol.token == Token::RParentheses {
        self.expression.push(self.symbol.clone());
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
      self.expression.push(self.symbol.clone());
      self.set_next_symbol();
      self.parse_expr();
      self.parse_list_expr_recursive();
    }
  }

  fn parse_expr(&mut self) {
    
    self.parse_simple_expr();

    if self.symbol.category == Type::RelOperator {
      self.expression.push(self.symbol.clone());
      self.set_next_symbol(); 
      self.parse_simple_expr();
    }

  }

  fn parse_simple_expr(&mut self) {
    if self.symbol.token == Token::Add || self.symbol.token == Token::Sub {
      self.expression.push(self.symbol.clone());
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
      self.expression.push(self.symbol.clone());
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
      self.expression.push(self.symbol.clone());
      self.set_next_symbol();
      self.parse_factor();
      self.parse_term_recursive();
    }
  }

  fn parse_factor(&mut self) {
    if self.symbol.category == Type::Identifier {
       self.expression.push(self.symbol.clone());
       let name: String = match self.symbol.token {
                              Token::LitStr(ref s) => s.to_string(),
                              _ => unimplemented!() };
                            
      if self.search_stack(&name) != Category::Undefined {
        self.set_next_symbol();
      } else {
        panic!("Identifier `{}` not declared => line {}", name, self.symbol.line);
      }
      self.parse_active_procedure();

    } else if self.symbol.token == Token::LParentheses {
      self.expression.push(self.symbol.clone());
      self.set_next_symbol();
      self.parse_expr();

      if self.symbol.token == Token::RParentheses {
        self.expression.push(self.symbol.clone());
        self.set_next_symbol();
      }
    } else if self.symbol.category == Type::RealLiteral || self.symbol.category == Type::IntLiteral || 
              self.symbol.token == Token::True || self.symbol.token == Token::False ||
              self.symbol.token == Token::Not {
      self.expression.push(self.symbol.clone());
      self.set_next_symbol();
    } else {
      panic!("Expected Factor `id` or `real` or `integer` or `true` or false` or `(` or `not` found `{}` => line {}",
             self.symbol.token, self.symbol.line)
    }   
  }

  fn bind_type_and_erase(&mut self){

    let cat: Category = match self.symbol.token {
                              Token::Integer => Category::Integer,
                              Token::Real => Category::Real,
                              Token::Boolean => Category::Boolean,
                              _ => unimplemented!() };
    
    while !self.identifiers_buffer.is_empty() {
      let mut tmp = self.identifiers_buffer.pop().unwrap();
      tmp.category = cat;
      self.stack.push(tmp);
    }

  }

  fn acceptable_types(&mut self, category: Category){
    self.acceptable_categories = match category {
      Category::Integer => vec![Category::Integer, Category::Real],
      Category::Real => vec![Category::Real, Category::Integer],
      Category::Boolean => vec![Category::Boolean],
      Category::Procedure => vec![Category::Undefined],
      _ => unimplemented!()
    };
  }

  fn search_scope(&self, id: &String) -> bool {
    let len = self.stack.len();

    for x in (0..len).rev() {  
      if self.stack[x].name == "$" {
        return false;
      }
      if self.stack[x].name == *id {
        return true;
      } 
    }
    false
  }

  fn search_stack(&self, id: &String) -> Category {
    let len = self.stack.len();

    for x in (0..len).rev() {
      if self.stack[x].name == *id {
        return self.stack[x].category;
      } 
    }
    Category::Undefined
  }


  fn clear_scope(&mut self) {
    let len = self.stack.len();

    for x in (0..len).rev() {
      if self.stack[x].name == "$" {
        self.stack.pop();
        break;
      } else {
        self.stack.pop();
      }
    }
  }

  fn evaluate_expr(&mut self) {
    // atomic expression
    if self.expression.len() == 1 {
      let syml = self.expression.pop().unwrap();
      let cat = self.match_token_category(syml);
      
      if !self.acceptable_categories.contains(&cat) {
        panic!("Mismatched types expected `{:?}` found `{:?}`", self.acceptable_categories[0], cat);
      }
    } else if self.acceptable_categories[0] == Category::Integer || self.acceptable_categories[0] == Category::Real {
      for e in self.expression.iter() {
        if e.category == Type::RelOperator {
          panic!("Type `{:?}` doesn't support operator relational `{}` => line {}", self.acceptable_categories[0], e.token, e.line);
        } else if e.category == Type::AddOperator || e.category == Type::MulOperator || e.token == Token::Not {
          match e.token {
            Token::Not | Token::And | Token::Or =>  panic!("Type `{:?}` doesn't support operator logical `{}` => line {}", self.acceptable_categories[0], e.token, e.line),
            _ => continue
          }
        } else if e.token == Token::LParentheses || e.token == Token::RParentheses {
          continue;
        } else {
          let cat = self.match_token_category(e.clone());
          if !self.acceptable_categories.contains(&cat) {
            panic!("Mismatched types expected `{:?}` found `{:?}`", self.acceptable_categories[0], cat);
          }
        }   
      }
    } else if self.acceptable_categories[0] == Category::Boolean {
   
      let mut type_stack: Vec<Symbol> = Vec::new();
      let mut op_stack: Vec<Symbol> = Vec::new();
      let mut flag_next = false;
      let mut flag_operation = false;

      for e in self.expression.iter() {

        //(
        if e.token == Token::LParentheses && !op_stack.is_empty(){
          flag_next = true;
        }

        // )
        if e.token == Token::RParentheses {
          if !op_stack.is_empty() {
          
            let mut op1 = type_stack.pop().unwrap();
            let mut op2 = type_stack.pop().unwrap();
            let mut operator = op_stack.pop().unwrap();

            if operator.category == Type::RelOperator {
              type_stack.push(Symbol{
                token : Token::True,
                category : Type::BoolLiteral,
                line : 0
              });
            } else {
              if op1.category == Type::BoolLiteral {
                panic!("Mismatched types expected `{:?}` found `{:?}`", Category::Real, Type::BoolLiteral);
              }
              if op2.category == Type::BoolLiteral {
                panic!("Mismatched types expected `{:?}` found `{:?}`", Category::Real, Type::BoolLiteral);
              }
              type_stack.push(op1.clone());
            }

          }
        }

        //literals
        if e.category == Type::IntLiteral || e.category == Type::RealLiteral || e.category == Type::BoolLiteral{
          
          if flag_next || !flag_operation {
            type_stack.push(e.clone());
          } else {

            let mut op1 = type_stack.pop().unwrap();
            let mut operator = op_stack.pop().unwrap();
            
            if operator.category == Type::RelOperator  {
              
              type_stack.push(Symbol{
                token : Token::True,
                category : Type::BoolLiteral,
                line : 0
              });

            } else {
              if op1.category == Type::BoolLiteral {
                panic!("Mismatched types expected `{:?}` found `{:?}` => line {}", Category::Real, Type::BoolLiteral, op1.line);
              }
              if e.category == Type::BoolLiteral {
                panic!("Mismatched types expected `{:?}` found `{:?}` => line {}", Category::Real, Type::BoolLiteral, e.line);
              }

              type_stack.push(op1.clone());
              flag_operation = false;
            }
          }
        }

        //identifier
        if e.category == Type::Identifier {
          //test before push
          let mut string = match e.token {
            Token::LitStr(ref s) => s.to_string(),
            _ => unimplemented!()
          };

          let mut category = self.search_stack(&string);
          
          match category {
            Category::Procedure | Category::Program | Category::Sentinel | Category::Undefined =>
              panic!("Mismatched types expected `{:?}` found `{:?}` => line {}", self.acceptable_categories[0], category, e.line),
            _=> continue
          };
          
          if flag_next || !flag_operation {
            type_stack.push(e.clone());
          } else {

            let mut op1 = type_stack.pop().unwrap();
            let mut operator = op_stack.pop().unwrap();
            
            if operator.category == Type::RelOperator {
              
              type_stack.push(Symbol{
                token : Token::True,
                category : Type::BoolLiteral,
                line : 0
              });

            } else {
              if op1.category == Type::BoolLiteral {
                panic!("Mismatched types expected `{:?}` found `{:?}` => line {}", Category::Real, Type::BoolLiteral, op1.line);
              }
              if e.category == Type::BoolLiteral {
                panic!("Mismatched types expected `{:?}` found `{:?}` => line {}", Category::Real, Type::BoolLiteral, e.line);
              }

              type_stack.push(op1.clone());
            }
          }
          
        }

        //operator
        if e.category == Type::AddOperator || e.category == Type::MulOperator || e.category == Type::RelOperator{
          op_stack.push(e.clone());
          flag_operation = true;

          if flag_next {
            flag_next = false;
          }
        
        }
      

      } //end of loop
      
      let cat = self.match_token_category(type_stack[0].clone());
      if !self.acceptable_categories.contains(&cat) {
           panic!("Mismatched types expected `{:?}` found `{:?}`", self.acceptable_categories[0], cat);
      } 

    }

    self.expression.clear();
  }

  fn match_token_category(&self, sym: Symbol) -> Category {
    match sym.token {
      Token::LitStr(ref s) => self.search_stack(s),
      Token::True | Token::False => Category::Boolean,
      Token::LitReal(r) => Category::Real,
      Token::LitInt(i) => Category::Integer,
      _ => unimplemented!() 
    }
  } 

  #[inline]
  fn set_next_symbol(&mut self) {
    self.symbol = self.scanner.next_symbol();
  }
}


#[test]
fn test_expr_boolean(){
  let mut p1: Parser = Parser::new();
  let res = p1.build_ast("files/program16.txt");
  for ex in p1.expression.iter() {
    println!("{:?}", ex);
  }
  assert!(res);
}

#[test]
fn test_expr(){
  let mut p1: Parser = Parser::new();
  let res = p1.build_ast("files/program6.txt");
  for ex in p1.expression.iter() {
    println!("{:?}", ex);
  }
  assert!(res);
}


#[test]
fn test_stack(){
  let mut p1: Parser = Parser::new();
  let res = p1.build_ast("files/program6.txt");
  for id in p1.stack.iter() {
    println!("{:?}", id);
  }
  assert!(res);
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
