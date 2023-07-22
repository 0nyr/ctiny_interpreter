use crate::semantic::semantic_analysis;
use crate::pipelines::parse_content_into_ast;

macro_rules! build_semantic_test {
    ($( $input_str:literal),* ) => {
        $( 
            let test_string = $input_str;
            let ast = parse_content_into_ast(test_string, None);
            let semantic_analysis_result = semantic_analysis(ast);
            assert!(semantic_analysis_result.is_err());
        )*
    }
}

#[test]
fn test_semantic_undeclared_variable() {
    // let test_string = "
    // int main () {
    //     int x;
    //     x = y; // y is not declared
    // }
    // ";
    
    // let ast = parse_content_into_ast(test_string, None);
    // let semantic_analysis_result = semantic_analysis(ast);
    // assert!(semantic_analysis_result.is_err());
    build_semantic_test!(
        "int main () {
            int x;
            x = y; // y is not declared
            return 0;
        }",
        "int foo(int x) {
            return x;
        }
        int main () {
            int x;
            x = foo(y); // y is not declared
            return 0;
        }"
    );
}

#[test]
fn test_semantic_undeclared_function() {
    build_semantic_test!(
        "int main () {
            int x;
            x = foo(); // foo is not declared
        }"
    );
}

#[test]
fn test_semantic_unassigned_variable() {
    build_semantic_test!(
        "int main () {
            int x;
            int y;
            x = y; // y is not assigned
            return 0;
        }",
        "int main () {
            int x;
            return x; // x is not assigned
        }",
        "int foo() {
            int x;
            return x; // x is not assigned
        }
        int main () {
            int x;
            x = 1;
            return x;
        }"
    );
}