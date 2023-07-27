use crate::abstract_syntax_tree::nodes::{Node, Declaration, Identifier};
use crate::semantic::errors::SemanticError;
use crate::symbol_table::structs::{SymbolTable, Variable, NormalVarData, ArrayVarData};

fn interpret_normal_declaration<'a>(
    declaration_node: &Node<'a, Declaration<'a>>,
    symbol_table: &mut SymbolTable,
    current_scope_node_id: &Node<'a, Identifier>,
) -> Result<(), SemanticError> {
    let declaration = &declaration_node.data;
    let identifier_node = &declaration.identifier;

    // Just add the variable to the symbol table.
    let current_scope = symbol_table.get_mut_scope(current_scope_node_id).unwrap();
    let normal_var = Variable::NormalVar(
        NormalVarData::new(
            identifier_node.data.clone(),
            declaration.type_specifier,
        )
    );
    current_scope.add_variable(Node {
        sp: identifier_node.sp.clone(),
        data: normal_var,
    })?;
    Ok(())
}

fn interpret_array_declaration<'a>(
    declaration_node: &Node<'a, Declaration<'a>>,
    symbol_table: &mut SymbolTable,
    current_scope_node_id: &Node<'a, Identifier>,
    size: usize,
) -> Result<(), SemanticError> {
    let declaration = &declaration_node.data;
    let identifier_node = &declaration.identifier;

    // Just add the variable to the symbol table.
    let current_scope = symbol_table.get_mut_scope(current_scope_node_id).unwrap();
    let array_var = Variable::ArrayVar(
        ArrayVarData::new(
            identifier_node.data.clone(),
        declaration.type_specifier,
        size,
        )
    );
    current_scope.add_variable(Node {
        sp: identifier_node.sp.clone(),
        data: array_var,
    })?;
    Ok(())
}

pub fn interpret_declaration<'a>(
    declaration_node: &Node<'a, Declaration<'a>>,
    symbol_table: &mut SymbolTable,
    current_scope_node_id: &Node<'a, Identifier>,
) -> Result<(), SemanticError> {
    let declaration = &declaration_node.data;

    // we need to know if we are declaring a normal variable or an array
    match declaration.array_size {
        Some(size) => {
            // we are declaring an array
            interpret_array_declaration(declaration_node, symbol_table, current_scope_node_id, size)
        },
        None => {
            // we are declaring a normal variable
            interpret_normal_declaration(declaration_node, symbol_table, current_scope_node_id)
        },
    }
}