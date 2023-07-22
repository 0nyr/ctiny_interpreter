use crate::abstract_syntax_tree::nodes::{Node, Value, Expression, Identifier, UnaryOperator, TypeSpecifier, Statement, AssignmentStatement};
use crate::semantic::errors::{SemanticError, UnexpectedExpressionParsingError, SemanticErrorTrait, UnexpectedStatementParsingError, UnexpectedTypeCastError};
use crate::semantic::operations::perform_binary_operation;
use crate::semantic::type_casts::cast_to_type;
use crate::symbol_table::structs::SymbolTable;

use super::interpret_expression::interpret_expression;

fn interpret_assignment_statement<'a>(
    statement_node: &Node<'a, Statement<'a>>,
    symbol_table: &mut SymbolTable,
    current_scope_node_id: &Node<'a, Identifier>,
) -> Result<(), SemanticError> {
    let assignment_statement = match &statement_node.data {
        Statement::Assignment(assignment_statement) => {
            assignment_statement
        },
        _ => {
            return Err(SemanticError::UnexpectedStatementParsing(
                UnexpectedStatementParsingError::init(
                    statement_node.sp,
                    format!(
                        "interpret_assignment_statement called on a non AssignmentStatement expression: {:?}", 
                        statement_node.data
                    ).as_str(),
                )
            ));
        },
    };

    // Interpret the expression on the right side of the assignment.
    let assignment_value_node = interpret_expression(
        &assignment_statement.right_expr, 
        symbol_table, 
        current_scope_node_id
    )?;

    let var_id_node = assignment_statement.left_var.data.identifier.clone();
    let potential_index_node = &assignment_statement.left_var.data.index;

    // We need to know if the assignment operation is on a normal variable or an array.
    if let Some(index_expr_node) = potential_index_node {
        // We want to assign a value into an array. We need to interpret the index-expression to get a usable index.
        let index_value_node = interpret_expression(
            &index_expr_node, symbol_table, current_scope_node_id
        )?;

        // try to set the value of the array
        // NOTE: after the use of immutable borrow for symbol_table, we need to use mutable borrow for current_scope
        let current_scope = symbol_table.get_mut_scope(current_scope_node_id).unwrap(); 
        return current_scope.set_array_variable_value(
            &var_id_node,
            index_value_node,
            assignment_value_node,
        );
    } else {
        // we are working with a normal variable. We can just set the value.
        let current_scope = symbol_table.get_mut_scope(current_scope_node_id).unwrap();
        return current_scope.set_normal_variable_value(
            &var_id_node,
            assignment_value_node,
        );
    }
}

fn interpret_if_statement<'a>(
    if_statement: &Node<'a, Statement<'a>>,
    symbol_table: &mut SymbolTable,
    current_scope_node_id: &Node<'a, Identifier>,
) -> Result<(), SemanticError> {
    let if_statement = match &if_statement.data {
        Statement::If(if_statement) => {
            if_statement
        },
        _ => {
            return Err(SemanticError::UnexpectedStatementParsing(
                UnexpectedStatementParsingError::init(
                    if_statement.sp,
                    format!(
                        "interpret_if_statement called on a non IfStatement expression: {:?}", 
                        if_statement.data
                    ).as_str(),
                )
            ));
        },
    };

    // first, interpret the condition
    let condition_value_node = interpret_expression(
        &if_statement.condition, 
        symbol_table,
        current_scope_node_id
    )?;
    let real_condition_value = cast_to_type(
        condition_value_node,
        TypeSpecifier::Bool
    )?;
    let real_condition = match real_condition_value.data {
        Value::Bool(bool_value) => bool_value,
        _ => {
            return Err(SemanticError::UnexpectedTypeCast(
                UnexpectedTypeCastError::init(
                real_condition_value.sp,
                format!(
                        "interpret_if_statement: expected bool, got {:?}",
                        real_condition_value.data
                    ).as_str(),
                )
            ));
        },
    };

    // then, interpret what should be done according to the condition
    if real_condition {
        // interpret the if block
        let if_body = &if_statement.if_body;
        for statement_node in if_body {
            interpret_statement(
                statement_node, 
                symbol_table, 
                current_scope_node_id
            )?;
        }
    } else {
        // interpret the else block if it exists
        let potential_else_body = &if_statement.else_body;
        if let Some(else_body) = potential_else_body {
            for statement_node in else_body {
                interpret_statement(
                    statement_node, 
                    symbol_table, 
                    current_scope_node_id
                )?;
            }
        }
    }

    // once all the statements have been interpreted, we can return
    Ok(())
}

/// Interpret a statement and returns a value as result.
pub fn interpret_statement<'a>(
    statement_node: &Node<'a, Statement<'a>>,
    symbol_table: &mut SymbolTable,
    current_scope_node_id: &Node<'a, Identifier>,
) -> Result<(), SemanticError> {
    match &statement_node.data {
        Statement::Assignment(_) => {
            interpret_assignment_statement(
                statement_node, 
                symbol_table, 
                current_scope_node_id
            )
        }
        Statement::If(_) => {
            interpret_if_statement(statement_node, symbol_table, current_scope_node_id)
        }
        // Statement::While(while_statement) => {
        //     interpret_while_statement(while_statement, symbol_table, current_scope_node_id)
        // }
        _ => {panic!("TODO: interpret_statement: {:?}", statement_node.data)}
    }
}