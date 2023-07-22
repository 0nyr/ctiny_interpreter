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
    let test_string = "a = foo(10); a = 1 + 2;";
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
        "if (a != false) { a = 3; } else { a = 4; }",
        "if (true) { a = (5 + foo(1024)); } else { a = bar(3.14159); }"
    );
}

#[test]
fn test_ast_nested_if_else() {
    let test_string = "if (a < 10) { if (a % 2 == 0) {} } else { if (a % 2 == 0) {} else {} }";
    let pairs = match CTinyParser::parse(Rule::if_else_statement, test_string) {
        Ok(pairs) => pairs,
        Err(error) => {
            print!("Syntax parsing error for {}: \n {}\n", test_string, error);
            panic!();
        },
    };
    
    let first_pair = pairs.clone().next().unwrap();
    assert_eq!(first_pair.as_rule(), Rule::if_else_statement);
    assert_eq!(first_pair.as_str(), test_string);

    print!("nb pairs: {}\n", pairs.clone().count());
    for pair in pairs.clone() {
        println!("Rule: {:?}, Text: {}", pair.as_rule(), pair.as_str());
    }
}

#[test]
fn test_ast_nested_if_while() {
    let test_string = "if (a < 10) { if (a % 2 == 0) { a = a + 1; } } else { while (a % 2 == 0) { a = a + 1; } }";
    let pairs = match CTinyParser::parse(Rule::if_else_statement, test_string) {
        Ok(pairs) => pairs,
        Err(error) => {
            print!("Syntax parsing error for {}: \n {}\n", test_string, error);
            panic!();
        },
    };
    
    let first_pair = pairs.clone().next().unwrap();
    assert_eq!(first_pair.as_rule(), Rule::if_else_statement);
    assert_eq!(first_pair.as_str(), test_string);

    print!("nb pairs: {}\n", pairs.clone().count());
    for pair in pairs.clone() {
        println!("Rule: {:?}, Text: {}", pair.as_rule(), pair.as_str());
    }
}

#[test]
fn test_while_statement() {
    build_test_statement!(Rule::while_statement,
        "while (true) { }",
        "while (a == b) { }",
        "while (false) { }",
        "while (a != false) { a = 3; }",
        "while (true) { a = (5 + foo(1024)); }",
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
        "if (true) { }",
        "if (a == b) { }",
        "if (false) { } else { }",
        "if (a != false) { a = 3; } else { a = 4; }",
        "if (true) { a = (5 + foo(1024)); } else { a = bar(3.14159); }",
        "while (true) { }",
        "while (a == b) { }",
        "while (false) { }",
        "while (a != false) { a = 3; }",
        "while (true) { a = (5 + foo(1024)); }",
        "while (a < 10) { if (a % 2 == 0) { a = a + 1; } else { a = a + 2; } }"
    );
}