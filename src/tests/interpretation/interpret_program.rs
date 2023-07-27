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
    ($test_name:ident, $test_str:expr) => {
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
                        "Expected error, but for program <{}>, got program return value <{:?}> instead.", 
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

build_translation_unit_test!(
    interpret_basic_program_with_declaration_and_assignment,
    "int main() { int a; a = 42; return a; }",
    Value::Int(42)
);

build_translation_unit_test!(
    interpret_basic_program_with_declaration_assignment_if,
    "int main() { int a; a = 42; if (a + 100 > 'a') { a = 'a'; } return a; }",
    Value::Int(97)
);

build_translation_unit_test!(
    interpret_basic_program_with_declaration_assignment_while,
    "int main() { int a; a = 42; while (a > 0) { a = a - 1; } return a; }",
    Value::Int(0)
);

build_translation_unit_test!(
    interpret_basic_program_with_declaration_assignment_if_while,
    "int main() { int a; int y; a = 0; y = 10; while (y > 0) { if (a < 100) { a = a + y; } y = y - 1; } return a; }",
    Value::Int(55)
);

build_translation_unit_test!(
    interpret_multi_function_program,
    "
    int foo(int a) { 
        a = a + 1;
        return a; 
    }
    int main() { 
        int a;
        a = 42;
        a = foo(a);
        return a; 
    }",
    Value::Int(43)
);

build_translation_unit_test!(
    interpret_basic_program_with_declaration_assignment_if_while_nested,
    "
    int foo(int a) { 
        while (a > 0) { 
            a = a - 1; 
        } 
        return a; 
    }
    int main() { 
        int a;
        int y;
        a = foo(42);
        y = 10; while (y > 0) { 
            if (a < 100) { a = a + y; } 
            y = y - 1; 
        } 
        return a; 
    }",
    Value::Int(55)
);

// tests that expect a panic
build_translation_unit_test!(
    interpret_basic_program_with_declaration_after_assignment,
    "int main() { 
        int a; 
        int y; // not allowed after assignments
        return a;
    }"
);

build_translation_unit_test!(
    interpret_basic_program_with_declaration_assignment_if_without_return,
    "int main() { 
        int a; a = 42; 
        if (a + 100 > 'a') { a = 'a'; } 
        // missing return
    }"
);

build_translation_unit_test!(
    interpret_basic_program_unused_variable,
    "int main() { 
        int a; // a is unused
        return 0; 
    }"
);

build_translation_unit_test!(
    test_semantic_undeclared_function,
    "int main () {
        int x;
        x = foo(); // foo is not declared
    }"
);

build_translation_unit_test!(
    test_semantic_undeclared_variable_in_function,
    "int foo () {
        int x;
        x = y; // y is not declared
        return x;
    }
    int main () {
        int x;
        x = foo();
        return x;
    }"
);

build_translation_unit_test!(
    test_semantic_missing_function_argument,
    "int foo (int x) {
        return x;
    }
    int main () {
        int x;
        x = foo(); // foo is missing argument
        return x;
    }"
);

build_translation_unit_test!(
    test_semantic_incorrect_function_argument_provided,
    "int foo (int x) {
        x = x + 1;
        return x;
    }
    int main () {
        int x;
        x = 10;
        x = foo(); // foo is missing argument
        return x;
    }"
);

build_translation_unit_test!(
    test_semantic_incorrect_function_array_argument_provided,
    "int foo (int x[10]) { // array argument not allowed
        x = x + 1;
        return x;
    }
    int main () {
        int x[10];
        int i;
        i = 0;
        while (i < 10) {
            x[i] = i;
            i = i + 1;
        }
        x = foo(x); 
        return x;
    }"
);

build_translation_unit_test!(
    test_semantic_incorrect_function_too_many_argument_provided,
    "int foo (int x, char y, int z) {
        x = x + 1;
        return x;
    }
    int main () {
        int x;
        x = 10;
        x = foo(1, 'a', 3, 4); // foo has too many arguments
        return x;
    }"
);