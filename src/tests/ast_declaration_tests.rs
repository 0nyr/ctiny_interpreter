use pest::Parser;

use crate::syntax_parsing::parser::{CTinyParser, Rule};
use crate::syntax_tree::declarations::build_declaration;

use crate::build_test;

macro_rules! build_test_declaration {
    ($rule:expr, $( $input_str:literal),* ) => {
        build_test!($rule, build_declaration, $( $input_str),* );
    }
}

#[test]
fn test_ast_declaration() {
    build_test_declaration!(Rule::declaration,
        "int x",
        "int x[10]",
        "char x",
        "char x[10]",
        "float x",
        "float x[10]",
        "bool x",
        "bool x[10]"
    );
}

#[test]
fn test_ast_parameter_list() {
    build_test_declaration!(Rule::parameter_list,
        "int x",
        "int x[10]",
        "char x",
        "char x[10]",
        "float x",
        "float x[10]",
        "bool x",
        "bool x[10]",
        "int x, int y[10], int z",
        "char x, char y[10], char z",
        "float x[10], int y, char z",
        "bool x, float y[10], bool z[10]"
    );
}