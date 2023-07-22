use super::interpret_statement_to_value_for_testing;
use crate::build_interpret_statement_to_value_test;


build_interpret_statement_to_value_test!(
    test_interpret_statement_if_empty_no_else,
    "if (true) { }"
);

build_interpret_statement_to_value_test!(
    test_interpret_statement_if_empty_else_empty,
    "if (true) { } else { }"
);

build_interpret_statement_to_value_test!(
    test_interpret_statement_if_nested_else,
    "if (true) { if (3 + 4 % (true + 'b' - 'a'*2)) {} } else {}"
);

build_interpret_statement_to_value_test!(
    test_interpret_statement_if_else_nested,
    "if (true) { if (3 + 4 % (true + 'b' - 'a'*2)) {} } else { if ('a' != 1) {} }"
);