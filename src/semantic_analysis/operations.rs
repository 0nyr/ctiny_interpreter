use pest::Span;

use crate::abstract_syntax_tree::nodes::{Node, Value, Expression, UnaryExpression, BinaryExpression, FunctionCall, TypeCast, GetOrSetValue, Identifier, UnaryOperator, TypeSpecifier, BinaryOperator};
use crate::merge_spans_no_check;
use crate::semantic_analysis::errors::{SemanticError, SemanticErrorTrait, UnexpectedTypeCastError};
use crate::semantic_analysis::type_casts::cast_literal_to_type;

use super::overflow_checks::*;

macro_rules! basic_binary_operation {
    ($operator_str:expr, $safe_float_operation:ident, $safe_int_operation:ident, $left_value_node:expr, $right_value_node:expr, $type:ty) => {{
        let left_type = $left_value_node.data.as_type_specifier();
        let right_type = $right_value_node.data.as_type_specifier();
        let common_span = merge_spans_no_check!(
            $left_value_node.sp, $right_value_node.sp
        ).unwrap();

        if left_type == TypeSpecifier::Float || right_type == TypeSpecifier::Float {
            let left_float = cast_literal_to_type(
                $left_value_node.clone(), TypeSpecifier::Float
            )?;
            let right_float = cast_literal_to_type(
                $right_value_node.clone(), TypeSpecifier::Float
            )?;
            match (left_float.data.clone(), right_float.data.clone()) {
                (Value::Float(left_float), Value::Float(right_float)) => {
                    let result = $safe_float_operation(
                        left_float, 
                        right_float,
                        common_span,
                    )?;
                    Ok(Node {
                        sp: common_span,
                        data: Value::Float(result),
                    })
                },
                _ => {
                    return Err(SemanticError::UnexpectedTypeCast(
                        UnexpectedTypeCastError::init(
                            common_span,
                            format!(
                                "In perform_binary_operation ({}), cast to float of {:?} or {:?} failed", 
                                $operator_str, left_float.data, right_float.data
                            ).as_str(),
                        )
                    ));
                },
            }
        } else {
            let left_int = cast_literal_to_type(
                $left_value_node.clone(), TypeSpecifier::Int
            )?;
            let right_int = cast_literal_to_type(
                $right_value_node.clone(), TypeSpecifier::Int
            )?;
            match (left_int.data, right_int.data) {
                (Value::Int(left_int), Value::Int(right_int)) => {
                    // We need to convert integers to floats before passing to the operation closure
                    let result = $safe_int_operation(
                        left_int, 
                        right_int,
                        common_span,
                    )?;
                    Ok(Node {
                        sp: common_span,
                        data: Value::Int(result),
                    })
                },
                (left, right) => {
                    return Err(SemanticError::UnexpectedTypeCast(
                        UnexpectedTypeCastError::init(
                            common_span,
                            format!(
                                "In perform_binary_operation ({}), cast to int of {:?} or {:?} failed", 
                                $operator_str, left, right
                            ).as_str(),
                        )
                    ));
                },
            }
        }
    }}
}

// pub fn perform_plus_operation<'a>(
//     left_value_node: &Node<'a, Value>,
//     right_value_node: &Node<'a, Value>,
// ) -> Result<Node<'a, Value>, SemanticError> {
//     // perform addition. Respect order of evaluation of operands and return the result.
//     // addition works really for all types, so just cast to int or float if necessary.
//     // if one of the operands is a float, cast both to float and perform addition
//     // if no operand is a float, cast both to int and perform addition
//     let left_type = left_value_node.data.as_type_specifier();
//     let right_type = right_value_node.data.as_type_specifier();
//     let common_span = merge_spans_no_check!(
//         left_value_node.sp, right_value_node.sp
//     ).unwrap();

//     if left_type == TypeSpecifier::Float || right_type == TypeSpecifier::Float {
//         let left_float = cast_literal_to_type(
//             left_value_node.clone(), TypeSpecifier::Float
//         )?;
//         let right_float = cast_literal_to_type(
//             right_value_node.clone(), TypeSpecifier::Float
//         )?;
//         // check that the cast worked and perform addition
//         match (left_float.data.clone(), right_float.data.clone()) {
//             (Value::Float(left_float), Value::Float(right_float)) => {
//                 // perform addition (check for overflow)
//                 let result = safe_float_add(left_float, right_float, common_span)?;
//                 Ok(Node {
//                     sp: common_span,
//                     data: Value::Float(result),
//                 })
//             },
//             _ => {
//                 return Err(SemanticError::UnexpectedTypeCast(
//                     UnexpectedTypeCastError::init(
//                         common_span,
//                         format!(
//                             "In perform_binaty_operation, cast to float of {:?} or {:?} failed", 
//                             left_float.data, right_float.data
//                         ).as_str(),
//                     )
//                 ));
//             },
//         }
//     } else {
//         let left_int = cast_literal_to_type(
//             left_value_node.clone(), TypeSpecifier::Int
//         )?;
//         let right_int = cast_literal_to_type(
//             right_value_node.clone(), TypeSpecifier::Int
//         )?;
//         match (left_int.data, right_int.data) {
//             (Value::Int(left_int), Value::Int(right_int)) => {
//                 // perform addition (check for overflow)
//                 let result = safe_int_add(left_int, right_int, common_span)?;
//                 Ok(Node {
//                     sp: common_span,
//                     data: Value::Int(result),
//                 })
//             },
//             (left, right) => {
//                 return Err(SemanticError::UnexpectedTypeCast(
//                     UnexpectedTypeCastError::init(
//                         common_span,
//                         format!(
//                             "In perform_binaty_operation, cast to int of {:?} or {:?} failed", 
//                             left, right
//                         ).as_str(),
//                     )
//                 ));
//             },
//         }
//     }
// }

