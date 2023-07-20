use crate::abstract_syntax_tree::nodes::Value;
use crate::tests::interpretation::interpret_expression_type_casts::interpret_expression_to_value_for_testing;
use crate::build_interpret_expression_to_value_test;

// Not operator tests
build_interpret_expression_to_value_test!(
    test_interpret_expression_unary_from_int_negative, 
    "!3",
    false,
    Bool
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_unary_from_int_positive, 
    "!0",
    true,
    Bool
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_unary_from_float_negative, 
    "!3.14159",
    false,
    Bool
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_unary_from_float_positive, 
    "!true",
    false,
    Bool
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_unary_from_char_negative, 
    "!'a'",
    false,
    Bool
);

// WARN: testing the string "!'\0'" is not possible since the grammar does not allow it
// but it can happen in the interpreter due to type casts

build_interpret_expression_to_value_test!(
    test_interpret_expression_unary_from_bool_negative, 
    "!true",
    false,
    Bool
);

build_interpret_expression_to_value_test!(
    test_interpret_expression_unary_from_bool_positive, 
    "!false",
    true,
    Bool
);