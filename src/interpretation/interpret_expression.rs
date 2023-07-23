use crate::abstract_syntax_tree::nodes::{Node, Value, Expression, Identifier, UnaryOperator, TypeSpecifier, TranslationUnit};
use crate::semantic::errors::{SemanticError, UnexpectedExpressionParsingError, SemanticErrorTrait, ArgumentNumberMismatchError};
use crate::semantic::operations::perform_binary_operation;
use crate::semantic::type_casts::cast_to_type;
use crate::symbol_table::structs::SymbolTable;

use super::interpret_function::interpret_function;

fn interpret_potential_index<'a>(
    potential_index: &Option<Box<Node<'a, Expression<'a>>>>,
    symbol_table: &mut SymbolTable,
    current_scope_node_id: &Node<'a, Identifier>,
    translation_unit: &TranslationUnit<'a>,
) -> Option<Node<'a, Value>> {
    match potential_index {
        Some(index) => {
            let interpreted_index = interpret_expression(
                &index, symbol_table, current_scope_node_id, translation_unit
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
    symbol_table: &mut SymbolTable,
    current_scope_node_id: &Node<'a, Identifier>,
    translation_unit: &TranslationUnit<'a>,
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
        potential_index, symbol_table, current_scope_node_id, translation_unit
    );

    let current_scope = symbol_table.get_scope(current_scope_node_id).unwrap();
    current_scope.get_variable_value(identifier, potential_index_value)
}

fn interpret_type_cast<'a>(
    expression_node: &Node<'a, Expression<'a>>,
    symbol_table: &mut SymbolTable,
    current_scope_node_id: &Node<'a, Identifier>,
    translation_unit: &TranslationUnit<'a>,
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
        &type_cast.expression, symbol_table, current_scope_node_id, translation_unit
    )?;

    cast_to_type(
        interpreted_expression, target_type
    )
}

fn interpret_unary_expression<'a>(
    expression_node: &Node<'a, Expression<'a>>,
    symbol_table: &mut SymbolTable,
    current_scope_node_id: &Node<'a, Identifier>,
    translation_unit: &TranslationUnit<'a>,
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
        &unary_expression.expression, symbol_table, current_scope_node_id, translation_unit
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
                    let casted_bool_value = cast_to_type(
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
    symbol_table: &mut SymbolTable,
    current_scope_node_id: &Node<'a, Identifier>,
    translation_unit: &TranslationUnit<'a>,
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
        &binary_expression.left, symbol_table, current_scope_node_id, translation_unit 
    )?;
    let interpreted_right_expression = interpret_expression(
        &binary_expression.right, symbol_table, current_scope_node_id, translation_unit
    )?;
    perform_binary_operation(
        &interpreted_left_expression,
        &interpreted_right_expression,
        &binary_expression.operator,
    )
}

fn interpret_function_call<'a>(
    function_call_node: &Node<'a, Expression<'a>>,
    symbol_table: &mut SymbolTable,
    current_scope_node_id: &Node<'a, Identifier>,
    translation_unit: &TranslationUnit<'a>,
) -> Result<Node<'a, Value>, SemanticError> {
    let function_call = {
        match &function_call_node.data {
            Expression::FunctionCall(function_call) => {
                function_call
            },
            _ => {
                return Err(SemanticError::UnexpectedExpressionParsing(
                    UnexpectedExpressionParsingError::init(
                        function_call_node.sp,
                        format!(
                            "interpret_function_call called on a non FunctionCall expression: {:?}", 
                            function_call_node.data
                        ).as_str(),
                    )
                ));
            },
        }
    };

    // check if function scope exists
    let function_id_node = function_call.name.clone();
    symbol_table.check_function_exists(&function_id_node)?;
    let function_scope = symbol_table.get_scope(&function_id_node).unwrap();

    // check that the number or arguments is correct
    let expected_number_of_arguments = function_scope.get_number_of_arguments();
    let actual_number_of_arguments = function_call.arguments.len();
    if expected_number_of_arguments != actual_number_of_arguments {
        return Err(SemanticError::ArgumentNumberMismatch(
            ArgumentNumberMismatchError::init(
                function_call_node.sp,
                format!(
                    "Expected {} arguments, got {} for function {}",
                    expected_number_of_arguments,
                    actual_number_of_arguments,
                    function_id_node.data.name,
                ).as_str()
            ),  
        ));
    }

    // interpret arguments and set them in the function scope
    for i in 0..expected_number_of_arguments {
        let current_expression = &function_call.arguments[i];
        let interpreted_expression = interpret_expression(
            current_expression, symbol_table, current_scope_node_id, translation_unit
        )?;
        // NOTE: functions can only have normal variables as arguments
        // This is because we don't have any pointers or references in our language.
        let function_scope_mut = symbol_table.get_mut_scope(&function_id_node).unwrap();
        let current_argument_id = function_scope_mut.get_argument_id(i);
        let current_argument_id_node = Node {
            sp: interpreted_expression.sp,
            data: current_argument_id,
        };
        function_scope_mut.set_normal_variable_value(
            &current_argument_id_node,
            interpreted_expression,
        )?;
    }

    // interpret function
    interpret_function(
        translation_unit.get_function_node(function_id_node)?,
        symbol_table,
        translation_unit,
    )
}



/// interpret an expression and return a value
pub fn interpret_expression<'a>(
    expression_node: &Node<'a, Expression<'a>>,
    symbol_table: &mut SymbolTable,
    current_scope_node_id: &Node<'a, Identifier>,
    translation_unit: &TranslationUnit<'a>, // for function calls
) -> Result<Node<'a, Value>, SemanticError> {
    match &expression_node.data {
        Expression::Literal(literal) => {
            Ok(Node {
                sp: expression_node.sp,
                data: literal.clone(),
            })
        },
        Expression::UnaryExpression(_) => {
            interpret_unary_expression(
                expression_node, 
                symbol_table, 
                current_scope_node_id, 
                translation_unit
            )
        }
        Expression::BinaryExpression(_) => {
            interpret_binary_expression(
                expression_node, 
                symbol_table, 
                current_scope_node_id, 
                translation_unit
            )
        }
        Expression::FunctionCall(_) => {
            interpret_function_call(
                expression_node, 
                symbol_table, 
                current_scope_node_id, 
                translation_unit
            )
        }
        Expression::TypeCast(_) => {
            interpret_type_cast(
                expression_node, 
                symbol_table, 
                current_scope_node_id, 
                translation_unit
            )
        }
        Expression::GetOrSetValue(_) => {
            // a GetOrSetValue evaluated as an expression is a GetValue operation
            interpret_get_value(
                expression_node, 
                symbol_table, 
                current_scope_node_id,
                translation_unit
            )
        }
    }
}