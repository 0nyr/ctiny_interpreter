use pest::Parser;

use crate::syntax_parsing::parser::{CTinyParser, Rule};
use crate::syntax_tree::statements::{build_statement, multi_statement_vector_from_pair};

use crate::build_test;

macro_rules! build_test_statement {
    ($rule:expr, $( $input_str:literal),* ) => {
        build_test!($rule, build_statement, $( $input_str),* );
    }
}

#[test]
fn test_ast_return() {
    build_test_statement!(Rule::return_statement,
        "return 1",
        "return 1 + 2",
        "return 1 + 2 * 3",
        "return 1 + foo(true) * 3 - 4"
    ); 
    // WARN: forgetting the semicolon will cause an error due 
    // to the fact that the function will try to return the last expression...
}

#[test]
fn test_ast_break() {
    build_test_statement!(Rule::break_statement,
        "break"
    );
}

#[test]
fn test_ast_continue() {
    build_test_statement!(Rule::continue_statement,
        "continue"
    );
}

#[test]
fn test_ast_assignment() {
    build_test_statement!(Rule::assignment_statement,
        "a = 1",
        "a = 1 + 2",
        "a = 1 + 2 * 3",
        "a = 1 + foo(true) * 3 - 4"
    );
}

// #[test]
// fn test_ast_empty_statement() {
//     let test_string = ";";

//     // pair parsing
//     let pairs = CTinyParser::parse(Rule::empty_statement, test_string);
//     print!("pairs: {:?}\n", pairs);
//     // check that pairs OK and empty
//     assert!(pairs.is_ok());
//     assert_eq!(pairs.unwrap().count(), 0);
// }

#[test]
fn test_multi_statement_vector_from_pair() {
    // test with assignment statements
    let test_string = "a = 1; b = 2; c = 3;";
    let pairs = CTinyParser::parse(Rule::multi_statement, test_string)
        .unwrap();
    
    let first_pair = pairs.clone().next().unwrap();
    assert_eq!(first_pair.as_rule(), Rule::multi_statement);
    assert_eq!(first_pair.as_str(), test_string);

    let multi_statement = multi_statement_vector_from_pair(first_pair);
    assert!(multi_statement.is_ok());
    assert_eq!(multi_statement.unwrap().len(), 3);

    // test with an empty statements
    let test_string = "; ; ;";
    let pairs = CTinyParser::parse(Rule::multi_statement, test_string)
        .unwrap();
    
    let first_pair = pairs.clone().next().unwrap();
    assert_eq!(first_pair.as_rule(), Rule::multi_statement);
    assert_eq!(first_pair.as_str(), test_string);

    let multi_statement = multi_statement_vector_from_pair(first_pair);
    assert!(multi_statement.is_ok());
    assert_eq!(multi_statement.unwrap().len(), 0);

    // test with an empty statements
    let test_string = "; true; foo(10);;";
    let pairs = CTinyParser::parse(Rule::multi_statement, test_string)
        .unwrap();

    print!("nb pairs: {}\n", pairs.clone().count());
    for pair in pairs.clone() {
        println!("Rule: {:?}, Text: {}", pair.as_rule(), pair.as_str());
    }
    
    let first_pair = pairs.clone().next().unwrap();
    assert_eq!(first_pair.as_rule(), Rule::multi_statement);
    assert_eq!(first_pair.as_str(), test_string);

    let multi_statement = multi_statement_vector_from_pair(first_pair);
    assert!(multi_statement.is_ok());
    assert_eq!(multi_statement.unwrap().len(), 2);
}

#[test]
fn test_ast_if_else() {
    build_test_statement!(Rule::if_else_statement,
        "if (true) { }",
        "if (true) { ; }",
        "if (true) { ; } else { }",
        "if (true) { 3; } else { true; }",
        "if (true) { 5 + foo(1024) } else { bar(3.14159); }"
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
//         let ast = build_statement(first_pair)
//         .unwrap_or_else(|error| { print!("{}\n", error); panic!(); });
//         print!("{:#?}", ast);
// }