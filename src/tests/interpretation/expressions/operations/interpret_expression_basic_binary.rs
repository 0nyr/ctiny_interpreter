use crate::abstract_syntax_tree::nodes::Value;
use super::super::interpret_expression_type_casts::interpret_expression_to_value_for_testing;
use crate::build_interpret_expression_to_value_test;

// tests for + operator (Binary plus)
build_interpret_expression_to_value_test!(
    test_interpret_expression_binary_plus_from_int_int, 
    "3 + 4",
    7,
    Int
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_binary_plus_from_int_float, 
    "3 + 4.0",
    7.0,
    Float
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_binary_plus_from_int_char, 
    "3 + 'a'",
    100,
    Int
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_binary_plus_from_int_bool, 
    "3 + true",
    4,
    Int
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_binary_plus_from_float_float,
    "3.14159 + 4.0",
    7.14159,
    Float
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_binary_plus_from_char_char,
    "'a' + 'b'",
    97 + 98,
    Int
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_binary_plus_from_bool_bool,
    "true + false",
    1,
    Int
);

// tests for - operator (Binary minus)
build_interpret_expression_to_value_test!(
    test_interpret_expression_binary_minus_from_int_int, 
    "7 - 4",
    3,
    Int
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_binary_minus_from_int_float, 
    "7 - 4.0",
    3.0,
    Float
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_binary_minus_from_int_char, 
    "100 - 'a'",
    3,
    Int
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_binary_minus_from_int_bool, 
    "4 - true",
    3,
    Int
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_binary_minus_from_float_float,
    "7.14159 - 4.0",
    3.14159,
    Float
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_binary_minus_from_char_char,
    "'b' - 'a'",
    1,
    Int
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_binary_minus_from_bool_bool,
    "true - false",
    1,
    Int
);

// tests for * operator (Binary multiply)
build_interpret_expression_to_value_test!(
    test_interpret_expression_binary_multiply_from_int_int, 
    "3 * 4",
    12,
    Int
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_binary_multiply_from_int_float, 
    "3 * 4.0",
    12.0,
    Float
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_binary_multiply_from_int_char, 
    "3 * 'a'",
    291,
    Int
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_binary_multiply_from_int_bool, 
    "3 * true",
    3,
    Int
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_binary_multiply_from_float_float,
    "3.14159 * 3.0",
    9.42477,
    Float
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_binary_multiply_from_char_char,
    "'a' * 'b'",
    9506,
    Int
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_binary_multiply_from_bool_bool,
    "true * false",
    0,
    Int
);

// tests for / operator (Binary divide)
build_interpret_expression_to_value_test!(
    test_interpret_expression_binary_divide_from_int_int, 
    "12 / 4",
    3,
    Int
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_binary_divide_from_int_float, 
    "12 / 4.0",
    3.0,
    Float
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_binary_divide_from_int_char, 
    "291 / 'a'",
    3,
    Int
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_binary_divide_from_int_bool, 
    "3 / true",
    3,
    Int
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_binary_divide_from_float_float,
    "9.42477 / 3.0",
    3.14159,
    Float
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_binary_divide_from_char_char,
    "'b' / 'a'",
    1,
    Int
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_binary_divide_from_bool_bool,
    "false / true",
    0,
    Int
);
