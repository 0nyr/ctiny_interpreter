use crate::abstract_syntax_tree::nodes::{Literal, Node};

use super::errors::{SemanticError, NegativeArrayIndexError, UnexpectedLiteralTypeError, SemanticErrorTrait};

// given a Literal, check that it is a positive integer, and return the value as usize
pub fn get_index_value_from_literal<'a>(literal_node: Node<'a, Literal>) -> Result<usize, SemanticError> {
    match literal_node.data {
        Literal::Int(integer) => {
            if integer < 0 {
                Err(
                    SemanticError::NegativeArrayIndex(
                        NegativeArrayIndexError::init(
                            literal_node.sp,
                            &format!("Negative array index: {}", integer)
                        )
                    )
                )
            } else {
                Ok(integer as usize)
            }
        },
        literal => Err(
            SemanticError::UnexpectedLiteralType(
                UnexpectedLiteralTypeError::init(
                    literal_node.sp,
                    &format!("Expected integer literal, got: {:?}", literal)
                )
            )
        ),
    }
}