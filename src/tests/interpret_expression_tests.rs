use std::collections::HashMap;

use pest::{Parser, Span};

use crate::pipelines::parse_content_into_ast;
use crate::symbol_table::{build_static_symbol_table, self};
use crate::abstract_syntax_tree::nodes::{Statement, Literal, Identifier, TypeSpecifier, Node};
use crate::interpretation::interpret_expression::interpret_expression;
use crate::symbol_table::structs::{NormalVarData, Variable, Scope, SymbolTable};
use crate::abstract_syntax_tree::expressions::build_expression;
use crate::syntax_parsing::{CTinyParser, Rule};


#[test]
fn test_interpret_expression_literal() {
    let input = "
    int main () {
        int x;
        x = 1;
        return 0;
    }
    ";

    // syntax parsing
    let ast = parse_content_into_ast(input, None);
    let main_id_node = &ast.data.main_function.data.name;

    let symbol_table = build_static_symbol_table(&ast);
    let literal_1_statement_node = &ast.data.main_function.data.body.data.statements[0];
    let literal_1_statement = &literal_1_statement_node.data;

    let literal_1_expression = match literal_1_statement {
        Statement::Assignment(assign_statement) => &assign_statement.expression,
        other => panic!("Expected an assignment statement. Got instead {:?}", other)
    };

    // interpret the expression
    let interpreted_literal = interpret_expression(
        literal_1_expression,
        &symbol_table,
        &main_id_node,
    )
    .unwrap();

    // Check that the interpreted literal matches the value we expect.
    // In this case, we expect the literal to be 1.
    match &interpreted_literal.data {
        Literal::Int(literal_value) => assert_eq!(*literal_value, 1),
        _ => panic!("Expected an Int literal."),
    }

    // BONUS: check the presence of x in the main scope
    let main_scope: &Scope = symbol_table.get_scope(main_id_node).unwrap();
    let x_var = main_scope.get_variable_from_id(&Identifier {name: "x".to_string()}).unwrap();
    assert_eq!(x_var, &Variable::NormalVar(NormalVarData::new(
        Identifier {name: "x".to_string()},
        TypeSpecifier::Int,
    )));
}

#[test]
fn test_interpret_expression_literal_2() {
    // this test is a simpler version of the preceding test
    let test_str = "1";
    let rule = Rule::expression;

    // Syntax parsing
    let pairs = CTinyParser::parse(rule, test_str).unwrap();

    let first_pair = pairs.into_iter().next().unwrap();
    assert_eq!(first_pair.as_rule(), rule);
    assert_eq!(first_pair.as_str(), test_str);

    // AST conversion
    let expression_node = build_expression(first_pair)
        .unwrap_or_else(|error| { 
            print!("AST ERROR for {}: \n {}\n", test_str, error); 
            panic!(); 
        });
    print!("AST for string \"{}\": \n {:#?} \n\n", test_str, expression_node);

    // for the need of the test, build a symbol table from scratch with one scope "main"
    let mut symbol_table = SymbolTable::new();
    let main_scope_id_node = Node {
        sp: Span::new(&test_str, 0, 1).unwrap(),
        data: Identifier {name: "main".to_string()},
    };
    let main_scope = Scope::new(
        main_scope_id_node.data.clone(),
        HashMap::new(),
    );
    symbol_table.add_scope(main_scope);

    // interpretation
    let interpreted_literal = interpret_expression(
        &expression_node,
        &symbol_table,
        &main_scope_id_node,
    )
    .unwrap();

    // Check that the interpreted literal matches the value we expect.
    // In this case, we expect the literal to be 1.
    match &interpreted_literal.data {
        Literal::Int(literal_value) => assert_eq!(*literal_value, 1),
        _ => panic!("Expected an Int literal."),
    }
}

// TODO: do the same for Char, Bool, Float