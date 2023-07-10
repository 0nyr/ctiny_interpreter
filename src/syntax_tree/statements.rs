use pest::error::Error;

use crate::syntax_parsing::parser::Rule;
use crate::syntax_tree::expressions::build_expression;
use crate::syntax_tree::expressions::get_or_set_value_from_pair;

use super::nodes::*;
use super::errors::make_ast_error;

// exported macros are available in the crate root (global scope)
use crate::unwrap_or_err_panic;
use crate::ok_build_node;

fn build_assignment_statement(pair: pest::iterators::Pair<Rule>) -> Result<Node<Statement>, Error<Rule>> {
    let mut inner_pairs = pair.clone().into_inner();
    let first_pair = inner_pairs.next().unwrap();
    let second_pair = inner_pairs.next().unwrap();

    let set_value_node = unwrap_or_err_panic!(get_or_set_value_from_pair(first_pair));
    let expression_node = unwrap_or_err_panic!(build_expression(second_pair));

    ok_build_node!(pair, Statement::Assignment(
        AssignmentStatement {
            set_value: set_value_node,
            expression: expression_node.data,
        }
    ))
}

pub fn multi_statement_vector_from_pair(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Statement>, Error<Rule>> {
    let mut statements = Vec::new();

    for inner_pair in pair.into_inner() {
        // check if the pair is empty
        if inner_pair.clone().into_inner().next().is_none() {
            continue;
        }

        let statement_node = unwrap_or_err_panic!(build_statement(inner_pair));
        statements.push(statement_node.data);
    }
    Ok(statements)
}

fn build_if_else_statement(pair: pest::iterators::Pair<Rule>) -> Result<Node<Statement>, Error<Rule>> {
    let mut inner_pairs = pair.clone().into_inner();
    let first_pair = inner_pairs.next().unwrap();
    let second_pair = inner_pairs.next().unwrap();
    let potential_third_pair = inner_pairs.next();

    let condition_expression = unwrap_or_err_panic!(build_expression(first_pair));
    let if_body_statements: Vec<Statement> = unwrap_or_err_panic!(multi_statement_vector_from_pair(second_pair));
    let else_body_statements: Option<Vec<Statement>> = match potential_third_pair {
        Some(third_pair) => {
            Some(unwrap_or_err_panic!(multi_statement_vector_from_pair(third_pair)))
        },
        None => None,
    };

    ok_build_node!(pair, Statement::If(
        IfStatement {
            condition: condition_expression.data,
            if_body: if_body_statements,
            else_body: else_body_statements,
        }
    ))
}

pub fn build_statement(pair: pest::iterators::Pair<Rule>) -> Result<Node<Statement>, Error<Rule>> {
    match pair.as_rule() {
        Rule::statement => {
            // need to handle the empty statement case
            build_statement(
                pair.into_inner().next()
                    .expect("🔴 Expected a pair inside <statement>. Maybe you are trying to call this function on an empty statement ';'?"
            ))
        },
        Rule::assignment_statement => build_assignment_statement(pair),
        Rule::if_else_statement => build_if_else_statement(pair),
        // Rule::while_statement => build_while_statement(pair),
        Rule::jump_statement => build_statement(pair.into_inner().next().unwrap()),
        Rule::return_statement => {
            let first_pair = pair.clone().into_inner().next().unwrap();
            let expression_node = build_expression(first_pair)?;
            ok_build_node!(pair, Statement::Jump(
                JumpStatement::Return(expression_node.data)
            ))
        },
        Rule::break_statement => {
            ok_build_node!(pair, Statement::Jump(
                JumpStatement::Break
            ))
        },
        Rule::continue_statement => {
            ok_build_node!(pair, Statement::Jump(
                JumpStatement::Continue
            ))
        },
        _ => Err(make_ast_error(
            pair.clone(), 
            format!("🔴 Unexpected rule inside <statement>: {:?}", pair.clone().as_rule()).as_str()
        )),
    }
}
