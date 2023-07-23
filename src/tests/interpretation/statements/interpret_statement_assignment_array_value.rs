use std::collections::HashMap;
use pest::{Parser, Span};

use crate::abstract_syntax_tree::statements::build_statement;
use crate::interpretation::interpret_expression::interpret_expression;
use crate::interpretation::interpret_statement::interpret_statement;
use crate::semantic::errors::{SemanticError, ASTBuildingError};
use crate::abstract_syntax_tree::nodes::{Identifier, Node, Statement, TypeSpecifier, Value};
use crate::semantic::type_casts::get_index_value_from_value_node;
use crate::symbol_table::structs::{Scope, SymbolTable, Variable, ArrayVarData};
use crate::syntax_parsing::{CTinyParser, Rule};
use crate::tests::interpretation::statements::create_pseudo_translation_unit;

pub fn interpret_statement_assignment_array_var<'a>(
    test_str: &'a str,
    array_var_type: TypeSpecifier,
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

    let pseudo_translation_unit = create_pseudo_translation_unit();

    // add array variable to scope main
    let mut main_scope_variables = HashMap::new();
    
    let assignement = match &statement_node.data {
        Statement::Assignment(assignment_node) => assignment_node,
        _ => panic!("Expected assignment statement, got {:?}", statement_node.data),
    };
    let array_var_id = assignement.left_var.data.identifier.clone();
    
    let real_index = {
        let potential_index_node = &assignement.left_var.data.index;
        match potential_index_node {
            Some(index_node) => {
                let interpreted_index = interpret_expression(
                    &index_node,
                    &mut symbol_table,
                    &main_scope_id_node,
                    &pseudo_translation_unit,
                )?;
                let index_value = get_index_value_from_value_node(
                    interpreted_index
                )?;
                print!("Index value: {}\n", index_value);
                index_value
            },
            None => panic!("Expected index, got None"),
        }
    };
    
    let array_var = Variable::ArrayVar(
        ArrayVarData::new(
            array_var_id.data.clone(),
            array_var_type,
            // since it's a getter string, we need for the purpose of the test
            // to have an array size of the size of the index + 1 
            real_index + 1,             
        )
    );
    main_scope_variables.insert(array_var_id.data.clone(), array_var);

    let main_scope = Scope::new(
        main_scope_id_node.data.clone(),
        main_scope_variables,
        None,
    );
    symbol_table.add_scope(main_scope);

    // interpretation
    interpret_statement(
        &statement_node,
        &mut symbol_table,
        &main_scope_id_node,
        &pseudo_translation_unit,
    )?;

    // return the value of the variable at the index
    let main_scope = symbol_table.get_scope(&main_scope_id_node).unwrap();
    let array_var = main_scope.get_array_variable(&array_var_id).unwrap();
    let array_value = array_var.get_value(real_index).unwrap();
    Ok(array_value.clone())
}

macro_rules! build_test_assignment_to_array_value {
    ($test_name:ident, $literal_type:ident, $test_str:expr, $test_value:expr, $var_type:ident) => {
        // positive test
        #[test]
        fn $test_name() {
            let test_str = $test_str;
            let test_value = $test_value;
            let var_type = TypeSpecifier::$var_type;

            let result = interpret_statement_assignment_array_var(
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

build_test_assignment_to_array_value!(
    test_assignment_to_array_value_int_to_int,
    NormalVar,
    "x[0] = 42;",
    Value::Int(42),
    Int
);

build_test_assignment_to_array_value!(
    test_assignment_to_array_value_int_to_int_2,
    NormalVar,
    "x[10] = 42;",
    Value::Int(42),
    Int
);

// Assignments with index 0 and different integer values
build_test_assignment_to_array_value!(
    test_assignment_to_array_value_int_to_int_3,
    NormalVar,
    "x[0] = 84;",
    Value::Int(84),
    Int
);

build_test_assignment_to_array_value!(
    test_assignment_to_array_value_int_to_int_4,
    NormalVar,
    "x[0] = 168;",
    Value::Int(168),
    Int
);

// Assignments with different indices and the same integer value
build_test_assignment_to_array_value!(
    test_assignment_to_array_value_int_to_int_5,
    NormalVar,
    "x[1] = 42;",
    Value::Int(42),
    Int
);

build_test_assignment_to_array_value!(
    test_assignment_to_array_value_int_to_int_6,
    NormalVar,
    "x[2] = 42;",
    Value::Int(42),
    Int
);

// Assignments with different indices and different float values
build_test_assignment_to_array_value!(
    test_assignment_to_array_value_float_to_float_1,
    NormalVar,
    "x[0] = 3.14;",
    Value::Float(3.14),
    Float
);

build_test_assignment_to_array_value!(
    test_assignment_to_array_value_float_to_float_2,
    NormalVar,
    "x[1] = 2.718;",
    Value::Float(2.718),
    Float
);

// Assignments with different indices and different boolean values
build_test_assignment_to_array_value!(
    test_assignment_to_array_value_bool_to_bool_1,
    NormalVar,
    "x[0] = true;",
    Value::Bool(true),
    Bool
);

build_test_assignment_to_array_value!(
    test_assignment_to_array_value_bool_to_bool_2,
    NormalVar,
    "x[1] = false;",
    Value::Bool(false),
    Bool
);

// Assignments with complex expressions on the right-hand side
build_test_assignment_to_array_value!(
    test_assignment_to_array_value_complex_expr_1,
    NormalVar,
    "x[0] = 2 * 3 + 5;",
    Value::Int(11),
    Int
);

build_test_assignment_to_array_value!(
    test_assignment_to_array_value_complex_expr_2,
    NormalVar,
    "x[1] = (5 > 2) && (3 < 4);",
    Value::Bool(true),
    Bool
);
