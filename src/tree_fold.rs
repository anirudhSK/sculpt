// A tree fold trait. This trait walks through an immutable tree and updates self.
// Implementations of this trait can override
// methods that process specific types of tree nodes, while using default
// methods for other types of tree nodes.

use grammar::*;

pub trait TreeFold<'a> {
  fn visit_prog(&mut self, tree : &'a Prog) {
    self.visit_snippets(&tree.snippets);
    self.visit_connections(&tree.connections);
  }

  fn visit_snippets(&mut self, tree : &'a Snippets) {
    for snippet in &tree.snippet_vector { self.visit_snippet(snippet); }
  }
 
  fn visit_snippet(&mut self, tree : &'a Snippet) {
    self.visit_identifier(&tree.snippet_id);
    self.visit_variable_decls(&tree.variable_decls);
    self.visit_statements(&tree.statements);
  }

  fn visit_connections(&mut self, tree : &'a Connections) {
    for connection in &tree.connection_vector {
      self.visit_connection(&connection);
    }
  }

  fn visit_connection(&mut self, tree : &'a Connection) {
    self.visit_identifier(&tree.from_snippet);
    self.visit_identifier(&tree.to_snippet);
  }

  fn visit_variable_decls(&mut self, tree : &'a VariableDecls) {
    for init in &tree.decl_vector { self.visit_variable_decl(init); }
  }

  fn visit_variable_decl(&mut self, tree : &'a VariableDecl) {
    self.visit_identifier(&tree.identifier);
    for value in &(tree.initial_values) { self.visit_value(value); };
    self.visit_var_type(&tree.var_type);
  }

  fn visit_var_type(&mut self, tree : &'a VarType) {
    let _ = tree;
    let _ = self;
    // Do nothing here.    
  }

  fn visit_statements(&mut self, tree : &'a Statements) {
    for stmt in &tree.stmt_vector { self.visit_statement(stmt); }
  }
  
  fn visit_statement(&mut self, tree : &'a Statement) {
    self.visit_lvalue(&tree.lvalue);
    self.visit_expr(&tree.expr);
  }
  
  fn visit_expr(&mut self, tree : &'a Expr) {
    self.visit_operand(&tree.op1);
    self.visit_expr_right(&tree.expr_right);
  }
  
  fn visit_expr_right(&mut self, tree : &'a ExprRight) {
    match tree {
      &ExprRight::BinOp(_, ref operand) => self.visit_operand(operand),
      &ExprRight::Cond(ref operand_true, ref operand_false) => {
        self.visit_operand(operand_true);
        self.visit_operand(operand_false);
      },
      &ExprRight::Empty() => ()
    }
  }
  
  fn visit_operand(&mut self, tree : &'a Operand) {
    match tree {
      &Operand::LValue(ref lvalue) => self.visit_lvalue(lvalue),
      &Operand::Value(ref value)   => self.visit_value(value)
    }
  }

  fn visit_lvalue(&mut self, tree : &'a LValue) {
    match tree {
      &LValue::Identifier(ref identifier) => self.visit_identifier(identifier),
      &LValue::Array(ref array, ref operand) => {
        self.visit_identifier(array);
        self.visit_operand(operand);
      }
    }
  }
 
  // The awkward let _ is required to suppress the unused variables warning
  // https://github.com/rust-lang/rust/issues/26487
  fn visit_identifier(&mut self, tree : &'a Identifier) { let _ = tree; let _ = self; }
  
  fn visit_value(&mut self, tree : &'a Value) { let _ = tree; let _ = self; }
}
