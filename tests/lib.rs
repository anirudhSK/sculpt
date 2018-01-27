extern crate sculpt;

use sculpt::lexer;
use sculpt::parser;
use sculpt::symbol_table_pass::SymbolTablePass;
use sculpt::define_before_use_pass::DefineBeforeUsePass;
use sculpt::parser_impl::Parsing;
use sculpt::tree_fold::TreeFold;
use std::collections::HashSet;

fn run_def_use(input_program : &str) {
  // Lexing
  let tokens = & mut lexer::get_tokens(input_program);

  // parsing
  let parse_tree = parser::Prog::parse(tokens);
  assert!(tokens.is_empty(), "Tokens is not empty.");
  println!("Parse tree: {:?}\n", parse_tree);

  // symbol table generation
  let mut symbol_table = HashSet::new();
  SymbolTablePass::visit_prog(&parse_tree, &mut symbol_table);
  println!("Symbol table: {:?}",symbol_table);

  // Check that identifiers are defined before use
  let mut definitions = HashSet::new();
  DefineBeforeUsePass::visit_prog(&parse_tree, &mut definitions);
}

#[test]
#[should_panic(expected="y used before definition")]
fn test_def_before_use_undefined(){
  let input_program = r"snippet fun(x, ) {
                          b = y;
                          m = 5;
                        }
                        ";
  run_def_use(input_program);
}

#[test]
fn test_def_before_use_defined(){
  let input_program = r"snippet fun(a, b, c, x, y, ) {
                          static x = 0;
                        }
                        snippet foo(a, b, c, ) {
                          x = a;
                        }
                        (foo, fun)
                        ";
  run_def_use(input_program);
}