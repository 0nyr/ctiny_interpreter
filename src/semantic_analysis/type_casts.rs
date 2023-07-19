use crate::abstract_syntax_tree::nodes::{Literal, Node, TypeSpecifier};

use super::errors::{SemanticError, NegativeArrayIndexError, UnexpectedLiteralTypeError, SemanticErrorTrait, IntToCharCastOverflowError, FloatToCharCastOverflowError, FloatToIntCastOverflowError};

// given a Literal, check that it is a positive integer, and return the value as usize
pub fn get_index_value_from_literal<'a>(literal_node: Node<'a, Literal>) -> Result<usize, SemanticError> {
    // first, need to cast to int
    let int_literal = cast_literal_to_type(
        literal_node.clone(), TypeSpecifier::Int
    )?.data;
    // then, check that it is positive
    match int_literal {
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

macro_rules! ok_literal_node {
    ($literal_type:ident, $span:expr, $bool:expr) => {
        Ok(
            Node {
                sp: $span,
                data: Literal::$literal_type($bool),
            }
        )
    };
}

fn cast_literal_to_bool<'a>(
    literal_node: Node<'a, Literal>
) -> Result<Node<'a, Literal>, SemanticError> {
    match literal_node.data {
        Literal::Bool(_) => Ok(literal_node),
        Literal::Int(integer) => {
            if integer == 0 {
                ok_literal_node!(Bool, literal_node.sp, false)
            } else {
                ok_literal_node!(Bool, literal_node.sp, true)
            }
        },
        Literal::Float(float) => {
            if float == 0.0 {
                ok_literal_node!(Bool, literal_node.sp, false)
            } else {
                ok_literal_node!(Bool, literal_node.sp, true)
            }
        },
        Literal::Char(character) => {
            if character == b'\0' {
                ok_literal_node!(Bool, literal_node.sp, false)
            } else {
                ok_literal_node!(Bool, literal_node.sp, true)
            }
        },
    }
}

fn cast_literal_to_float<'a>(
    literal_node: Node<'a, Literal>
) -> Result<Node<'a, Literal>, SemanticError> {
    match literal_node.data {
        Literal::Bool(boolean) => {
            if boolean {
                ok_literal_node!(Float, literal_node.sp, 1.0)
            } else {
                ok_literal_node!(Float, literal_node.sp, 0.0)
            }
        },
        Literal::Int(integer) => {
            ok_literal_node!(Float, literal_node.sp, integer as f32)
        },
        Literal::Float(_) => Ok(literal_node),
        Literal::Char(character) => {
            ok_literal_node!(Float, literal_node.sp, character as f32)
        },
    }
}

fn cast_literal_to_char<'a>(
    literal_node: Node<'a, Literal>
) -> Result<Node<'a, Literal>, SemanticError> {
    match literal_node.data {
        Literal::Bool(boolean) => {
            if boolean {
                ok_literal_node!(Char, literal_node.sp, 1)
            } else {
                ok_literal_node!(Char, literal_node.sp, b'\0')
            }
        },
        Literal::Int(integer) => {
            if integer < 0 || integer > 255 {
                Err(
                    SemanticError::IntToCharCastOverflow(
                        IntToCharCastOverflowError::init(
                            literal_node.sp,
                            &format!("Int literal out of range for char: {}", integer)
                        )
                    )
                )
            } else {
                ok_literal_node!(Char, literal_node.sp, integer as u8)
            }
        },
        Literal::Float(float) => {
            if float < 0.0 || float > 255.0 {
                Err(
                    SemanticError::FloatToCharCastOverflow(
                        FloatToCharCastOverflowError::init(
                            literal_node.sp,
                            &format!("Float literal out of range for char: {}", float)
                        )
                    )
                )
            } else {
                ok_literal_node!(Char, literal_node.sp, float as u8)
            }
        },
        Literal::Char(_) => Ok(literal_node),
    }
}

fn cast_literal_to_int<'a>(
    literal_node: Node<'a, Literal>
) -> Result<Node<'a, Literal>, SemanticError> {
    match literal_node.data {
        Literal::Bool(boolean) => {
            if boolean {
                ok_literal_node!(Int, literal_node.sp, 1)
            } else {
                ok_literal_node!(Int, literal_node.sp, 0)
            }
        },
        Literal::Int(_) => Ok(literal_node),
        Literal::Float(float) => {
            if float > i16::MAX as f32 || float < i16::MIN as f32 {
                Err(
                    SemanticError::FloatToIntCastOverflow(
                        FloatToIntCastOverflowError::init(
                            literal_node.sp,
                            &format!("Float literal out of range for int: {}", float)
                        )
                    )
                )
            } else {
                ok_literal_node!(Int, literal_node.sp, float as i16)
            }
        },
        Literal::Char(character) => {
            ok_literal_node!(Int, literal_node.sp, character as i16)
        },
    }
}

/// this function is used to cast a literal to a given type
/// 
/// It checks overflows and underflows, as well as invalid values
pub fn cast_literal_to_type<'a>(
    input_literal_node: Node<'a, Literal>,
    target_type: TypeSpecifier,
) -> Result<Node<'a, Literal>, SemanticError> {
    match target_type {
        TypeSpecifier::Bool => {
            cast_literal_to_bool(input_literal_node)
        },
        TypeSpecifier::Float => {
            cast_literal_to_float(input_literal_node)
        },
        TypeSpecifier::Char => {
            cast_literal_to_char(input_literal_node)
        },
        TypeSpecifier::Int => {
            cast_literal_to_int(input_literal_node)
        },
    }
}