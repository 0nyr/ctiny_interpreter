use pest::Span;

use crate::abstract_syntax_tree::nodes::{Node, Value, TypeSpecifier};
use crate::semantic_analysis::type_casts::cast_literal_to_type;


macro_rules! cast_literal_test {
    ($test_name:ident, $literal_to_convert:expr, $input_str:expr, $type_spec:expr, $expect:expr) => {
        #[test]
        fn $test_name() {
            let test_literal = $literal_to_convert;
            let input_literal_node = Node {
                sp: Span::new($input_str, 0, $input_str.len()).unwrap(),
                data: test_literal.clone(),
            };
            if $expect {
                assert_eq!(
                    $type_spec,
                    cast_literal_to_type(input_literal_node, $type_spec)
                        .unwrap().data.as_type_specifier()
                );
            } else {
                assert!(cast_literal_to_type(input_literal_node, $type_spec).is_err());
            }
        }
    };
}

// Trivial Correct Cases
cast_literal_test!(
    test_cast_literal_to_type_int_to_int,
    Value::Int(1),
    "1",
    TypeSpecifier::Int,
    true
);

cast_literal_test!(
    test_cast_literal_to_type_char_to_char,
    Value::Char(b'a'),
    "'a'",
    TypeSpecifier::Char,
    true
);

cast_literal_test!(
    test_cast_literal_to_type_float_to_float,
    Value::Float(1.0),
    "1.0",
    TypeSpecifier::Float,
    true
);

cast_literal_test!(
    test_cast_literal_to_type_bool_to_bool,
    Value::Bool(true),
    "true",
    TypeSpecifier::Bool,
    true
);

// Non-trivial correct Cases
// for Int
cast_literal_test!(
    test_cast_literal_to_type_float_to_int,
    Value::Float(1.0),
    "1.0",
    TypeSpecifier::Int,
    true
);

cast_literal_test!(
    test_cast_literal_to_type_char_to_int,
    Value::Char(b'a'),
    "'a'",
    TypeSpecifier::Int,
    true
);

cast_literal_test!(
    test_cast_literal_to_type_bool_to_int,
    Value::Bool(true),
    "true",
    TypeSpecifier::Int,
    true
);

// for Char
cast_literal_test!(
    test_cast_literal_to_type_int_to_char,
    Value::Int(97),
    "97",
    TypeSpecifier::Char,
    true
);

cast_literal_test!(
    test_cast_literal_to_type_float_to_char,
    Value::Float(97.0),
    "97.0",
    TypeSpecifier::Char,
    true
);

cast_literal_test!(
    test_cast_literal_to_type_bool_to_char,
    Value::Bool(true),
    "true",
    TypeSpecifier::Char,
    true
);

// for Float
cast_literal_test!(
    test_cast_literal_to_type_int_to_float,
    Value::Int(1),
    "1",
    TypeSpecifier::Float,
    true
);

cast_literal_test!(
    test_cast_literal_to_type_char_to_float,
    Value::Char(b'a'),
    "'a'",
    TypeSpecifier::Float,
    true
);

cast_literal_test!(
    test_cast_literal_to_type_bool_to_float,
    Value::Bool(true),
    "true",
    TypeSpecifier::Float,
    true
);

// for Bool
cast_literal_test!(
    test_cast_literal_to_type_int_to_bool,
    Value::Int(1),
    "1",
    TypeSpecifier::Bool,
    true
);

cast_literal_test!(
    test_cast_literal_to_type_char_to_bool,
    Value::Char(b'a'),
    "'a'",
    TypeSpecifier::Bool,
    true
);

cast_literal_test!(
    test_cast_literal_to_type_float_to_bool,
    Value::Float(1.0),
    "1.0",
    TypeSpecifier::Bool,
    true
);

// Incorrect Cases
cast_literal_test!(
    test_overflow_when_casting_large_float_to_int,
    Value::Float(std::f32::MAX),
    "3.4028235e+38",
    TypeSpecifier::Int,
    false
);

cast_literal_test!(
    test_overflow_when_casting_large_int_to_char,
    Value::Int(i16::MAX),
    "32767",
    TypeSpecifier::Char,
    false
);

cast_literal_test!(
    test_overflow_when_casting_large_float_to_char,
    Value::Float(std::f32::MAX),
    "3.4028235e+38",
    TypeSpecifier::Char,
    false
);







#[test]
fn test_cast_literal_to_type_int_manual() {
    // Correct Cases

    // Case: Casting Int to Int
    let test_literal = Value::Int(1);
    let input_literal_node = Node {
        sp: Span::new("1", 0, 1).unwrap(),
        data: test_literal.clone(),
    };
    assert_eq!(
        cast_literal_to_type(input_literal_node, TypeSpecifier::Int).unwrap().data,
        test_literal
    );

    // Incorrect Cases

    // Case: Overflow when casting large Float to Int
    let input_literal_node = Node {
        sp: Span::new("3.4028235e+38", 0, 12).unwrap(),
        data: Value::Float(std::f32::MAX as f32),
    };
    assert!(
        cast_literal_to_type(input_literal_node, TypeSpecifier::Int).is_err()
    );
}