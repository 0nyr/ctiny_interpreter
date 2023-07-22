use pest::Parser;

use crate::syntax_parsing::{CTinyParser, Rule};
use crate::abstract_syntax_tree::functions::{build_block, build_function_definition, build_entry_point_function};

use crate::build_test;

#[test]
fn test_ast_function_block_return() {
    let test_str = "{ int x; char y[10]; x = 3; y[0] = 4; if (x == y[0]) { y[1] = y[0]; } return y[10]; }";
    let rule = Rule::block;

    // Syntax parsing
    let pairs = CTinyParser::parse(rule, test_str).unwrap();

    let first_pair = pairs.into_iter().next().unwrap();
    assert_eq!(first_pair.as_rule(), rule);
    assert_eq!(first_pair.as_str(), test_str);

    // AST conversion
    // WARN: don't forget to change the method if needed
    let ast = build_block(first_pair)
        .unwrap_or_else(|error| { 
            print!("AST ERROR for {}: \n {}\n", test_str, error); 
            panic!(); 
        });
    print!("AST for string \"{}\": \n {:#?} \n\n", test_str, ast);
}

#[test]
fn test_ast_function_block() {
    build_test!(Rule::block, build_block,
        "{ return 0; }",
        "{ int x; return x; }",
        "{ int x; int y; return y; }",
        "{ int x; char y[10]; x = 3; y[0] = 4; return 0; }",
        "{ int x; char y[10]; x = 3; y[0] = 4; if (x == y[0]) { y[1] = y[0]; } return y[10]; }"
    );
}

#[test]
fn test_ast_function_definition() {
    build_test!(Rule::function_definition, build_function_definition,
        "int foo() { return 0; }",
        "int foo() { int x; return x; }",
        "int foo() { int x; int y; return y; }",
        "int foo() { int x; char y[10]; x = 3; y[0] = 4; return x + y[0]; }",
        "int foo() { int x; char y[10]; x = 3; y[0] = 4; if (x == y[0]) { y[1] = y[0]; } return x + y[0]; }",
        "int foo(int a, char b[10], bool c) { int x; char y[10]; x = 3; y[0] = 4; if (x == y[0]) { y[1] = y[0]; } return x + y[0]; }"
    );
}

#[test]
fn test_ast_entry_point_function() {
    build_test!(Rule::entry_point_function_definition, build_entry_point_function,
        "int main() { return 0; }",
        "int main() { int x; return x; }",
        "int main() { int x; int y; return y; }",
        "int main() { int x; char y[10]; x = 3; y[0] = 4; return x + y[0]; }",
        "int main() { int x; char y[10]; x = 3; y[0] = 4; if (x == y[0]) { y[1] = y[0]; } return x + y[0]; }"
    );
}