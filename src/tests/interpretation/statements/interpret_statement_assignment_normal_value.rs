use std::collections::HashMap;
use pest::{Parser, Span};

use crate::abstract_syntax_tree::statements::build_statement;
use crate::interpretation::interpret_statement::interpret_statement;
use crate::semantic::errors::{SemanticError, ASTBuildingError};
use crate::abstract_syntax_tree::nodes::{Identifier, Node, Statement, TypeSpecifier, Value};
use crate::symbol_table::structs::{Scope, SymbolTable, NormalVarData, Variable};
use crate::syntax_parsing::{CTinyParser, Rule};

pub fn interpret_statement_assignment_normal_var<'a>(
    test_str: &'a str,
    normal_var_type: TypeSpecifier,
) -> Result<Value, SemanticError> {
    let rule = Rule::statement;
    
    // Syntax parsing
    let pairs = CTinyParser::parse(rule, test_str).unwrap();

    let first_pair = pairs.into_iter().next().unwrap();
    assert_eq!(first_pair.as_rule(), rule);
    assert_eq!(first_pair.as_str(), test_str);

    // AST conversion
    let statement_node = {
        match build_statement(first_pair) {
            // need to convert the AST Error into a Semantic Error
            Ok(statement_node) => statement_node,
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
    print!("AST for string \"{}\": \n {:#?} \n\n", test_str, statement_node);

    // for the need of the test, build a symbol table from scratch with one scope "main"
    let mut symbol_table = SymbolTable::new();
    let main_scope_id_node = Node {
        sp: Span::new(&test_str, 0, test_str.len()).unwrap(),
        data: Identifier {name: "main".to_string()},
    };

    // add normal variable to scope main
    let mut main_scope_variables = HashMap::new();
    
    let assignement = match &statement_node.data {
        Statement::Assignment(assignment_node) => assignment_node,
        _ => panic!("Expected assignment statement, got {:?}", statement_node.data),
    };
    let normal_var_id = assignement.set_value.data.identifier.clone();
    let normal_var = Variable::NormalVar(
        NormalVarData::new(
            normal_var_id.data.clone(),
            normal_var_type,
        )
    );
    main_scope_variables.insert(normal_var_id.data.clone(), normal_var);

    let main_scope = Scope::new(
        main_scope_id_node.data.clone(),
        main_scope_variables,
    );
    symbol_table.add_scope(main_scope);

    // interpretation
    interpret_statement(
        &statement_node,
        &mut symbol_table,
        &main_scope_id_node,
    )?;

    // get value of the variable
    let value_of_var = symbol_table.get_scope(&main_scope_id_node)
        .unwrap()
        .get_normal_variable(&normal_var_id)
        .unwrap()
        .get_value()
        .unwrap();

    Ok(value_of_var.clone())
}


macro_rules! build_test_assignment_to_normal_value {
    ($test_name:ident, $literal_type:ident, $test_str:expr, $test_value:expr, $var_type:ident) => {
        // positive test
        #[test]
        fn $test_name() {
            let test_str = $test_str;
            let test_value = $test_value;
            let var_type = TypeSpecifier::$var_type;

            let result = interpret_statement_assignment_normal_var(
                test_str,
                var_type,
            );

            match result {
                Ok(value) => {
                    assert_eq!(value, test_value);
                    println!("Successfull interpretation of assignment statement <{}> to value {:?} of type {:?}",
                        test_str,
                        test_value,
                        var_type,
                    );
                },
                Err(error) => {
                    panic!("Unexpected error during interpretation of statement <{}>: {}",
                        test_str,
                        error,
                    );
                },
            }
        }
    };
}

build_test_assignment_to_normal_value!(
    test_assignment_to_normal_value_int_to_int,
    Int,
    "x = 42;",
    Value::Int(42),
    Int
);

build_test_assignment_to_normal_value!(
    test_assignment_to_normal_value_implicit_cast_int_to_float,
    Int,
    "x = 42;",
    Value::Float(42.0),
    Float
);

build_test_assignment_to_normal_value!(
    test_assignment_to_normal_value_arithmetic_expression,
    Int,
    "x = 42 + 2;",
    Value::Int(44),
    Int
);

build_test_assignment_to_normal_value!(
    test_assignment_to_normal_value_logical_expression,
    Bool,
    "x = true && false;",
    Value::Bool(false),
    Bool
);

build_test_assignment_to_normal_value!(
    test_assignment_to_normal_value_division_expression,
    Int,
    "x = 45 / 5;",
    Value::Int(9),
    Int
);

build_test_assignment_to_normal_value!(
    test_assignment_to_normal_value_multiple_arithmetic_expression,
    Int,
    "x = 2 * 3 + 5;",
    Value::Int(11),
    Int
);

build_test_assignment_to_normal_value!(
    test_assignment_to_normal_value_boolean_comparison_expression,
    Bool,
    "x = 5 > 2;",
    Value::Bool(true),
    Bool
);

build_test_assignment_to_normal_value!(
    test_assignment_to_normal_value_complex_boolean_expression,
    Bool,
    "x = (5 > 2) && (3 < 4);",
    Value::Bool(true),
    Bool
);

build_test_assignment_to_normal_value!(
    test_assignment_to_normal_value_implicit_cast_float_to_int,
    Float,
    "x = 42.0;",
    Value::Int(42),
    Int
);

build_test_assignment_to_normal_value!(
    test_assignment_to_normal_value_complex_mixed_expression,
    Bool,
    "x = (2 + 3 > 4) && (3 * 2 == 6);",
    Value::Bool(true),
    Bool
);
