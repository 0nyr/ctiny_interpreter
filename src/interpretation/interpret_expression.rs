use crate::abstract_syntax_tree::nodes::{Node, Value, Expression, UnaryExpression, BinaryExpression, FunctionCall, TypeCast, GetOrSetValue, Identifier, UnaryOperator, TypeSpecifier, BinaryOperator};
use crate::errors::make_semantic_error;
use crate::merge_spans_no_check;
use crate::semantic_analysis::errors::{SemanticError, UnexpectedExpressionParsingError, SemanticErrorTrait, UnexpectedTypeCastError};
use crate::semantic_analysis::operations::perform_binary_operation;
use crate::semantic_analysis::type_casts::cast_literal_to_type;
use crate::symbol_table::structs::{SymbolTable, Variable, NormalVarData, ArrayVarData};

fn interpret_potential_index<'a>(
    potential_index: &Option<Box<Node<'a, Expression<'a>>>>,
    symbol_table: &SymbolTable,
    current_scope_node_id: &Node<'a, Identifier>,
) -> Option<Node<'a, Value>> {
    match potential_index {
        Some(index) => {
            let interpreted_index = interpret_expression(
                &index, symbol_table, current_scope_node_id
            );
            match interpreted_index {
                Ok(interpreted_index) => Some(interpreted_index),
                Err(error) => {
                    panic!("Error interpreting index: {}", error);
                },
            }
        },
        None => None,
    }
}

fn interpret_get_value<'a>(
    expression_node: &Node<'a, Expression<'a>>, 
    symbol_table: &SymbolTable,
    current_scope_node_id: &Node<'a, Identifier>,
) -> Result<Node<'a, Value>, SemanticError> {
    let get_or_set_value = {
        match &expression_node.data {
            Expression::GetOrSetValue(get_or_set_value) => {
                get_or_set_value
            },
            _ => {
                return Err(SemanticError::UnexpectedExpressionParsing(
                    UnexpectedExpressionParsingError::init(
                        expression_node.sp,
                        format!(
                            "interpret_get_or_set_value called on a non GetOrSetValue expression: {:?}", 
                            expression_node.data
                        ).as_str(),
                    )
                ));
            },
        }
    };
    let identifier = &get_or_set_value.identifier;
    let potential_index = &get_or_set_value.index;
    let potential_index_value = interpret_potential_index(
        potential_index, symbol_table, current_scope_node_id
    );

    let current_scope = symbol_table.get_scope(current_scope_node_id).unwrap();
    current_scope.get_variable_value(identifier, potential_index_value)
}

fn interpret_type_cast<'a>(
    expression_node: &Node<'a, Expression<'a>>,
    symbol_table: &SymbolTable,
    current_scope_node_id: &Node<'a, Identifier>,
) -> Result<Node<'a, Value>, SemanticError> {
    let type_cast = {
        match &expression_node.data {
            Expression::TypeCast(type_cast) => {
                type_cast
            },
            _ => {
                return Err(SemanticError::UnexpectedExpressionParsing(
                    UnexpectedExpressionParsingError::init(
                        expression_node.sp,
                        format!(
                            "interpret_type_cast called on a non TypeCast expression: {:?}", 
                            expression_node.data
                        ).as_str(),
                    )
                ));
            },
        }
    };
    let target_type = type_cast.type_specifier;
    let interpreted_expression = interpret_expression(
        &type_cast.expression, symbol_table, current_scope_node_id
    )?;

    cast_literal_to_type(
        interpreted_expression, target_type
    )
}

