use pest::{Parser, iterators::Pairs};

use crate::syntax_parsing::parser::{CTinyParser, Rule};

#[cfg(test)]
mod macros;
#[cfg(test)]
mod ast_expression_tests;
#[cfg(test)]
mod ast_statement_tests;
#[cfg(test)]
mod ast_declaration_tests;


fn print_tokens(pairs: Pairs<'_, Rule>) {
    let tokens = pairs.tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}


#[test]
fn test_addition() {
    // to see test output: $ cargo test -- --nocapture
    let pairs = CTinyParser::parse(Rule::addition, "1773 + 1362").unwrap();
    
    // check that the first pair is an the correct addition
    let pair = pairs.clone().next().unwrap();
    assert_eq!(pair.as_rule(), Rule::addition);
    assert_eq!(pair.as_str(), "1773 + 1362");
    
    print_tokens(pairs);
}

// write a test to test the following: { char c; c = 'a'; }
#[test]
fn test_block_statement() {
    let test_string = "{ char c; c = 'a'; }";
    let pairs = CTinyParser::parse(Rule::block, test_string)
        .unwrap();
    
    let first_pair = pairs.clone().next().unwrap();
    assert_eq!(first_pair.as_rule(), Rule::block);
    assert_eq!(first_pair.as_str(), test_string);

    print_tokens(pairs);
}

// test empty block statement
#[test]
fn test_empty_block_statement() {
    let test_string = "{}";
    let pairs = CTinyParser::parse(Rule::block, test_string)
    .unwrap();
    
    let first_pair = pairs.clone().next().unwrap();
    assert_eq!(first_pair.as_rule(), Rule::block);
    assert_eq!(first_pair.as_str(), test_string);

    print_tokens(pairs);
}

// write a test to test the following: while (i > 0) {c = 'a';}
#[test]
fn test_while_statement() {
    let test_string = "while (i > 0) {c = 'a';}";
    let pairs = CTinyParser::parse(Rule::statement, test_string)
        .unwrap().next().unwrap()
        .into_inner();


    let pair = pairs.clone().next().unwrap();
    assert_eq!(pair.as_rule(), Rule::while_statement);
    assert_eq!(pair.as_str(), test_string);

    print_tokens(pairs);
}

#[test]
fn test_if_statement() {
    let test_string = "if (i > 0) {c = 'a';}";
    let pairs = CTinyParser::parse(Rule::statement, test_string)
        .unwrap().next().unwrap()
        .into_inner();

    let pair = pairs.clone().next().unwrap();
    assert_eq!(pair.as_rule(), Rule::if_else_statement);
    assert_eq!(pair.as_str(), test_string);

    print_tokens(pairs);
}

#[test]
fn test_if_else_statement() {
    let test_string = "if (i > 0) {c = 'a';} else {c = 3; d = 4;}";
    let pairs = CTinyParser::parse(Rule::statement, test_string)
        .unwrap().next().unwrap()
        .into_inner();

    let pair = pairs.clone().next().unwrap();
    assert_eq!(pair.as_rule(), Rule::if_else_statement);
    assert_eq!(pair.as_str(), test_string);

    print_tokens(pairs);
}

// test variable declaration
#[test]
fn test_variable_declaration() {
    let test_string = "int variable_name;";
    let pairs = CTinyParser::parse(Rule::multi_declaration, test_string)
        .unwrap();
    
    let first_pair = pairs.clone().next().unwrap();

    assert_eq!(first_pair.as_rule(), Rule::multi_declaration);
    assert_eq!(first_pair.as_str(), test_string);

    let second_pair = first_pair.into_inner().next().unwrap();
    assert_eq!(second_pair.as_rule(), Rule::declaration);
    assert_eq!(second_pair.as_str(), "int variable_name");

    print_tokens(pairs);
}

// test multiple variable declaration
#[test]
fn test_multiple_variable_declaration_with_array() {
    let test_string = "int variable_name[10], variable_name2, variable_name3[20];";
    let pairs = CTinyParser::parse(Rule::multi_declaration, test_string)
        .unwrap();
    
    let first_pair = pairs.clone().next().unwrap();

    assert_eq!(first_pair.as_rule(), Rule::multi_declaration);
    assert_eq!(first_pair.as_str(), test_string);

    print_tokens(pairs);
}

#[test]
fn test_multiple_variable_declaration() {
    let test_string = "float y, z;";
    let pairs = CTinyParser::parse(Rule::multi_declaration, test_string)
        .unwrap();
    
    let first_pair = pairs.clone().next().unwrap();

    assert_eq!(first_pair.as_rule(), Rule::multi_declaration);
    assert_eq!(first_pair.as_str(), test_string);

    print_tokens(pairs);
}

// write a test to test the following: int test_function(int a, int b) { return a + b; }
#[test]
fn test_function_definition() {
    let test_string = "int test_function(int a, int b) { return a + b; }";
    let pairs = CTinyParser::parse(Rule::function_definition, test_string)
        .unwrap();
    
    let first_pair = pairs.clone().next().unwrap();

    assert_eq!(first_pair.as_rule(), Rule::function_definition);
    assert_eq!(first_pair.as_str(), test_string);

    print_tokens(pairs);
}

#[test]
fn test_function_definition_array_parameter() {
    let test_string = "int test_function(int a[10]) { return a + b; }";
    let pairs = CTinyParser::parse(Rule::function_definition, test_string)
        .unwrap();
    
    let first_pair = pairs.clone().next().unwrap();

    assert_eq!(first_pair.as_rule(), Rule::function_definition);
    assert_eq!(first_pair.as_str(), test_string);

    print_tokens(pairs);
}

// test very simple main function
#[test]
fn test_main_function() {
    let test_string = "int main() { return 0; }";
    let pairs = CTinyParser::parse(Rule::entry_point_function_definition, test_string)
        .unwrap();
    
    let first_pair = pairs.clone().next().unwrap();

    assert_eq!(first_pair.as_rule(), Rule::entry_point_function_definition);
    assert_eq!(first_pair.as_str(), test_string);

    print_tokens(pairs);
}

// test very basic program
#[test]
fn test_basic_program() {
    let test_string = "int main() { return 0; }";
    let pairs = CTinyParser::parse(Rule::translation_unit, test_string)
        .unwrap();
    
    let first_pair = pairs.clone().next().unwrap();

    assert_eq!(first_pair.as_rule(), Rule::translation_unit);
    assert_eq!(first_pair.as_str(), test_string);

    print_tokens(pairs);
}

// test function call
// NOTE: to run this specific test: $ cargo test test_function_call -- --nocapture
#[test]
fn test_function_call() {
    let test_string = "test_function(1, 2, a, 2*3)";
    let pairs = CTinyParser::parse(Rule::expression, test_string)
        .unwrap();
    
    let first_pair = pairs.clone().next().unwrap();

    assert_eq!(first_pair.as_rule(), Rule::expression);
    assert_eq!(first_pair.as_str(), test_string);

    print_tokens(pairs);
}