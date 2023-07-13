use pest::Parser;

use crate::syntax_parsing::{CTinyParser, Rule};
use crate::abstract_syntax_tree::statements::{build_statement, build_multi_statement};

use crate::build_test;

macro_rules! build_test_statement {
    ($rule:expr, $( $input_str:literal),* ) => {
        build_test!($rule, build_statement, $( $input_str),* );
    }
}

#[test]
fn test_ast_return() {
    build_test_statement!(Rule::return_statement,
        "return 1;",
        "return 1 + 2;",
        "return 1 + 2 * 3;",
        "return 1 + foo(true) * 3 - 4;"
    ); 
    // WARN: forgetting the semicolon after macro call will cause an error due 
    // to the fact that the function will try to return the last expression...
}

#[test]
fn test_ast_break() {
    build_test_statement!(Rule::break_statement,
        "break;",
        "break ;"
    );
}

#[test]
fn test_ast_continue() {
    build_test_statement!(Rule::continue_statement,
        "continue;",
        "continue ;"
    );
}

#[test]
fn test_ast_assignment() {
    build_test_statement!(Rule::assignment_statement,
        "a = 1;",
        "a = true;",
        "a = 3.14159;",
        "a = 'a';",
        "a = 1 + 2;",
        "a = 1 + 2 * 3;",
        "a = 1 + foo(true) * 3 - 4;"
    );
}

#[test]
fn test_build_multi_statement() {
    // test with assignment statements
    let test_string = "a = 1; b = 2; c = 3;";
    let pairs = CTinyParser::parse(Rule::multi_statement, test_string)
        .unwrap();
    
    let first_pair = pairs.clone().next().unwrap();
    assert_eq!(first_pair.as_rule(), Rule::multi_statement);
    assert_eq!(first_pair.as_str(), test_string);

    let multi_statement = build_multi_statement(first_pair);
    assert!(multi_statement.is_ok());
    assert_eq!(multi_statement.unwrap().len(), 3);

    // test other statements
    let test_string = "a = foo(10); return 1;";
    let pairs = CTinyParser::parse(Rule::multi_statement, test_string)
        .unwrap();

    print!("nb pairs: {}\n", pairs.clone().count());
    for pair in pairs.clone() {
        println!("Rule: {:?}, Text: {}", pair.as_rule(), pair.as_str());
    }
    
    let first_pair = pairs.clone().next().unwrap();
    assert_eq!(first_pair.as_rule(), Rule::multi_statement);
    assert_eq!(first_pair.as_str(), test_string);

    let multi_statement = build_multi_statement(first_pair);
    assert!(multi_statement.is_ok());
    assert_eq!(multi_statement.unwrap().len(), 2);
}

#[test]
fn test_ast_if_else() {
    build_test_statement!(Rule::if_else_statement,
        "if (true) { }",
        "if (a == b) { }",
        "if (false) { } else { }",
        "if (a != false) { a = 3; } else { return true; }",
        "if (true) { a = (5 + foo(1024)); continue; } else { a = bar(3.14159); }"
    );
}

#[test]
fn test_while_statement() {
    build_test_statement!(Rule::while_statement,
        "while (true) { }",
        "while (a == b) { }",
        "while (false) { }",
        "while (a != false) { a = 3; }",
        "while (true) { a = (5 + foo(1024)); continue; }",
        "while (a < 10) { if (a % 2 == 0) { a = a + 1; } else { a = a + 2; } }"
    );
}

#[test]
fn test_ast_statement() {
    build_test_statement!(Rule::statement,
        "a = 1;",
        "a = true;",
        "a = 3.14159;",
        "a = 'a';",
        "a = 1 + 2;",
        "a = 1 + 2 * 3;",
        "a = 1 + foo(true) * 3 - 4;",
        "return 1;",
        "return 1 + 2;",
        "return 1 + 2 * 3;",
        "return 1 + foo(true) * 3 - 4;",
        "break;",
        "continue;",
        "if (true) { }",
        "if (a == b) { }",
        "if (false) { } else { }",
        "if (a != false) { a = 3; } else { return true; }",
        "if (true) { a = (5 + foo(1024)); continue; } else { a = bar(3.14159); }",
        "while (true) { }",
        "while (a == b) { }",
        "while (false) { }",
        "while (a != false) { a = 3; }",
        "while (true) { a = (5 + foo(1024)); continue; }",
        "while (a < 10) { if (a % 2 == 0) { a = a + 1; } else { a = a + 2; } }"
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