use pest::Parser;

use crate::syntax_parsing::parser::{CTinyParser, Rule};
use crate::syntax_tree::expressions::build_expression;

macro_rules! build_test {
    ($rule:expr, $( $input_str:literal),* ) => {

        let input_strs = {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($input_str);
            )*
            temp_vec
        };

        for test_str in input_strs {
            // Syntax parsing
            let pairs = CTinyParser::parse($rule, test_str).unwrap();

            let first_pair = pairs.into_iter().next().unwrap();
            assert_eq!(first_pair.as_rule(), $rule);
            assert_eq!(first_pair.as_str(), test_str);

            // AST conversion
            // WARN: don't forget to change the method if needed
            let ast = build_expression(first_pair)
                .unwrap_or_else(|error| { 
                    print!("AST ERROR for {}: \n {}\n", test_str, error); 
                    panic!(); 
                });
            print!("AST for string \"{}\": \n {:#?} \n\n", test_str, ast);
        }
    };
}

#[test]
fn test_ast_literal() {
    build_test!(Rule::literal, "1024", "'a'", "true", "false", "3.14159", "0.001");
}

#[test]
fn test_ast_type_cast() {
    // we are not at the intepreter stage yet
    // so we don't need to check for type errors
    build_test!(Rule::type_cast, 
        "(int) 3.14159", 
        "(float) 1024", 
        "(int) 'a'", 
        "(bool) true", 
        "(bool) false",
        "(int) true",
        "(float) true",
        "(char) false",
        "(bool) 1024",
        "(float) 2.71828",
        "(char) 1024"
    );
}

#[test]
fn test_ast_function_call() {
    build_test!(Rule::function_call, 
        "foo()",
        "foo(1)",
        "foo(1, 2, 3)", 
        "foo(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)",
        // TODO: add more complex expressions as arguments, once we have them
        "foo(1 + 2, (3 * 4), bar(true))"
    );
}

#[test]
fn test_as_get_value() {
    build_test!(Rule::get_value,
        "a",
        "a[0]"
    );
}

#[test]
fn test_ast_parenthesized_expression() {
    build_test!(Rule::parenthesized_expression,
        "(1 + 2)",
        "(1.001 + (float) 2)",
        "((1 + 2) * foo(3))",
        "(((1 + 2) * 3) / 4)"
    );
}

#[test]
fn test_ast_primary() {
    build_test!(Rule::primary,
        "1",
        "1.001",
        "'a'",
        "true",
        "false",
        "foo()",
        "foo(1)",
        "foo(1, 2, 3)", 
        "foo(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)",
        "a",
        "a[0]",
        "(1 + 2)",
        "(1.001 + (float) 2)",
        "((1 + 2) * 3)",
        "((1 + 2) * foo(3))",
        "(((1 + 2) * 3) / 4)"
    );
}

#[test]
fn test_ast_factor() {
    build_test!(Rule::factor,
        "-1",
        "-1.001",
        "!true",
        "!false",
        "!(1 == 2)",
        "-foo()"
    );
}

#[test]
fn test_ast_term() {
    build_test!(Rule::term,
        "1 * (-2)"
    );
}

#[test]
fn test_ast_disjunction() {
    build_test!(Rule::disjunction,
        "1 || 2", 
        "1 || 2 || 3", 
        "(1 && 2) || 3"
    );
}

#[test]
fn test_ast_conjunction() {
    build_test!(Rule::conjunction, 
        "1 && 2", 
        "1 && 2 && 3", 
        "1 && (2 || 3)"
    );
}

#[test]
fn test_ast_equality() {
    build_test!(Rule::equality, 
        "1 == 2", 
        "1 != 2", 
        "1 == 2 != 3"
    );
}

#[test]
fn test_ast_relation() {
    build_test!(Rule::relation,
        "1",
        "(1 < 2)", 
        "1 < 2", 
        "1 > 2", 
        "1 <= 2", 
        "1 >= 2",
        "(1 <= 2)"
    );
}

#[test]
fn test_ast_addition() {
    build_test!(Rule::addition,
        "1 + 2", 
        "1 - 2", 
        "1 + 2 - 3"
    );
}

#[test]
fn test_ast_expression() {
    build_test!(Rule::expression, 
        "1 && 2 || 3", 
        "(1 && 2) || 3", 
        "1 && (2 || 3)",
        "1 == 2 && 3 < 4",
        "1 == 2 && 3 < 4 || 5 >= 6",
        "1 == 2 && 3 < foo(7 <= 8) || 5 >= 6 && 7 <= 8"
    );
}


// #[test]
// fn test_ast_relation_comp_or_eq() {
//         let test_string = "(1 <= 2)";

//         // pair parsing
//         let pairs = CTinyParser::parse(Rule::expression, test_string)
//             .unwrap();

//         // print all pairs
//         let nb_pairs = pairs.clone().count();
//         print!("nb pairs: {}\n", nb_pairs);
//         for pair in pairs.clone().into_iter() {
//             print!("pair {:?}: {}\n", pair.as_rule(), pair.as_str());
//         }

//         let first_pair = pairs.into_iter().next().unwrap();
//         assert_eq!(first_pair.as_rule(), Rule::relation);
//         assert_eq!(first_pair.as_str(), test_string);

//         // AST conversion
//         let ast = build_expression(first_pair)
//         .unwrap_or_else(|error| { print!("{}\n", error); panic!(); });
//         print!("{:#?}", ast);
// }