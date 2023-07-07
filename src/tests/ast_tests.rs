use pest::{Parser, iterators::Pairs};

use crate::syntax_parsing::parser::{CTinyParser, Rule};

#[test]
fn test_ast_integer_literal() {
    let test_string = "1024";

    // pair parsing
    let pairs = CTinyParser::parse(Rule::literal, test_string)
        .unwrap();

    let first_pair = pairs.into_iter().next().unwrap();


    // let pair = first_pair.clone().into_inner().next().unwrap();
    // assert_eq!(pair.as_rule(), Rule::assignment_statement);
    // assert_eq!(pair.as_str(), test_string);

    // AST conversion
    let ast = crate::syntax_tree::build_literal(first_pair)
    .unwrap_or_else(|error| { print!("{}\n", error); panic!(); });
    print!("{:#?}", ast);
}