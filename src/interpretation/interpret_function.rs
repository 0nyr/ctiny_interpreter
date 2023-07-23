use crate::abstract_syntax_tree::nodes::{Node, Declaration, Value, Expression, Identifier, UnaryOperator, TypeSpecifier, Function, TranslationUnit};
use crate::semantic::errors::{SemanticError, UnexpectedExpressionParsingError, SemanticErrorTrait, UndeclaredFunctionError};
use crate::semantic::operations::perform_binary_operation;
use crate::semantic::type_casts::cast_to_type;
use crate::symbol_table::structs::{SymbolTable, Variable, NormalVarData, ArrayVarData};

use super::interpret_declaration::interpret_declaration;
use super::interpret_expression::interpret_expression;
use super::interpret_statement::interpret_statement;

/// This function interprets a function expression.
/// Do not confuse this function with function call. 
/// This function is not responsible for calling the function or setting its arguments.
pub fn interpret_function<'a>(
    function_node: &Node<'a, Function<'a>>,
    symbol_table: &mut SymbolTable,
    translation_unit: &TranslationUnit<'a>,
) -> Result<Node<'a, Value>, SemanticError> {

    // get function scope
    let function_scope_id_node = &function_node.data.name;

    // interpret function body
    let function_body = &function_node.data.body;
    for declaration in &function_body.data.declarations {
        interpret_declaration(
            &declaration, 
            symbol_table, 
            function_scope_id_node
        )?;
    }
    for statement in &function_body.data.statements {
        interpret_statement(
            &statement, 
            symbol_table, 
            function_scope_id_node,
            translation_unit,
        )?;
    }

    // interpret return
    let return_value_node = interpret_expression(
        &function_body.data.function_return, 
        symbol_table, 
        function_scope_id_node,
        translation_unit
    )?;
    let return_of_function_type: Node<'a, Value> = cast_to_type(
        return_value_node, 
        function_node.data.return_type
    )?;
    Ok(return_of_function_type)
}

/// This function interprets a program and return the value returned by the main function.
pub fn interpret_translation_unit<'a>(
    translation_unit: &Node<'a, TranslationUnit<'a>>,
    symbol_table: &mut SymbolTable,
) -> Result<Node<'a, Value>, SemanticError> {
    // interpret main function
    let main_function_node = &translation_unit.data.main_function;
    interpret_function(
        &main_function_node, 
        symbol_table,
        &translation_unit.data,
    )
}
    