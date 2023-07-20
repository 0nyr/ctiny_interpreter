use pest::{Parser, iterators::Pairs};

use crate::syntax_parsing::{CTinyParser, Rule};

fn print_tokens(pairs: Pairs<'_, Rule>) {
    let tokens = pairs.tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}

pub fn syntax_parsing_test(input_str: &str, rule: Rule) {
    // parse the input string
    let pairs = CTinyParser::parse(rule, input_str).unwrap();

    // check that the first pair is an the correct rule
    let first_pair = pairs.clone().next().unwrap();
    assert_eq!(first_pair.as_rule(), rule);

    // check that the first pair is the whole input string
    assert_eq!(first_pair.as_str(), input_str);
    print_tokens(pairs);
}

macro_rules! build_syntax_parsing_test {
    ($test_name:ident, $input_str:expr, $rule:expr) => {
        #[test]
        fn $test_name() {
            let input_str = $input_str;
            syntax_parsing_test(input_str, $rule)
        }
    };
}

// tests
build_syntax_parsing_test!(
    test_addition,
    "1773 + 1362",
    Rule::addition
);

build_syntax_parsing_test!(
    test_block_statement,
    "{ char c; c = 'a'; }",
    Rule::block
);

build_syntax_parsing_test!(
    test_empty_block_statement,
    "{}",
    Rule::block
);

build_syntax_parsing_test!(
    test_while_statement,
    "while (i > 0) {c = 'a';}",
    Rule::while_statement
);

build_syntax_parsing_test!(
    test_if_statement,
    "if (i > 0) {c = 'a';}",
    Rule::if_else_statement
);

build_syntax_parsing_test!(
    test_if_else_statement,
    "if (i > 0) {c = 'a';} else {c = 3; d = 4;}",
    Rule::if_else_statement
);

build_syntax_parsing_test!(
    test_variable_declaration,
    "int variable_name;",
    Rule::multi_declaration
);

build_syntax_parsing_test!(
    test_multiple_variable_declaration_with_array,
    "int variable_name[10], variable_name2, variable_name3[20];",
    Rule::multi_declaration
);

build_syntax_parsing_test!(
    test_multiple_variable_declaration,
    "float y, z;",
    Rule::multi_declaration
);

build_syntax_parsing_test!(
    test_function_definition,
    "int test_function(int a, int b) { return a + b; }",
    Rule::function_definition
);

build_syntax_parsing_test!(
    test_function_definition_array_parameter,
    "int test_function(int a[10]) { return a + b; }",
    Rule::function_definition
);

build_syntax_parsing_test!(
    test_main_function,
    "int main() { return 0; }",
    Rule::entry_point_function_definition
);

build_syntax_parsing_test!(
    test_basic_program,
    "int main() { return 0; }",
    Rule::translation_unit
);

build_syntax_parsing_test!(
    test_function_call,
    "test_function(1, 2, a, 2*3)",
    Rule::expression
);