use pest::Span;

use crate::abstract_syntax_tree::nodes::{Node, Literal, TypeSpecifier};
use crate::semantic_analysis::type_casts::cast_literal_to_type;


// TODO: add tests for all the different types of casts

#[test]
fn test_cast_literal_to_type_int() {
    // Correct Cases

    // Case: Casting Int to Int
    let test_literal = Literal::Int(1);
    let input_literal_node = Node {
        sp: Span::new("1", 0, 1).unwrap(),
        data: test_literal.clone(),
    };
    assert_eq!(
        cast_literal_to_type(input_literal_node, TypeSpecifier::Int).unwrap().data,
        test_literal
    );

    // Incorrect Cases

    // Case: Overflow when casting large Float to Int
    let input_literal_node = Node {
        sp: Span::new("3.4028235e+38", 0, 12).unwrap(),
        data: Literal::Float(std::f32::MAX as f32),
    };
    assert!(
        cast_literal_to_type(input_literal_node, TypeSpecifier::Int).is_err()
    );
}