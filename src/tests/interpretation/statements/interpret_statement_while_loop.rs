use super::interpret_statement_to_value_for_testing;
use crate::build_interpret_statement_to_value_test;


build_interpret_statement_to_value_test!(
    test_interpret_statement_while_no_iteration_no_body,
    "while (false) { }"
);

build_interpret_statement_to_value_test!(
    test_interpret_statement_while_no_iteration,
    "while (false) { if (true) {} }"
);

build_interpret_statement_to_value_test!(
    test_interpret_statement_while_infinite_loop,
    "while (true) { }",
    false
);

build_interpret_statement_to_value_test!(
    test_interpret_statement_while_infinite_loop_2,
    "while (true) { if (true) {} }",
    false
);