fn interpret_unary_expression<'a>(
    expression_node: &Node<'a, Expression<'a>>,
    symbol_table: &SymbolTable,
    current_scope_node_id: &Node<'a, Identifier>,
) -> Result<Node<'a, Value>, SemanticError> {
    let unary_expression = {
        match &expression_node.data {
            Expression::UnaryExpression(unary_expression) => {
                unary_expression
            },
            _ => {
                return Err(SemanticError::UnexpectedExpressionParsing(
                    UnexpectedExpressionParsingError::init(
                        expression_node.sp,
                        format!(
                            "interpret_unary_expression called on a non UnaryExpression expression: {:?}", 
                            expression_node.data
                        ).as_str(),
                    )
                ));
            },
        }
    };
    let interpreted_expression = interpret_expression(
        &unary_expression.expression, symbol_table, current_scope_node_id
    )?;
    match unary_expression.operator {
        UnaryOperator::Negation => {
            match interpreted_expression.data {
                Value::Int(int) => {
                    Ok(Node {
                        sp: expression_node.sp,
                        data: Value::Int(-int),
                    })
                },
                Value::Float(float) => {
                    Ok(Node {
                        sp: expression_node.sp,
                        data: Value::Float(-float),
                    })
                },
                // negation of a char is just ignored
                Value::Char(_) => {
                    Ok(Node {
                        sp: expression_node.sp,
                        data: interpreted_expression.data,
                    })
                },
                // negation of a bool is just ignored
                Value::Bool(_) => {
                    Ok(Node {
                        sp: expression_node.sp,
                        data: interpreted_expression.data,
                    })
                },
            }
        },
        UnaryOperator::Not => {
            match interpreted_expression.data {
                Value::Bool(bool) => {
                    Ok(Node {
                        sp: expression_node.sp,
                        data: Value::Bool(!bool),
                    })
                },
                // if Value is not a bool, convert to bool and then negate
                not_bool_value => {
                    let casted_bool_value = cast_literal_to_type(
                        Node {
                            sp: expression_node.sp,
                            data: not_bool_value,
                        },
                        TypeSpecifier::Bool,
                    )?;
                    match casted_bool_value.data {
                        Value::Bool(bool) => {
                            Ok(Node {
                                sp: expression_node.sp,
                                data: Value::Bool(!bool),
                            })
                        },
                        unexpected_non_bool_value => {
                            panic!(
                                "In interpret_unary_expression, cast to bool of {:?} failed in ", 
                                unexpected_non_bool_value
                            );
                        },
                    }
                }
            }
        },
    }
}

fn interpret_binary_expression<'a>(
    expression_node: &Node<'a, Expression<'a>>,
    symbol_table: &SymbolTable,
    current_scope_node_id: &Node<'a, Identifier>,
) -> Result<Node<'a, Value>, SemanticError> {
    let binary_expression = {
        match &expression_node.data {
            Expression::BinaryExpression(binary_expression) => {
                binary_expression
            },
            _ => {
                return Err(SemanticError::UnexpectedExpressionParsing(
                    UnexpectedExpressionParsingError::init(
                        expression_node.sp,
                        format!(
                            "interpret_binary_expression called on a non BinaryExpression expression: {:?}", 
                            expression_node.data
                        ).as_str(),
                    )
                ));
            },
        }
    };
    let interpreted_left_expression = interpret_expression(
        &binary_expression.left, symbol_table, current_scope_node_id
    )?;
    let interpreted_right_expression = interpret_expression(
        &binary_expression.right, symbol_table, current_scope_node_id
    )?;
    perform_binary_operation(
        &interpreted_left_expression,
        &interpreted_right_expression,
        &binary_expression.operator,
    )
}




/// interpret an expression and return a value
pub fn interpret_expression<'a>(
    expression_node: &Node<'a, Expression<'a>>,
    symbol_table: &SymbolTable,
    current_scope_node_id: &Node<'a, Identifier>,
) -> Result<Node<'a, Value>, SemanticError> {
    match &expression_node.data {
        Expression::Literal(literal) => {
            Ok(Node {
                sp: expression_node.sp,
                data: literal.clone(),
            })
        },
        Expression::UnaryExpression(_) => {
            interpret_unary_expression(expression_node, symbol_table, current_scope_node_id)
        }
        Expression::BinaryExpression(binary_expression) => {
            interpret_binary_expression(expression_node, symbol_table, current_scope_node_id)
        }
        // Expression::FunctionCall(function_call) => {
        //     interpret_function_call(function_call, symbol_table)
        // }
        Expression::TypeCast(_) => {
            interpret_type_cast(expression_node, symbol_table, current_scope_node_id)
        }
        Expression::GetOrSetValue(_) => {
            // a GetOrSetValue evaluated as an expression is a GetValue operation
            interpret_get_value(expression_node, symbol_table, current_scope_node_id)
        }
        _ => {panic!("TODO: interpret_expression: {:?}", expression_node.data)}
    }
}