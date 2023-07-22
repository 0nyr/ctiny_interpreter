use std::collections::HashMap;
use pest::{Parser, Span};

use crate::abstract_syntax_tree::statements::build_statement;
use crate::interpretation::interpret_statement::interpret_statement;
use crate::semantic::errors::{SemanticError, ASTBuildingError, SyntaxParsingError};
use crate::abstract_syntax_tree::nodes::{Identifier, Node};
use crate::symbol_table::structs::{Scope, SymbolTable};
use crate::syntax_parsing::{CTinyParser, Rule};

#[cfg(test)]
mod interpret_statement_assignment_normal_value;
#[cfg(test)]
mod interpret_statement_assignment_array_value;
#[cfg(test)]
mod interpret_statement_if_else;

pub fn interpret_statement_to_value_for_testing<'a>(
    test_str: &'a str,
) -> Result<(), SemanticError> {
    let rule = Rule::statement;

    // Syntax parsing
    let pairs = match CTinyParser::parse(rule, test_str) {
        Ok(pairs) => pairs,
        Err(error) => {
            print!("Syntax parsing error for {}: \n {}\n", test_str, error);
            return Err(
                SemanticError::SyntaxParsing(
                    SyntaxParsingError::from(error)
                )
            );
        },
    };

    let first_pair = pairs.into_iter().next().unwrap();
    assert_eq!(first_pair.as_rule(), rule);
    assert_eq!(first_pair.as_str(), test_str);

    // AST conversion
    let statement_node = {
        match build_statement(first_pair) {
            // need to convert the AST Error into a Semantic Error
            Ok(statement_node) => statement_node,
            Err(error) => {
                print!("AST ERROR for {}: \n {}\n", test_str, error);
                return Err(
                    SemanticError::ASTBuilding(
                        ASTBuildingError::from(error)
                    )
                );
            },
        }
    };
    print!("AST for string \"{}\": \n {:#?} \n\n", test_str, statement_node);

    // for the need of the test, build a symbol table from scratch with one scope "main"
    let mut symbol_table = SymbolTable::new();
    let main_scope_id_node = Node {
        sp: Span::new(&test_str, 0, test_str.len()).unwrap(),
        data: Identifier {name: "main".to_string()},
    };
    let main_scope = Scope::new(
        main_scope_id_node.data.clone(),
        HashMap::new(),
    );
    symbol_table.add_scope(main_scope);

    // interpretation
    interpret_statement(
        &statement_node,
        &mut symbol_table,
        &main_scope_id_node,
    )
}

#[macro_export]
macro_rules! build_interpret_statement_to_value_test {
    ($test_name:ident, $test_str:expr) => {
        // positive test
        #[test]
        fn $test_name() {
            let test_str = $test_str;

            // interpretation
            let interpreted_literal = interpret_statement_to_value_for_testing(
                test_str,
            );

            // check and print
            match &interpreted_literal {
                Ok(_) => {
                    print!("Successfully interpreted statement <{}>.\n\n", test_str); 
                },
                Err(error) => {
                    panic!(
                        "Error interpreting statement <{}>: {}\n\n", 
                        test_str, 
                        error
                    );
                },
            }
        }
    };
    ($test_name:ident, $test_str:expr, false) => {
        // negative test
        #[test]
        #[should_panic]
        fn $test_name() {
            let test_str = $test_str;

            // interpretation
            let interpreted_literal = interpret_statement_to_value_for_testing(
                test_str,
            );

            // check and print
            match interpreted_literal {
                Ok(interpreted_literal) => {
                    print!(
                        "Expected error, but got Ok stament instead, for statement <{}>.", 
                        test_str,
                    );
                },
                Err(error) => {
                    panic!(
                        "Expected error occured while interpreting statement <{}>: {}\n\n", 
                        test_str, 
                        error
                    );
                },
            }
        }
    };
}
