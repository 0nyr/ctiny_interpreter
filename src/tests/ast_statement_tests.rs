use pest::Parser;

use crate::syntax_parsing::parser::{CTinyParser, Rule};
use crate::syntax_tree::statements::build_statement;

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
    // WARN: forgetting the semicolon will cause an error due 
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