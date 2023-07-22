use crate::abstract_syntax_tree::nodes::Value;
use super::super::interpret_expression_type_casts::interpret_expression_to_value_for_testing;
use crate::build_interpret_expression_to_value_test;

// tests for < operator (Less than)
build_interpret_expression_to_value_test!(
    test_interpret_expression_less_from_int_int, 
    "3 < 4",
    true,
    Bool
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_less_from_float_int, 
    "4.5 < 5",
    true,
    Bool
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_less_from_char_char, 
    "'a' < 'b'",
    true,
    Bool
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_less_from_bool_bool, 
    "false < true",
    true,
    Bool
);

// tests for > operator (Greater than)
build_interpret_expression_to_value_test!(
    test_interpret_expression_greater_from_int_int, 
    "4 > 3",
    true,
    Bool
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_greater_from_float_int, 
    "5 > 4.5",
    true,
    Bool
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_greater_from_char_char, 
    "'b' > 'a'",
    true,
    Bool
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_greater_from_bool_bool, 
    "true > false",
    true,
    Bool
);

// tests for <= operator (Less than or equal to)
build_interpret_expression_to_value_test!(
    test_interpret_expression_less_or_equal_from_int_int, 
    "3 <= 4",
    true,
    Bool
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_less_or_equal_from_float_int, 
    "4.5 <= 5",
    true,
    Bool
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_less_or_equal_from_char_char, 
    "'a' <= 'b'",
    true,
    Bool
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_less_or_equal_from_bool_bool, 
    "false <= true",
    true,
    Bool
);

// tests for >= operator (Greater than or equal to)
build_interpret_expression_to_value_test!(
    test_interpret_expression_greater_or_equal_from_int_int, 
    "4 >= 3",
    true,
    Bool
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_greater_or_equal_from_float_int, 
    "5 >= 4.5",
    true,
    Bool
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_greater_or_equal_from_char_char, 
    "'b' >= 'a'",
    true,
    Bool
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_greater_or_equal_from_bool_bool, 
    "true >= false",
    true,
    Bool
);

// tests for == operator (Equal to)
build_interpret_expression_to_value_test!(
    test_interpret_expression_equal_from_int_int, 
    "4 == 4",
    true,
    Bool
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_equal_from_float_int, 
    "4.5 == 4.5",
    true,
    Bool
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_equal_from_char_char, 
    "'a' == 'a'",
    true,
    Bool
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_equal_from_bool_bool, 
    "true == true",
    true,
    Bool
);

// tests for != operator (Not equal to)
build_interpret_expression_to_value_test!(
    test_interpret_expression_not_equal_from_int_int, 
    "4 != 5",
    true,
    Bool
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_not_equal_from_float_int, 
    "4.5 != 5",
    true,
    Bool
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_not_equal_from_char_char, 
    "'a' != 'b'",
    true,
    Bool
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_not_equal_from_bool_bool, 
    "true != false",
    true,
    Bool
);
