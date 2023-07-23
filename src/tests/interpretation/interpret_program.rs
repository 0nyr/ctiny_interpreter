use pest::Parser;

use crate::interpretation::interpret_function::interpret_translation_unit;
use crate::semantic::errors::{SemanticError, SyntaxParsingError, ASTBuildingError}; 
use crate::abstract_syntax_tree::nodes::{Node, Value};
use crate::symbol_table::build_static_symbol_table;
use crate::syntax_parsing::{Rule, CTinyParser};
use crate::abstract_syntax_tree::build_translation_unit;


fn interpret_program_to_value_for_testing<'a>(
    test_str: &'a str,
) -> Result<Node<'a, Value>, SemanticError> {
    let rule = Rule::translation_unit;

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
    let ast = {
        match build_translation_unit(first_pair) {
            // need to convert the AST Error into a Semantic Error
            Ok(translation_unit) => translation_unit,
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
    print!("AST for string \"{}\": \n {:#?} \n\n", test_str, ast);

    // build symbol table
    let mut symbol_table = build_static_symbol_table(&ast);

    // interpretation
    interpret_translation_unit(
        &ast,
        &mut symbol_table,
    )
}

macro_rules! build_translation_unit_test {
    ($test_name:ident, $test_str:expr, $test_value:expr) => {
        // positive test
        #[test]
        fn $test_name() {
            let test_str = $test_str;

            // interpretation
            let interpreted_literal = interpret_program_to_value_for_testing(
                test_str,
            );

            // check and print
            match &interpreted_literal {
                Ok(interpreted_value_node) => {
                    assert_eq!(interpreted_value_node.data, $test_value);
                    print!("Successfully interpreted program <{}>.\n\n", test_str); 
                },
                Err(error) => {
                    panic!(
                        "Error interpreting program <{}>: {}\n\n", 
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
            let interpreted_literal = interpret_program_to_value_for_testing(
                test_str,
            );

            // check and print
            match interpreted_literal {
                Ok(interpreted_literal) => {
                    print!(
                        "Expected error, but for program <{}>, got program return value <{}> instead.", 
                        test_str,
                        interpreted_literal
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

build_translation_unit_test!(
    interpret_empty_basic_program,
    "int main() { return 0; }",
    Value::Int(0)
);

build_translation_unit_test!(
    interpret_basic_program,
    "int main() { return 42; }",
    Value::Int(42)
);

// build_translation_unit_test!(
//     interpret_basic_program_with_declaration_and_assignment,
//     "int main() { int a; a = 42; return a; }",
//     Value::Int(42)
// );

// TODO: fix this test
#[test]
fn test_interpret_basic_program_with_declaration_and_assignment() {
    let test_str = "int main() { int a; a = 42; return a; }";
    let rule = Rule::translation_unit;

    // Syntax parsing
    let pairs = match CTinyParser::parse(rule, test_str) {
        Ok(pairs) => pairs,
        Err(error) => {
            panic!("Syntax parsing error for {}: \n {}\n", test_str, error);
        },
    };

    let first_pair = pairs.into_iter().next().unwrap();
    assert_eq!(first_pair.as_rule(), rule);
    assert_eq!(first_pair.as_str(), test_str);

    // AST conversion
    let ast = {
        match build_translation_unit(first_pair) {
            // need to convert the AST Error into a Semantic Error
            Ok(translation_unit) => translation_unit,
            Err(error) => {
                panic!("AST ERROR for {}: \n {}\n", test_str, error);
            },
        }
    };
    print!("AST for string \"{}\": \n {:#?} \n\n", test_str, ast);

    // build symbol table
    let mut symbol_table = build_static_symbol_table(&ast);

    // interpretation
    let interpreted_value = interpret_translation_unit(
        &ast,
        &mut symbol_table,
    );
    match interpreted_value {
        Ok(interpreted_value_node) => {
            assert_eq!(interpreted_value_node.data, Value::Int(42));
            print!("Successfully interpreted program <{}>.\n\n", test_str); 
        },
        Err(error) => {
            panic!(
                "Error interpreting program <{}>: {}\n\n", 
                test_str, 
                error
            );
        },
    }
}