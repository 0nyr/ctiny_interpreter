use pest::{Parser, iterators::Pairs};

use crate::syntax_parsing::parser::{CTinyParser, Rule};

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
            let ast = crate::syntax_tree::build_literal(first_pair)
                .unwrap_or_else(|error| { print!("AST ERROR for {}: \n {}\n", test_str, error); panic!(); });
            print!("AST for string \"{}\": \n {:#?} \n\n", test_str, ast);
        }
    };
}

#[test]
fn test_ast_literal() {
    build_test!(Rule::literal, "1024", "'a'", "true", "false", "3.14159", "0.001");
}

// #[test]
// fn test_ast_char_literal() {
//     let test_string = ;

//     // pair parsing
//     let pairs = CTinyParser::parse(Rule::literal, test_string)
//         .unwrap();

//     let first_pair = pairs.into_iter().next().unwrap();
//     assert_eq!(first_pair.as_rule(), Rule::literal);
//     assert_eq!(first_pair.as_str(), test_string);

//     // AST conversion
//     let ast = crate::syntax_tree::build_literal(first_pair)
//     .unwrap_or_else(|error| { print!("{}\n", error); panic!(); });
//     print!("{:#?}", ast);
// }

