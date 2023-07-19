use std::collections::HashMap;
use pest::{Parser, Span};

use crate::semantic_analysis::errors::{SemanticError, ASTBuildingError};
use crate::abstract_syntax_tree::nodes::{Value, Identifier, Node, Expression};
use crate::interpretation::interpret_expression::interpret_expression;
use crate::symbol_table::structs::{Variable, Scope, SymbolTable, ArrayVarData};
use crate::abstract_syntax_tree::expressions::build_expression;
use crate::syntax_parsing::{CTinyParser, Rule};

fn interpret_expression_get_value_array_var_for_testing<'a>(
    rule: Rule, 
    test_str: &'a str,
    test_value: Value,
) -> Result<Node<'a, Value>, SemanticError> {
    // get requested index for the string. It's the only integer in the string.
    let get_index = test_str
        .split(|c: char| !c.is_ascii_digit())
        .filter(|s| !s.is_empty())
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    print!("Get index: {}\n", get_index);

    // since it's a getter string, we need for the purpose of the test 
    // to have an array size of the size of the index + 1
    let array_size = get_index + 1;
    print!("Array size: {}\n", array_size);

    // Syntax parsing
    let pairs = CTinyParser::parse(rule, test_str).unwrap();

    let first_pair = pairs.into_iter().next().unwrap();
    assert_eq!(first_pair.as_rule(), rule);
    assert_eq!(first_pair.as_str(), test_str);

    // AST conversion
    let expression_node = {
        match build_expression(first_pair) {
            // need to convert the AST Error into a Semantic Error
            Ok(expression_node) => expression_node,
            Err(error) => {
                print!("AST ERROR for {}: \n {}\n", test_str, error);
                return Err(
                    SemanticError::ASTBuilding(
                        ASTBuildingError::from(error)
                    )
                );
            },
        }
    };
    print!("AST for string \"{}\": \n {:#?} \n\n", test_str, expression_node);

    // for the need of the test, build a symbol table from scratch with one scope "main"
    let mut symbol_table = SymbolTable::new();
    let main_scope_id_node = Node {
        sp: Span::new(&test_str, 0, 1).unwrap(),
        data: Identifier {name: "main".to_string()},
    };

    // for the need of the test, add a variable x to the main scope
    let mut main_scope_variables = HashMap::new();
    
    let get_or_set_value_node = match &expression_node.data {
        Expression::GetOrSetValue(get_or_set_value) => get_or_set_value,
        expr => panic!("Expected a GetOrSetValue expression. Got instead {:?}", expr),
    };
    let x_var_id = get_or_set_value_node.identifier.data.clone();
    let x_var = Variable::ArrayVar(ArrayVarData::new(
        x_var_id.clone(),
        test_value.as_type_specifier(),
        array_size,
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
    main_scope.set_array_variable_value(
        &Node {
            sp: test_str_span.clone(),
            data: x_var_id.clone(),
        },
        Node {
            sp: test_str_span.clone(),
            data: Value::Int(get_index as i16),
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

macro_rules! test_get_value_for_array_var {
    ($test_name:ident, $literal_type:ident, $test_str:expr, $test_value:expr, $rule:expr, $expect:expr) => {
        #[test]
        fn $test_name() {
            let test_str = $test_str;
            let test_value = $test_value;
            let rule = $rule;

            // interpretation
            let interpreted_literal = interpret_expression_get_value_array_var_for_testing(
                rule,
                test_str,
                Value::$literal_type(test_value),
            );

            // check and print
            if $expect {
                // positive test
                match &interpreted_literal.unwrap().data {
                    Value::$literal_type(literal_value) => {
                        assert_eq!(*literal_value, test_value);
                        print!("Interpreted literal <{}>: {} of type {}\n\n", test_str, *literal_value, stringify!($literal_type));   
                    },
                    _ => panic!("Expected {} literal.", stringify!($literal_type)),
                }
            } else {
                // negative test
                assert!(interpreted_literal.is_err());
                // print error
                print!("Expected error: {}\n\n", interpreted_literal.unwrap_err());
            }
        }
    };
    ($test_name:ident, $literal_type:ident, $test_str:expr, $test_value:expr, $rule:expr) => {
        // default test is expected to be positive (i.e. doesn't expect any error)
        test_get_value_for_array_var!(
            $test_name, 
            $literal_type, 
            $test_str, 
            $test_value, 
            $rule, 
            true
        );
    };
}

// positive tests
test_get_value_for_array_var!(
    test_interpret_expression_get_value_array_int, 
    Int, 
    "x[1]", 
    1, 
    Rule::get_or_set_value
);

test_get_value_for_array_var!(
    test_interpret_expression_get_value_array_char, 
    Char, 
    "x[10]",
    b'a', 
    Rule::get_or_set_value
);

test_get_value_for_array_var!(
    test_interpret_expression_get_value_array_bool, 
    Bool, 
    "x[100]", 
    true, 
    Rule::get_or_set_value
);

test_get_value_for_array_var!(
    test_interpret_expression_get_value_array_float, 
    Float, 
    "x[1000]", 
    3.14159, 
    Rule::get_or_set_value
);

// negative tests
test_get_value_for_array_var!(
    test_interpret_expression_get_value_array_int_index_overflow,
    Int, 
    "x[32768]", // 32768 is the max value for an i16 + 1
    1, 
    Rule::get_or_set_value,
    false
);
