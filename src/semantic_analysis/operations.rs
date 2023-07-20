use crate::abstract_syntax_tree::nodes::{Node, Value, Expression, UnaryExpression, BinaryExpression, FunctionCall, TypeCast, GetOrSetValue, Identifier, UnaryOperator, TypeSpecifier, BinaryOperator};
use crate::errors::make_semantic_error;
use crate::merge_spans_no_check;
use crate::semantic_analysis::errors::{SemanticError, UnexpectedExpressionParsingError, SemanticErrorTrait, UnexpectedTypeCastError};
use crate::semantic_analysis::type_casts::cast_literal_to_type;
use crate::symbol_table::structs::{SymbolTable, Variable, NormalVarData, ArrayVarData};

use super::overflow_checks::{safe_int_add, safe_float_add};

pub fn perform_plus_operation<'a>(
    left_value_node: &Node<'a, Value>,
    right_value_node: &Node<'a, Value>,
) -> Result<Node<'a, Value>, SemanticError> {
    // perform addition. Respect order of evaluation of operands and return the result.
    // addition works really for all types, so just cast to int or float if necessary.
    // if one of the operands is a float, cast both to float and perform addition
    // if no operand is a float, cast both to int and perform addition
    let left_type = left_value_node.data.as_type_specifier();
    let right_type = right_value_node.data.as_type_specifier();
    let common_span = merge_spans_no_check!(
        left_value_node.sp, right_value_node.sp
    ).unwrap();

    if left_type == TypeSpecifier::Float || right_type == TypeSpecifier::Float {
        let left_float = cast_literal_to_type(
            left_value_node.clone(), TypeSpecifier::Float
        )?;
        let right_float = cast_literal_to_type(
            right_value_node.clone(), TypeSpecifier::Float
        )?;
        // check that the cast worked and perform addition
        match (left_float.data.clone(), right_float.data.clone()) {
            (Value::Float(left_float), Value::Float(right_float)) => {
                // perform addition (check for overflow)
                let result = safe_float_add(left_float, right_float, common_span)?;
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
                            "In perform_binaty_operation, cast to float of {:?} or {:?} failed", 
                            left_float.data, right_float.data
                        ).as_str(),
                    )
                ));
            },
        }
    } else {
        let left_int = cast_literal_to_type(
            left_value_node.clone(), TypeSpecifier::Int
        )?;
        let right_int = cast_literal_to_type(
            right_value_node.clone(), TypeSpecifier::Int
        )?;
        match (left_int.data, right_int.data) {
            (Value::Int(left_int), Value::Int(right_int)) => {
                // perform addition (check for overflow)
                let result = safe_int_add(left_int, right_int, common_span)?;
                Ok(Node {
                    sp: common_span,
                    data: Value::Int(result),
                })
            },
            _ => {
                return Err(SemanticError::UnexpectedTypeCast(
                    UnexpectedTypeCastError::init(
                        common_span,
                        format!(
                            "In perform_binaty_operation, cast to int of {:?} or {:?} failed", 
                            left_int.data, right_int.data
                        ).as_str(),
                    )
                ));
            },
        }
    }
}

pub fn perform_binaty_operation<'a>(
    left_value_node: &Node<'a, Value>,
    right_value_node: &Node<'a, Value>,
    operator: BinaryOperator,
) -> Result<Node<'a, Value>, SemanticError> {
    match operator {
        BinaryOperator::Plus => {
            perform_plus_operation(left_value_node, right_value_node) // TODO: test
        },
        BinaryOperator::Minus => todo!(),
        BinaryOperator::Multiply => todo!(),
        BinaryOperator::Divide => todo!(),
        BinaryOperator::Modulo => todo!(),
        BinaryOperator::Less => todo!(),
        BinaryOperator::Greater => todo!(),
        BinaryOperator::LessOrEqual => todo!(),
        BinaryOperator::GreaterOrEqual => todo!(),
        BinaryOperator::Equal => todo!(),
        BinaryOperator::NotEqual => todo!(),
        BinaryOperator::LogicalAnd => todo!(),
        BinaryOperator::LogicalOr => todo!(),
    }
}