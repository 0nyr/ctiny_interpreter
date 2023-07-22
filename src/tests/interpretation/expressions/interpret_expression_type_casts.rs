use std::collections::HashMap;
use pest::{Parser, Span};

use crate::semantic::errors::{SemanticError, ASTBuildingError};
use crate::abstract_syntax_tree::nodes::{Value, Identifier, Node};
use crate::interpretation::interpret_expression::interpret_expression;
use crate::symbol_table::structs::{Scope, SymbolTable};
use crate::abstract_syntax_tree::expressions::build_expression;
use crate::syntax_parsing::{CTinyParser, Rule};

pub fn interpret_expression_to_value_for_testing<'a>(
    test_str: &'a str,
) -> Result<Node<'a, Value>, SemanticError> {
    let rule = Rule::expression;

    // Syntax parsing
    let pairs = CTinyParser::parse(rule, test_str).unwrap();

    let first_pair = pairs.into_iter().next().unwrap();
    assert_eq!(first_pair.as_rule(), rule);
    assert_eq!(first_pair.as_str(), test_str);

    // AST conversion
    let expression_node = {
        match build_expression(first_pair) {
            // need to convert the AST Error into a Semantic Error
            Ok(expression_node) => expression_node,
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
    print!("AST for string \"{}\": \n {:#?} \n\n", test_str, expression_node);

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
    interpret_expression(
        &expression_node,
        &symbol_table,
        &main_scope_id_node,
    )
}

#[macro_export]
macro_rules! build_interpret_expression_to_value_test {
    ($test_name:ident, $test_str:expr, $test_value:expr, $literal_type:ident) => {
        // positive test
        #[test]
        fn $test_name() {
            let test_str = $test_str;
            let test_value = $test_value;

            // interpretation
            let interpreted_literal = interpret_expression_to_value_for_testing(
                test_str,
            );

            // check and print
            match &interpreted_literal.unwrap().data {
                Value::$literal_type(literal_value) => {
                    assert_eq!(*literal_value, test_value);
                    print!("Interpreted literal <{}>: {} of type {}\n\n", test_str, *literal_value, stringify!($literal_type));   
                },
                _ => panic!("Expected {} literal.", stringify!($literal_type)),
            }
        }
    };
    ($test_name:ident, $test_str:expr) => {
        // negative test
        #[test]
        fn $test_name() {
            let test_str = $test_str;

            // interpretation
            let interpreted_literal = interpret_expression_to_value_for_testing(
                test_str,
            );

            // check and print
            match interpreted_literal {
                Ok(interpreted_literal) => {
                    panic!(
                        "Expected error, but got interpreted literal <{}>: {:?} of type {}", 
                        test_str, 
                        interpreted_literal.data, 
                        stringify!(interpreted_literal.data.as_type_specifier())
                    );
                },
                Err(error) => {
                    print!(
                        "Expected error occured while interpreting literal <{}>: {}\n\n", 
                        test_str, 
                        error
                    );
                },
            }
        }
    };
}

// type cast tests
// int to something else
build_interpret_expression_to_value_test!(
    test_interpret_expression_type_cast_int_to_int, 
    "(int) 3",
    3,
    Int
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_type_cast_int_to_float, 
    "(float) 3",
    3.0,
    Float
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_type_cast_int_to_char, 
    "(char) 3",
    b'\x03',
    Char
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_type_cast_int_to_bool, 
    "(bool) 3",
    true,
    Bool
);

// floats to something else
build_interpret_expression_to_value_test!(
    test_interpret_expression_type_cast_float_to_float, 
    "(float) 3.14159",
    3.14159,
    Float
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_type_cast_float_to_int, 
    "(int) 3.14159",
    3,
    Int
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_type_cast_float_to_char, 
    "(char) 3.14159",
    b'\x03',
    Char
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_type_cast_float_to_bool, 
    "(bool) 3.14159",
    true,
    Bool
);

// chars to something else
build_interpret_expression_to_value_test!(
    test_interpret_expression_type_cast_char_to_char, 
    "(char) 'a'",
    b'a',
    Char
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_type_cast_char_to_int, 
    "(int) 'a'",
    97,
    Int
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_type_cast_char_to_float, 
    "(float) 'a'",
    97.0,
    Float
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_type_cast_char_to_bool, 
    "(bool) 'a'",
    true,
    Bool
);

// bools to something else
build_interpret_expression_to_value_test!(
    test_interpret_expression_type_cast_bool_to_bool, 
    "(bool) true",
    true,
    Bool
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_type_cast_bool_to_int, 
    "(int) true",
    1,
    Int
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_type_cast_bool_to_float, 
    "(float) true",
    1.0,
    Float
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_type_cast_bool_to_char, 
    "(char) true",
    b'\x01',
    Char
);
