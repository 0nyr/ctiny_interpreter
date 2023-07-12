use pest::Parser;

use crate::syntax_parsing::parser::{CTinyParser, Rule};
use crate::syntax_tree::functions::{build_block, build_function_definition, build_entry_point_function};

use crate::build_test;

#[test]
fn test_ast_function_block() {
    build_test!(Rule::block, build_block,
        "{ }",
        "{ int x; }",
        "{ int x; int y; }",
        "{ int x; char y[10]; x = 3; y[0] = 4; }",
        "{ int x; char y[10]; x = 3; y[0] = 4; if (x == y[0]) { y[1] = y[0]; } }"
    );
}

#[test]
fn test_ast_function_definition() {
    build_test!(Rule::function_definition, build_function_definition,
        "int foo() { }",
        "int foo() { int x; }",
        "int foo() { int x; int y; }",
        "int foo() { int x; char y[10]; x = 3; y[0] = 4; }",
        "int foo() { int x; char y[10]; x = 3; y[0] = 4; if (x == y[0]) { y[1] = y[0]; } }",
        "int foo(int a, char b[10], bool c) { int x; char y[10]; x = 3; y[0] = 4; if (x == y[0]) { y[1] = y[0]; } }"
    );
}

#[test]
fn test_ast_entry_point_function() {
    build_test!(Rule::entry_point_function_definition, build_entry_point_function,
        "int main() { }",
        "int main() { int x; }",
        "int main() { int x; int y; }",
        "int main() { int x; char y[10]; x = 3; y[0] = 4; }",
        "int main() { int x; char y[10]; x = 3; y[0] = 4; if (x == y[0]) { y[1] = y[0]; } }",
        "int main() { int x; char y[10]; x = 3; y[0] = 4; if (x == y[0]) { y[1] = y[0]; } }"
    );
}