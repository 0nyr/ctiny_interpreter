use crate::abstract_syntax_tree::nodes::{Node, Literal, Expression, UnaryExpression, BinaryExpression, FunctionCall, TypeCast, GetOrSetValue};
use crate::symbol_table::structs::{SymbolTable, Variable, NormalVarData, ArrayVarData};

/// interpret an expression and return a value
pub fn interpret_expression<'a>(
    expression_node: &Node<'a, Expression>,
    symbol_table: &SymbolTable,
) -> Literal {
    match &expression_node.data {
        Expression::Literal(literal) => literal.clone(),
        Expression::UnaryExpression(unary_expression) => {
            interpret_unary_expression(unary_expression, symbol_table)
        }
        Expression::BinaryExpression(binary_expression) => {
            interpret_binary_expression(binary_expression, symbol_table)
        }
        Expression::FunctionCall(function_call) => {
            interpret_function_call(function_call, symbol_table)
        }
        Expression::TypeCast(type_cast) => {
            interpret_type_cast(type_cast, symbol_table)
        },
        Expression::GetOrSetValue(get_or_set_value) => {
            interpret_get_or_set_value(get_or_set_value, symbol_table)
        }
    }
}