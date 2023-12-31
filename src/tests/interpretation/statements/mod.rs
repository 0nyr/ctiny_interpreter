use pest::Parser;

use crate::abstract_syntax_tree::statements::build_statement;
use crate::interpretation::interpret_statement::interpret_statement;
use crate::semantic::errors::{SemanticError, ASTBuildingError, SyntaxParsingError};
use crate::syntax_parsing::{CTinyParser, Rule};
use crate::tests::interpretation::{create_symbol_table_and_empty_main_scope, create_pseudo_translation_unit};

#[cfg(test)]
mod interpret_statement_assignment_normal_value;
#[cfg(test)]
mod interpret_statement_assignment_array_value;
#[cfg(test)]
mod interpret_statement_if_else;
#[cfg(test)]
mod interpret_statement_while_loop;

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
    let (mut symbol_table, main_scope_id_node) = create_symbol_table_and_empty_main_scope(
        test_str,
    );
    let pseudo_translation_unit = create_pseudo_translation_unit();

    // interpretation
    interpret_statement(
        &statement_node,
        &mut symbol_table,
        &main_scope_id_node,
        &pseudo_translation_unit,
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
                Ok(_) => {
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