// pub fn perform_minus_operation<'a>(
//     left_value_node: &Node<'a, Value>,
//     right_value_node: &Node<'a, Value>,
// ) -> Result<Node<'a, Value>, SemanticError> {
//     let left_type = left_value_node.data.as_type_specifier();
//     let right_type = right_value_node.data.as_type_specifier();
//     let common_span = merge_spans_no_check!(
//         left_value_node.sp, right_value_node.sp
//     ).unwrap();

//     if left_type == TypeSpecifier::Float || right_type == TypeSpecifier::Float {
//         let left_float = cast_literal_to_type(
//             left_value_node.clone(), TypeSpecifier::Float
//         )?;
//         let right_float = cast_literal_to_type(
//             right_value_node.clone(), TypeSpecifier::Float
//         )?;
//         match (left_float.data.clone(), right_float.data.clone()) {
//             (Value::Float(left_float), Value::Float(right_float)) => {
//                 let result = safe_float_subtract(left_float, right_float, common_span)?;
//                 Ok(Node {
//                     sp: common_span,
//                     data: Value::Float(result),
//                 })
//             },
//             _ => {
//                 return Err(SemanticError::UnexpectedTypeCast(
//                     UnexpectedTypeCastError::init(
//                         common_span,
//                         format!(
//                             "In perform_minus_operation, cast to float of {:?} or {:?} failed", 
//                             left_float.data, right_float.data
//                         ).as_str(),
//                     )
//                 ));
//             },
//         }
//     } else {
//         let left_int = cast_literal_to_type(
//             left_value_node.clone(), TypeSpecifier::Int
//         )?;
//         let right_int = cast_literal_to_type(
//             right_value_node.clone(), TypeSpecifier::Int
//         )?;
//         match (left_int.data, right_int.data) {
//             (Value::Int(left_int), Value::Int(right_int)) => {
//                 let result = safe_int_subtract(left_int, right_int, common_span)?;
//                 Ok(Node {
//                     sp: common_span,
//                     data: Value::Int(result),
//                 })
//             },
//             (left, right) => {
//                 return Err(SemanticError::UnexpectedTypeCast(
//                     UnexpectedTypeCastError::init(
//                         common_span,
//                         format!(
//                             "In perform_minus_operation, cast to int of {:?} or {:?} failed", 
//                             left, right
//                         ).as_str(),
//                     )
//                 ));
//             },
//         }
//     }
// }

pub fn perform_binary_operation<'a>(
    left_value_node: &Node<'a, Value>,
    right_value_node: &Node<'a, Value>,
    operator: &BinaryOperator,
) -> Result<Node<'a, Value>, SemanticError> {
    match operator {
        // basic binary operations
        BinaryOperator::Plus => {
            basic_binary_operation!(
                "+",
                safe_float_add,
                safe_int_add,
                left_value_node,
                right_value_node,
                i16
            )
        },
        BinaryOperator::Minus => {
            basic_binary_operation!(
                "-",
                safe_float_subtract,
                safe_int_subtract,
                left_value_node,
                right_value_node,
                i16
            )
        },
        BinaryOperator::Multiply => {
            basic_binary_operation!(
                "*",
                safe_float_multiply,
                safe_int_multiply,
                left_value_node,
                right_value_node,
                i16
            )
        },
        BinaryOperator::Divide => {
            basic_binary_operation!(
                "/",
                safe_float_divide,
                safe_int_divide,
                left_value_node,
                right_value_node,
                i16
            )
        },
        // modulo binary operation
        BinaryOperator::Modulo => todo!(),
        // comparison binary operations
        BinaryOperator::Less => todo!(),
        BinaryOperator::Greater => todo!(),
        BinaryOperator::LessOrEqual => todo!(),
        BinaryOperator::GreaterOrEqual => todo!(),
        BinaryOperator::Equal => todo!(),
        BinaryOperator::NotEqual => todo!(),
        // logical binary operations
        BinaryOperator::LogicalAnd => todo!(),
        BinaryOperator::LogicalOr => todo!(),
    }
}