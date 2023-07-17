use crate::symbol_table::build_static_symbol_table;
use crate::pipelines::parse_content_into_ast;
use crate::abstract_syntax_tree::nodes::{Identifier, TypeSpecifier};
use crate::symbol_table::structs::{Variable, NormalVarData, ArrayVarData, Scope};

#[test]
fn test_build_static_symbol_table() {
    let test_string = "
    int main () {
        int x;
        x = 1;
        return 0;
    }
    ";
    let ast = parse_content_into_ast(test_string, None);
    let main_id_node = &ast.data.main_function.data.name;

    let symbol_table = build_static_symbol_table(&ast);
    let main_scope: &Scope = symbol_table.get_scope(main_id_node).unwrap();
    let x_var = main_scope.get_variable_from_id(&Identifier {name: "x".to_string()}).unwrap();
    assert_eq!(x_var, &Variable::NormalVar(NormalVarData::new(
        Identifier {name: "x".to_string()},
        TypeSpecifier::Int,
    )));
}

#[test]
fn test_build_static_symbol_table_multi_functions() {
    let test_string = "
    int foo(int x) {
        return x;
    }
    int main () {
        int x;
        x = foo(1);
        return 0;
    }
    ";
    let ast = parse_content_into_ast(test_string, None);
    let main_id_node = &ast.data.main_function.data.name;
    let foo_id_node = &ast.data.functions.as_ref().unwrap()[0].data.name;

    let symbol_table = build_static_symbol_table(&ast);

    // check main scope
    let main_scope: &Scope = symbol_table.get_scope(main_id_node).unwrap();
    let x_var = main_scope.get_variable_from_id(&Identifier {name: "x".to_string()}).unwrap();
    assert_eq!(x_var, &Variable::NormalVar(NormalVarData::new(
        Identifier {name: "x".to_string()},
        TypeSpecifier::Int,
    )));
    
    // check foo scope
    let foo_scope: &Scope = symbol_table.get_scope(foo_id_node).unwrap();
    let foo_x_var = foo_scope.get_variable_from_id(&Identifier {name: "x".to_string()}).unwrap();
    assert_eq!(foo_x_var, &Variable::NormalVar(NormalVarData::new(
        Identifier {name: "x".to_string()},
        TypeSpecifier::Int,
    )));
}

#[test]
fn test_build_static_symbol_table_array() {
    let test_string = "
    int foo(int a[10]) {
        return a[0];
    }
    int main () {
        int x;
        int y[10];
        x = foo(y);
        return 0;
    }
    ";
    let ast = parse_content_into_ast(test_string, None);
    let main_id_node = &ast.data.main_function.data.name;
    let foo_id_node = &ast.data.functions.as_ref().unwrap()[0].data.name;

    let symbol_table = build_static_symbol_table(&ast);

    // check main scope
    let main_scope: &Scope = symbol_table.get_scope(main_id_node).unwrap();
    let x_var = main_scope.get_variable_from_id(&Identifier {name: "x".to_string()}).unwrap();
    assert_eq!(x_var, &Variable::NormalVar(NormalVarData::new(
        Identifier {name: "x".to_string()},
        TypeSpecifier::Int,
    )));
    let y_var = main_scope.get_variable_from_id(&Identifier {name: "y".to_string()}).unwrap();
    assert_eq!(y_var, &Variable::ArrayVar(ArrayVarData::new(
        Identifier {name: "y".to_string()},
        TypeSpecifier::Int,
        10,
    )));
    
    // check foo scope
    let foo_scope: &Scope = symbol_table.get_scope(foo_id_node).unwrap();
    let foo_a_var = foo_scope.get_variable_from_id(&Identifier {name: "a".to_string()}).unwrap();
    assert_eq!(foo_a_var, &Variable::ArrayVar(ArrayVarData::new(
        Identifier {name: "a".to_string()},
        TypeSpecifier::Int,
        10,
    )));
}