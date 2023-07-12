use pest::Parser;

use crate::syntax_parsing::parser::{CTinyParser, Rule};
use crate::syntax_tree::build_translation_unit;

use crate::build_test;

macro_rules! build_test_program {
    ($rule:expr, $( $input_str:literal),* ) => {
        build_test!($rule, build_translation_unit, $( $input_str),* );
    }
}

#[test]
fn test_ast_translation_unit() {
    build_test_program(Rule::translation_unit,
        "int main () { }",
        "char foo() { } int bar() { } int main () { }",
        "int foo() { int x; char y[10]; x = 3; y[0] = 4; } int main() { int x; char y[10]; x = 3; y[0] = 4; }",
        "int main() { int x; char y[10]; x = 3; y[0] = 4; if (x == y[0]) { y[1] = y[0]; } }"
    )
}




