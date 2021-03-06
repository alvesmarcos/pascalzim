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
  types_stack : Vec<Category>
} 

impl Parser {
  pub fn new() -> Parser {
    Parser { 
      scanner: Scanner::new(), 
      stack: Vec::new(),
      acceptable_categories: Vec::new(),
      identifiers_buffer: Vec::new(), 
      types_stack : Vec::new(),
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
        panic!("Identifier `{}` already declared => line `{}`", name, self.symbol.line);
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
        let name: String = match self.symbol.token {
                Token::LitStr(ref s) => s.to_string(),
                _ => unimplemented!() };

        if !self.search_scope(&name) {
          // pushing
          self.identifiers_buffer.push(
            Identifier {
              name: match self.symbol.token {
                Token::LitStr(ref s) => s.to_string(),
                _ => unimplemented!()
              }, 
              category: Category::Integer
            });
          self.set_next_symbol();
        } else {
          panic!("Identifier `{}` already declared => line `{}`", name, self.symbol.line);
        }
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
          let mut saved_line = self.symbol.line;
          self.set_next_symbol();
          self.parse_expr();
          let mut exp_result : Category = self.types_stack.pop().unwrap(); 
          if !self.acceptable_categories.contains(&exp_result){

              panic!("Mismatched types expected `{:?}` found `{:?}` => line {}", self.acceptable_categories[0], exp_result, saved_line);
          }
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
      let mut saved_operator = self.symbol.clone();
      self.set_next_symbol(); 
      self.parse_simple_expr();
      let mut op1 = self.types_stack.pop().unwrap();
      let mut op2 = self.types_stack.pop().unwrap();

      if saved_operator.token == Token::Equal || saved_operator.token == Token::NotEqual {
        
        if op1 != op2 {
            panic!("Mismatched types `{:?}` is different from `{:?}` for `{}` => line {}", op1, op2, saved_operator.token, saved_operator.line);    
        }

      } else if saved_operator.token == Token::Imp { 

        if op1 != Category::Boolean || op2 != Category::Boolean {
          panic!("Logic operator `{}` only supports Boolean operands => line {}", saved_operator.token, saved_operator.line);
        }

      } else {

        if op1 != Category::Real && op1 != Category::Integer {  
          panic!("Type `{:?}` doesn't support relational operator `{}` => line {}", op1, saved_operator.token, saved_operator.line)
        }

        if op2 != Category::Real && op2 != Category::Integer {  
          panic!("Type `{:?}` doesn't support relational operator `{}` => line {}", op2, saved_operator.token, saved_operator.line)
        }


      }

      self.types_stack.push(Category::Boolean);

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
      let mut saved_operator = self.symbol.clone();
      self.set_next_symbol();
      self.parse_term();

      let mut op1 = self.types_stack.pop().unwrap();
      let mut op2 = self.types_stack.pop().unwrap();

      if saved_operator.token == Token::Add || saved_operator.token == Token::Sub {
        if op1 == Category::Integer && op2 == Category::Integer {
          self.types_stack.push(Category::Integer);
        } else if op1 == Category::Boolean || op2 == Category::Boolean {
          panic!("Type `{:?}` doesn't support arithmetic operator `{}` => line {}", Category::Boolean, saved_operator.token, saved_operator.line);
        } else {
          self.types_stack.push(Category::Real);
        }
      } else {
        //or
        if op1 != Category::Boolean || op2 != Category::Boolean {
          panic!("Logic operator `{}` only supports Boolean operands => line {}", saved_operator.token, saved_operator.line);
        }
          self.types_stack.push(Category::Boolean);
      }

      self.parse_simple_expr_recursive();
    }
  }

  fn parse_term(&mut self) {
    self.parse_factor();
    self.parse_term_recursive();
  }

  fn parse_term_recursive(&mut self) {
    if self.symbol.category == Type::MulOperator {
      let mut saved_operator = self.symbol.clone();

      self.set_next_symbol();
      self.parse_factor();

      let mut op1 = self.types_stack.pop().unwrap();
      let mut op2 = self.types_stack.pop().unwrap();

      if saved_operator.token == Token::Mult || saved_operator.token == Token::Div {
        if op1 == Category::Integer && op2 == Category::Integer {
          self.types_stack.push(Category::Integer);
        } else if op1 == Category::Boolean || op2 == Category::Boolean {
          panic!("Type `{:?}` doesn't support arithmetic operator `{}` => line {}", Category::Boolean, saved_operator.token, saved_operator.line);
        } else {
          self.types_stack.push(Category::Real);
        }
      } else {
        //and
        if op1 != Category::Boolean || op2 != Category::Boolean {
          panic!("Logic operator `{}` only supports Boolean operands => line {}", saved_operator.token, saved_operator.line);
        }
          self.types_stack.push(Category::Boolean);
      }

      self.parse_term_recursive();
    }
  }

