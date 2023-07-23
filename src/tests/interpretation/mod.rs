#[cfg(test)]
mod expressions;
#[cfg(test)]
mod statements;
#[cfg(test)]
mod interpret_declaration;
#[cfg(test)]
mod interpret_program;

use std::collections::HashMap;
use pest::Span;

use crate::abstract_syntax_tree::nodes::{Identifier, Node, TranslationUnit, TypeSpecifier, Value, Expression, Block, Function};
use crate::symbol_table::structs::{Scope, SymbolTable};

pub fn create_symbol_table_and_empty_main_scope(
    test_str: &str
) -> (SymbolTable, Node<Identifier>) {
    let mut symbol_table = SymbolTable::new();

    let main_scope_id_node = Node {
        sp: Span::new(&test_str, 0, test_str.len()).unwrap(),
        data: Identifier {name: "main".to_string()},
    };
    let main_scope = Scope::new(
        main_scope_id_node.data.clone(),
        HashMap::new(),
        None,
    );
    symbol_table.add_scope(main_scope);

    return (symbol_table, main_scope_id_node);
}

/// Create a basic translation unit (program) with a main function.
/// The main function returns 0.
pub fn create_pseudo_translation_unit() -> TranslationUnit<'static> {
    let pseudo_main_str = "main() { return 0; }";
    let pseudo_span = Span::new(&pseudo_main_str, 0, pseudo_main_str.len()).unwrap();
    let main_scope_id_node = Node {
        sp: pseudo_span,
        data: Identifier {name: "main".to_string()},
    };
    let pseudo_translation_unit = TranslationUnit {
        functions: None,
        main_function: Node {
            sp: pseudo_span,
            data: Function {
                name: main_scope_id_node.clone(),
                params: None,
                return_type: TypeSpecifier::Int,
                body: Node {
                    sp: pseudo_span,
                    data: Block {
                        declarations: Vec::new(),
                        statements: Vec::new(),
                        function_return: Node {
                            sp: pseudo_span,
                            data: Expression::Literal(
                                Value::Int(0)
                            ),
                        },
                    },
                },
            },
        }
    };
    pseudo_translation_unit
}