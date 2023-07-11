use pest::Parser;

use crate::syntax_parsing::parser::{CTinyParser, Rule};
use crate::syntax_tree::declarations::build_declaration;

use crate::build_test;

macro_rules! build_test_declaration {
    ($rule:expr, $( $input_str:literal),* ) => {
        build_test!($rule, build_declaration, $( $input_str),* );
    }
}