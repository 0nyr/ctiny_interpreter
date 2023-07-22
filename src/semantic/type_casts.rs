use crate::abstract_syntax_tree::nodes::{Value, Node, TypeSpecifier};

use super::errors::{SemanticError, NegativeArrayIndexError, UnexpectedLiteralTypeError, SemanticErrorTrait, IntToCharCastOverflowError, FloatToCharCastOverflowError, FloatToIntCastOverflowError};

// given a Literal, check that it is a positive integer, and return the value as usize
pub fn get_index_value_from_value_node<'a>(input_node: Node<'a, Value>) -> Result<usize, SemanticError> {
    // first, need to cast to int
    let int_literal = cast_to_type(
        input_node.clone(), TypeSpecifier::Int
    )?.data;
    // then, check that it is positive
    match int_literal {
        Value::Int(integer) => {
            if integer < 0 {
                Err(
                    SemanticError::NegativeArrayIndex(
                        NegativeArrayIndexError::init(
                            input_node.sp,
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
                    input_node.sp,
                    &format!("Expected integer literal, got: {:?}", literal)
                )
            )
        ),
    }
}

macro_rules! ok_value_node {
    ($value_type:ident, $span:expr, $bool:expr) => {
        Ok(
            Node {
                sp: $span,
                data: Value::$value_type($bool),
            }
        )
    };
}

fn cast_to_bool<'a>(
    input_node: Node<'a, Value>
) -> Result<Node<'a, Value>, SemanticError> {
    match input_node.data {
        Value::Bool(_) => Ok(input_node),
        Value::Int(integer) => {
            if integer == 0 {
                ok_value_node!(Bool, input_node.sp, false)
            } else {
                ok_value_node!(Bool, input_node.sp, true)
            }
        },
        Value::Float(float) => {
            if float == 0.0 {
                ok_value_node!(Bool, input_node.sp, false)
            } else {
                ok_value_node!(Bool, input_node.sp, true)
            }
        },
        Value::Char(character) => {
            if character == b'\0' {
                ok_value_node!(Bool, input_node.sp, false)
            } else {
                ok_value_node!(Bool, input_node.sp, true)
            }
        },
    }
}

fn cast_to_float<'a>(
    input_node: Node<'a, Value>
) -> Result<Node<'a, Value>, SemanticError> {
    match input_node.data {
        Value::Bool(boolean) => {
            if boolean {
                ok_value_node!(Float, input_node.sp, 1.0)
            } else {
                ok_value_node!(Float, input_node.sp, 0.0)
            }
        },
        Value::Int(integer) => {
            ok_value_node!(Float, input_node.sp, integer as f32)
        },
        Value::Float(_) => Ok(input_node),
        Value::Char(character) => {
            ok_value_node!(Float, input_node.sp, character as f32)
        },
    }
}

fn cast_to_char<'a>(
    input_node: Node<'a, Value>
) -> Result<Node<'a, Value>, SemanticError> {
    match input_node.data {
        Value::Bool(boolean) => {
            if boolean {
                ok_value_node!(Char, input_node.sp, 1)
            } else {
                ok_value_node!(Char, input_node.sp, b'\0')
            }
        },
        Value::Int(integer) => {
            if integer < 0 || integer > 255 {
                Err(
                    SemanticError::IntToCharCastOverflow(
                        IntToCharCastOverflowError::init(
                            input_node.sp,
                            &format!("Int literal out of range for char: {}", integer)
                        )
                    )
                )
            } else {
                ok_value_node!(Char, input_node.sp, integer as u8)
            }
        },
        Value::Float(float) => {
            if float < 0.0 || float > 255.0 {
                Err(
                    SemanticError::FloatToCharCastOverflow(
                        FloatToCharCastOverflowError::init(
                            input_node.sp,
                            &format!("Float literal out of range for char: {}", float)
                        )
                    )
                )
            } else {
                ok_value_node!(Char, input_node.sp, float as u8)
            }
        },
        Value::Char(_) => Ok(input_node),
    }
}

fn cast_to_int<'a>(
    input_node: Node<'a, Value>
) -> Result<Node<'a, Value>, SemanticError> {
    match input_node.data {
        Value::Bool(boolean) => {
            if boolean {
                ok_value_node!(Int, input_node.sp, 1)
            } else {
                ok_value_node!(Int, input_node.sp, 0)
            }
        },
        Value::Int(_) => Ok(input_node),
        Value::Float(float) => {
            if float > i16::MAX as f32 || float < i16::MIN as f32 {
                Err(
                    SemanticError::FloatToIntCastOverflow(
                        FloatToIntCastOverflowError::init(
                            input_node.sp,
                            &format!("Float literal out of range for int: {}", float)
                        )
                    )
                )
            } else {
                ok_value_node!(Int, input_node.sp, float as i16)
            }
        },
        Value::Char(character) => {
            ok_value_node!(Int, input_node.sp, character as i16)
        },
    }
}

/// this function is used to cast a literal to a given type
/// 
/// It checks overflows and underflows, as well as invalid values
pub fn cast_to_type<'a>(
    input_node: Node<'a, Value>,
    target_type: TypeSpecifier,
) -> Result<Node<'a, Value>, SemanticError> {
    match target_type {
        TypeSpecifier::Bool => {
            cast_to_bool(input_node)
        },
        TypeSpecifier::Float => {
            cast_to_float(input_node)
        },
        TypeSpecifier::Char => {
            cast_to_char(input_node)
        },
        TypeSpecifier::Int => {
            cast_to_int(input_node)
        },
    }
}