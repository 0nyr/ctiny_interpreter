use pest::Parser;

use crate::syntax_parsing::{CTinyParser, Rule};
use crate::abstract_syntax_tree::build_translation_unit;

use crate::build_test;

macro_rules! build_test_program {
    ($rule:expr, $( $input_str:literal),* ) => {
        build_test!($rule, build_translation_unit, $( $input_str),* );
    }
}

#[test]
fn test_ast_translation_unit() {
    build_test_program!(Rule::translation_unit,
        "int main () { return 0; }",
        "char foo() { return 'a'; } int bar() { return 10; } int main () { return foo() + bar(); }",
        "int foo() { int x; char y[10]; x = 3; y[0] = 4; return x + y[0]; } int main() { int x; char y[10]; x = 3; y[0] = 4; return x + y[0] + foo(); }",
        "int main() { int x; char y[10]; x = 3; y[0] = 4; if (x == y[0]) { y[1] = y[0]; } return 0; }"
    );
}




