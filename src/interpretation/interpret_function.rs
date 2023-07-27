use crate::abstract_syntax_tree::nodes::{Node, Value, Function, TranslationUnit};
use crate::semantic::errors::SemanticError;
use crate::semantic::type_casts::cast_to_type;
use crate::symbol_table::structs::SymbolTable;

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
    // WARN: No need to interpret declarations here, this has been done by the symbol table creation.
    // for declaration in &function_body.data.declarations {
    //     interpret_declaration(
    //         &declaration, 
    //         symbol_table, 
    //         function_scope_id_node
    //     )?;
    // }
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

    // before returning, check that all variables have been assigned a value
    let function_scope = symbol_table.get_scope(function_scope_id_node).unwrap();
    function_scope.check_all_variables_have_been_assigned()?;

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
    