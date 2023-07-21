use crate::abstract_syntax_tree::nodes::Value;
use crate::tests::interpretation::interpret_expression_type_casts::interpret_expression_to_value_for_testing;
use crate::build_interpret_expression_to_value_test;

// tests for % operator (Binary modulo)
build_interpret_expression_to_value_test!(
    test_interpret_expression_binary_modulo_from_int_int, 
    "10 % 3",
    1,
    Int
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_binary_modulo_from_int_float, 
    "10 % 4.0",
    2,
    Int
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_binary_modulo_from_int_char, 
    "10 % 'a'",
    10 % 97,
    Int
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_binary_modulo_from_int_bool, 
    "10 % true",
    0,
    Int
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_binary_modulo_from_float_float,
    "10.0 % 4.0",
    2,
    Int
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_binary_modulo_from_char_char,
    "'c' % 'a'",
    99 % 97,
    Int
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_binary_modulo_from_bool_bool_division_by_zero,
    "true % false"
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_binary_modulo_from_int_float_division_by_zero,
    "10 % 0.0"
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_binary_modulo_from_int_int_division_by_zero,
    "10 % 0"
);
