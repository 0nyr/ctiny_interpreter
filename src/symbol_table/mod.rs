use std::collections::HashMap;

use crate::abstract_syntax_tree::nodes::{AST, Identifier, Declaration, Function};


use structs::*;

pub mod structs;

fn build_variable(declaration: &Declaration) -> Variable {
    if declaration.is_array() {
        Variable::ArrayVar(ArrayVarData::new(
            declaration.identifier.data.clone(),
            declaration.type_specifier,
            declaration.array_size.unwrap(),
        ))
    } else {
        Variable::NormalVar(NormalVarData::new(
            declaration.identifier.data.clone(),
            declaration.type_specifier,
        ))
    }
}

fn build_scope<'a>(scope_id: Identifier, scope_function: &'a Function<'a>) -> Scope {
    let mut scope_vars = HashMap::new();

    // add function parameters to the variables of the current scope
    if let Some(function_params) = &scope_function.params {
        for param in function_params {
            let current_declaration = &param.data;
            let current_var = build_variable(&current_declaration);
            scope_vars.insert(current_declaration.identifier.data.clone(), current_var);
        }
    }

    // keep argument identifiers in a separate vector
    let mut scope_args: Option<Vec<Identifier>> = None;
    if let Some(function_params) = &scope_function.params {
        scope_args = Some(Vec::new());
        for param in function_params {
            let current_declaration = &param.data;
            scope_args.as_mut().unwrap().push(current_declaration.identifier.data.clone());
        }
    }

    // add function variables to the variables of the current scope
    let block = &scope_function.body.data;
    for declaration in &block.declarations {
        let current_declaration = &declaration.data;
        let current_var = build_variable(&current_declaration);
        scope_vars.insert(current_declaration.identifier.data.clone(), current_var);
    }

    Scope::new(scope_id, scope_vars, scope_args)
}

pub fn build_static_symbol_table<'a>(ast: &AST<'a>) -> SymbolTable {
    let translation_unit = &ast.data;

    let mut symbol_table = SymbolTable::new();

    // in Ctiny, each function has a single scope
    // start by entry point function
    let current_scope_id = Identifier {name: "main".to_string()};
    let current_scope_function = &translation_unit.main_function.data;
    let main_scope = build_scope(current_scope_id, current_scope_function);
    symbol_table.add_scope(main_scope);

    // build the scopes of the other functions
    if let Some(functions) = &translation_unit.functions {
        for function in functions {
            let current_scope_id = function.data.name.data.clone();
            let current_scope_function = &function.data;
            let current_scope = build_scope(current_scope_id, current_scope_function);
            symbol_table.add_scope(current_scope);
        }
    }
    
    symbol_table
}