  fn parse_factor(&mut self) {
    if self.symbol.category == Type::Identifier {
       let name: String = match self.symbol.token {
                              Token::LitStr(ref s) => s.to_string(),
                              _ => unimplemented!() };


      if self.search_stack(&name) != Category::Undefined {
        //coloca o tipo na pilha
        let mut cat = self.match_token_category(self.symbol.clone());
        self.types_stack.push(cat);
        self.set_next_symbol();
      } else {
        panic!("Identifier `{}` not declared => line {}", name, self.symbol.line);
      }

      

      self.parse_active_procedure();

    } else if self.symbol.token == Token::LParentheses {
      self.set_next_symbol();
      self.parse_expr();

      if self.symbol.token == Token::RParentheses {
        self.set_next_symbol();
      }
    } else if self.symbol.category == Type::RealLiteral || self.symbol.category == Type::IntLiteral || 
              self.symbol.token == Token::True || self.symbol.token == Token::False {
      //coloca o tipo na pilha
      let mut cat = self.match_token_category(self.symbol.clone());
      self.types_stack.push(cat);    
      
      self.set_next_symbol();
    } else if self.symbol.token == Token::Not {
      self.set_next_symbol();
      self.parse_factor();
    } else{
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
      Category::Integer => vec![Category::Integer],
      Category::Real => vec![Category::Real, Category::Integer],
      Category::Boolean => vec![Category::Boolean],
      _ => vec![Category::Undefined]
    };
  }

  fn search_scope(&self, id: &String) -> bool {
    let len = self.stack.len();
   
    if self.is_program_or_procedure(id) {
      panic!("You can't define variables with name of the program or procedure `{}`=> line {}", self.symbol.token, self.symbol.line);
    }

    for e in self.identifiers_buffer.iter() {
      if e.name == *id { return true; }
    }
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

  fn is_program_or_procedure(&self, id: &String) -> bool {  
    for e in self.identifiers_buffer.iter() {
      if e.name == *id && (e.category == Category::Program || e.category == Category::Procedure) {
        return true;
      }
    }
    for e in self.stack.iter() {
      if e.name == *id  && (e.category == Category::Program || e.category == Category::Procedure) {
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
/*
  fn evaluate_expr(&mut self) {
    // atomic expression
    if self.expression.len() == 1 {
      let syml = self.expression.pop().unwrap();
      let cat = self.match_token_category(syml.clone());
      
      if !self.acceptable_categories.contains(&cat) {
        panic!("Mismatched types expected `{:?}` found `{:?}` => line {}", self.acceptable_categories[0], cat, syml.line);
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
            panic!("Mismatched types expected `{:?}` found `{:?}` => line {}", self.acceptable_categories[0], cat, e.line);
          }
        }   
      }
    } else if self.acceptable_categories[0] == Category::Boolean {
      let mut pct: Vec<Category> = Vec::new();
      let mut as1: Category = Category::Undefined;

      while !self.expression.is_empty() {
        let mut op = self.expression.pop().unwrap();
        
        if op.category == Type::RelOperator {
          if let Some(e) = self.expression.pop() {
            let mut res = self.match_token_category(e.clone());
            self.acceptable_types(as1);
            if !self.acceptable_categories.contains(&res) {
              panic!("Mismatched types expected `{:?}` found `{:?}` => line {}", self.acceptable_categories[0], res, op.line);
            }
            as1 = match op.token {
              Token::Equal | Token::NotEqual => Category::Undefined,
              _ => {
                if as1 == Category::Boolean || res == Category::Boolean {
                   panic!("Type `{:?}` doesn't support operator relational `{}` => line {}", as1, op.token, op.line)
                } else {
                  Category::Undefined
                }
              } 
            };
            pct.push(Category::Boolean);
          } else {
             panic!("Operator relational is binary => line {}", self.symbol.line);
          }
        } else if op.category == Type::BoolLiteral || op.category == Type::Identifier || op.category == Type::IntLiteral || op.category == Type::RealLiteral {
          as1 = self.match_token_category(op);
        }
      }
    }
    self.expression.clear();
  }
*/
  fn match_token_category(&self, sym: Symbol) -> Category {
    match sym.token {
      Token::LitStr(ref s) => self.search_stack(s),
      Token::True | Token::False => Category::Boolean,
      Token::LitReal(_) => Category::Real,
      Token::LitInt(_) => Category::Integer,
      _ => Category::Undefined 
    }
  } 

  #[inline]
  fn set_next_symbol(&mut self) {
    self.symbol = self.scanner.next_symbol();
  }
}



#[test]
fn test_program_final(){
  let mut p1: Parser = Parser::new();
  let res = p1.build_ast("files/programFinal.txt");
  for id in p1.stack.iter() {
    println!("{:?}", id);
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

//#[test]
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

//#[test]
fn test_parser_program12() {
  let mut p1: Parser = Parser::new();
  let res = p1.build_ast("files/program12.txt");

  assert!(res);
}

//#[test]
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
