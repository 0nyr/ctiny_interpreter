use pest::Span;

use crate::abstract_syntax_tree::nodes::{Node, Value, Expression, UnaryExpression, BinaryExpression, FunctionCall, TypeCast, GetOrSetValue, Identifier, UnaryOperator, TypeSpecifier, BinaryOperator};
use crate::merge_spans_no_check;
use crate::semantic_analysis::errors::{SemanticError, SemanticErrorTrait, UnexpectedTypeCastError};
use crate::semantic_analysis::type_casts::cast_literal_to_type;

use super::errors::DivisionByZeroError;
use super::overflow_checks::*;

macro_rules! basic_binary_operation {
    ($operator_str:expr, $safe_float_operation:ident, $safe_int_operation:ident, $left_value_node:expr, $right_value_node:expr) => {{
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

macro_rules! comparison_operation {
    ($operator_str:expr, $compare_operator:tt, $left_value_node:expr, $right_value_node:expr) => {{
        let left_type = $left_value_node.data.as_type_specifier();
        let right_type = $right_value_node.data.as_type_specifier();
        let common_span = merge_spans_no_check!(
            $left_value_node.sp, $right_value_node.sp
        ).unwrap();

        // We choose to perform comparisons as floats if any of the operands are floats.
        if left_type == TypeSpecifier::Float || right_type == TypeSpecifier::Float {
            let left_float = cast_literal_to_type(
                $left_value_node.clone(), TypeSpecifier::Float
            )?;
            let right_float = cast_literal_to_type(
                $right_value_node.clone(), TypeSpecifier::Float
            )?;
            match (left_float.data.clone(), right_float.data.clone()) {
                (Value::Float(left_float), Value::Float(right_float)) => {
                    let result = left_float $compare_operator right_float;
                    Ok(Node {
                        sp: common_span,
                        data: Value::Bool(result),
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
                    let result = left_int $compare_operator right_int;
                    Ok(Node {
                        sp: common_span,
                        data: Value::Bool(result),
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

/// Computes the modulo operation. Both operands must be integers.
/// So this function will cast any non-integer operand to integer first.
/// If the right operand is zero, a division by zero error is returned.
pub fn perform_modulo_operation<'a>(
    left_value_node: &Node<'a, Value>,
    right_value_node: &Node<'a, Value>,
) -> Result<Node<'a, Value>, SemanticError> {
    // cast left operand to int
    let left_casted = cast_literal_to_type(
        left_value_node.clone(), TypeSpecifier::Int
    )?.data;
    let left_int = match left_casted {
        Value::Int(left_int) => left_int,
        non_int => {
            return Err(SemanticError::UnexpectedTypeCast(
                UnexpectedTypeCastError::init(
                    left_value_node.sp,
                    format!(
                        "In perform_modulo_operation, cast to int of {:?} failed", 
                        non_int
                    ).as_str(),
                )
            ));
        },
    };

    // cast right operand to int
    let right_casted = cast_literal_to_type(
        right_value_node.clone(), TypeSpecifier::Int
    )?.data;
    let right_int = match right_casted {
        Value::Int(right_int) => right_int,
        non_int => {
            return Err(SemanticError::UnexpectedTypeCast(
                UnexpectedTypeCastError::init(
                    right_value_node.sp,
                    format!(
                        "In perform_modulo_operation, cast to int of {:?} failed", 
                        non_int
                    ).as_str(),
                )
            ));
        },
    };

    // check that right operand is not zero
    if right_int == 0 {
        return Err(SemanticError::DivisionByZero(
            DivisionByZeroError::init(
                right_value_node.sp,
                "In perform_modulo_operation, right operand is zero."
            )
        ));
    }

    // compute modulo
    let common_span = merge_spans_no_check!(
        left_value_node.sp, right_value_node.sp
    ).unwrap();
    let result = safe_int_modulo(left_int, right_int, common_span)?;
    Ok(Node {
        sp: common_span,
        data: Value::Int(result),
    })
}

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
                right_value_node
            )
        },
        BinaryOperator::Minus => {
            basic_binary_operation!(
                "-",
                safe_float_subtract,
                safe_int_subtract,
                left_value_node,
                right_value_node
            )
        },
        BinaryOperator::Multiply => {
            basic_binary_operation!(
                "*",
                safe_float_multiply,
                safe_int_multiply,
                left_value_node,
                right_value_node
            )
        },
        BinaryOperator::Divide => {
            basic_binary_operation!(
                "/",
                safe_float_divide,
                safe_int_divide,
                left_value_node,
                right_value_node
            )
        },
        // modulo binary operation
        BinaryOperator::Modulo => {
            perform_modulo_operation(
                left_value_node,
                right_value_node,
            )
        },
        // comparison binary operations
        BinaryOperator::Less => {
            comparison_operation!(
                "<",
                <,
                left_value_node,
                right_value_node
            )
        },
        BinaryOperator::Greater => {
            comparison_operation!(
                ">",
                >,
                left_value_node,
                right_value_node
            )
        },
        BinaryOperator::LessOrEqual => {
            comparison_operation!(
                "<=",
                <=,
                left_value_node,
                right_value_node
            )
        },
        BinaryOperator::GreaterOrEqual => {
            comparison_operation!(
                ">=",
                >=,
                left_value_node,
                right_value_node
            )
        },
        BinaryOperator::Equal => {
            comparison_operation!(
                "==",
                ==,
                left_value_node,
                right_value_node
            )
        },
        BinaryOperator::NotEqual => {
            comparison_operation!(
                "!=",
                !=,
                left_value_node,
                right_value_node
            )
        },
        // logical binary operations
        BinaryOperator::LogicalAnd => todo!(),
        BinaryOperator::LogicalOr => todo!(),
    }
}