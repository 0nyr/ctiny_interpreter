use pest::Parser;

use crate::abstract_syntax_tree::declarations::build_multi_declaration;
use crate::semantic::errors::{SemanticError, ASTBuildingError, SyntaxParsingError};
use crate::syntax_parsing::{CTinyParser, Rule};
use crate::interpretation::interpret_declaration::interpret_declaration;
use crate::tests::interpretation::create_symbol_table_and_empty_main_scope;

pub fn interpret_statement_to_value_for_testing<'a>(
    test_str: &'a str,
) -> Result<(), SemanticError> {
    let rule = Rule::multi_declaration;

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
    let declaration_nodes = {
        match build_multi_declaration(first_pair) {
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
    print!("AST for string \"{}\": \n {:#?} \n\n", test_str, declaration_nodes);

    // for the need of the test, build a symbol table from scratch with one scope "main"
    let (mut symbol_table, main_scope_id_node) = create_symbol_table_and_empty_main_scope(
        test_str,
    );

    // interpretation
    for declaration in declaration_nodes {
        interpret_declaration(
            &declaration,
            &mut symbol_table,
            &main_scope_id_node,
        )?;
    }

    Ok(())
}

macro_rules! build_interpret_declaration_test {
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
                    print!("Successfully interpreted multi-declaration <{}>.\n\n", test_str); 
                },
                Err(error) => {
                    panic!(
                        "Error interpreting multi-declaration <{}>: {}\n\n", 
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
                        "Expected error, but got Ok multi-declaration instead, for multi-declaration <{}>.", 
                        test_str,
                    );
                },
                Err(error) => {
                    panic!(
                        "Expected error occured while interpreting multi-declaration <{}>: {}\n\n", 
                        test_str, 
                        error
                    );
                },
            }
        }
    };
}

build_interpret_declaration_test!(
    interpret_declaration_normal_var_error_assign_not_allowed,
    "int x = 5;",
    false
);

build_interpret_declaration_test!(
    interpret_declaration_normal_var_no_value,
    "int x;"
);

build_interpret_declaration_test!(
    interpret_declaration_normal_var_multiple,
    "int x, y, z;"
);

build_interpret_declaration_test!(
    interpret_declaration_array_var_no_value,
    "int x[5];"
);

build_interpret_declaration_test!(
    interpret_declaration_array_var_multiple,
    "int x[5], y[5], z[5];"
);

build_interpret_declaration_test!(
    interpret_declaration_normal_and_array_var_multiple,
    "int x, y[5], z;"
);

build_interpret_declaration_test!(
    interpret_declaration_redeclaration_error,
    "int x, y, x;",
    false
);

build_interpret_declaration_test!(
    interpret_declaration_redeclaration_error_array,
    "int x[5], y[3], x[10];",
    false
);

build_interpret_declaration_test!(
    interpret_declaration_redeclaration_mixed_error,
    "int x, y, x[5], y[3];",
    false
);