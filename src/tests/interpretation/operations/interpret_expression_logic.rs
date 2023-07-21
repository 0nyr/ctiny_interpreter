use crate::abstract_syntax_tree::nodes::Value;
use crate::tests::interpretation::interpret_expression_type_casts::interpret_expression_to_value_for_testing;
use crate::build_interpret_expression_to_value_test;

// LogicalAnd tests
build_interpret_expression_to_value_test!(
    logical_and_true_true,
    "true && true",
    true,
    Bool
);

build_interpret_expression_to_value_test!(
    logical_and_true_false,
    "true && false",
    false,
    Bool
);

build_interpret_expression_to_value_test!(
    logical_and_false_true,
    "false && true",
    false,
    Bool
);

build_interpret_expression_to_value_test!(
    logical_and_false_false,
    "false && false",
    false,
    Bool
);

// LogicalOr tests
build_interpret_expression_to_value_test!(
    logical_or_true_true,
    "true || true",
    true,
    Bool
);

build_interpret_expression_to_value_test!(
    logical_or_true_false,
    "true || false",
    true,
    Bool
);

build_interpret_expression_to_value_test!(
    logical_or_false_true,
    "false || true",
    true,
    Bool
);

build_interpret_expression_to_value_test!(
    logical_or_false_false,
    "false || false",
    false,
    Bool
);

// Testing complex type casts
build_interpret_expression_to_value_test!(
    logical_and_complex,
    "(1 > 0) && (1 == 1)",
    true,
    Bool
);

build_interpret_expression_to_value_test!(
    logical_and_complex_2,
    "(1 > 0) && (1 == 1) && (1 < 2)",
    true,
    Bool
);

build_interpret_expression_to_value_test!(
    logical_or_complex,
    "(1 < 0) || (1 == 1)",
    true,
    Bool
);

build_interpret_expression_to_value_test!(
    logical_or_complex_2,
    "(1 < 0) || (1 == 1) || (1 < 2)",
    true,
    Bool
);