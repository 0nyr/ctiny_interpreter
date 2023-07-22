use std::collections::HashMap;
use pest::{Parser, Span};

use crate::semantic::errors::SemanticError;
use crate::abstract_syntax_tree::nodes::{Value, Identifier, Node};
use crate::interpretation::interpret_expression::interpret_expression;
use crate::symbol_table::structs::{NormalVarData, Variable, Scope, SymbolTable};
use crate::abstract_syntax_tree::expressions::build_expression;
use crate::syntax_parsing::{CTinyParser, Rule};

fn interpret_expression_get_value_simple_var_for_testing<'a>(
    rule: Rule, 
    test_str: &'a str,
    test_value: Value,
) -> Result<Node<'a, Value>, SemanticError> {
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

    // for the need of the test, add a variable x to the main scope
    let mut main_scope_variables = HashMap::new();
    let x_var_id = Identifier {name: test_str.to_string()};
    let x_var = Variable::NormalVar(NormalVarData::new(
        x_var_id.clone(),
        test_value.as_type_specifier(),
    ));
    main_scope_variables.insert(x_var_id.clone(), x_var);

    let main_scope = Scope::new(
        main_scope_id_node.data.clone(),
        main_scope_variables,
    );
    symbol_table.add_scope(main_scope);

    // for the need of the test, set the value of the variable to provided literal
    let test_str_span = Span::new(&test_str, 0, test_str.len()).unwrap();
    let main_scope = symbol_table.get_mut_scope(&main_scope_id_node).unwrap();
    main_scope.set_normal_variable_value(
        &Node {
            sp: test_str_span.clone(),
            data: x_var_id.clone(),
        },
        Node {
            sp: test_str_span.clone(),
            data: test_value,
        },
    ).unwrap();

    // interpretation
    let interpreted_literal = interpret_expression(
        &expression_node,
        &symbol_table,
        &main_scope_id_node,
    );
    interpreted_literal
}

macro_rules! test_get_value_for_literal {
    ($literal_type:ident, $test_str:expr, $test_value:expr) => {

        let test_str = $test_str;
        let test_value = $test_value;
        let rule = Rule::get_or_set_value;

        // interpretation
        let interpreted_literal = interpret_expression_get_value_simple_var_for_testing(
            rule,
            test_str,
            Value::$literal_type(test_value),
        ).unwrap();

        // check and print
        match &interpreted_literal.data {
            Value::$literal_type(literal_value) => {
                assert_eq!(*literal_value, test_value);
                print!("Interpreted literal <{}>: {} of type {}\n\n", test_str, *literal_value, stringify!($literal_type));   
            },
            _ => panic!("Expected {} literal.", stringify!($literal_type)),
        }
    };
}

#[test]
fn test_interpret_expression_get_value_normal_int() {
    test_get_value_for_literal!(
        Int, 
        "x", 
        1
    );
}

#[test]
fn test_interpret_expression_get_value_normal_char() {
    test_get_value_for_literal!(
        Char, 
        "x", 
        b'a'
    );
}

#[test]
fn test_interpret_expression_get_value_normal_bool() {
    test_get_value_for_literal!(
        Bool, 
        "x", 
        true
    );
}

#[test]
fn test_interpret_expression_get_value_normal_float() {
    test_get_value_for_literal!(
        Float, 
        "x", 
        3.14159
    );
}