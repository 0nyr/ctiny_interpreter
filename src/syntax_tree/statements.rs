use pest::error::Error;

use crate::syntax_parsing::parser::Rule;
use crate::syntax_tree::expressions::build_expression;

use super::nodes::*;
use super::errors::make_ast_error;

// exported macros are available in the crate root (global scope)
use crate::unwrap_or_err_panic;
use crate::ok_build_node;

pub fn build_statement(pair: pest::iterators::Pair<Rule>) -> Result<Node<Statement>, Error<Rule>> {
    match pair.as_rule() {
        Rule::statement => build_statement(pair.into_inner().next().unwrap()),
        // Rule::assignment_statement => build_assignment_statement(pair),
        // Rule::if_statement => build_if_statement(pair),
        // Rule::while_statement => build_while_statement(pair),
        Rule::jump_statement => build_statement(pair.into_inner().next().unwrap()),
        Rule::return_statement => {
            let first_pair = pair.clone().into_inner().next().unwrap();
            let expression_node = build_expression(first_pair)?;
            ok_build_node!(pair, Statement::Jump(
                JumpStatement::Return(expression_node.data)
            ))
        },
        // Rule::break_statement => build_break_statement(pair),
        // Rule::continue_statement => build_continue_statement(pair),
        // Rule::empty_statement => build_empty_statement(pair),
        _ => Err(make_ast_error(
            pair.clone(), 
            format!("🔴 Unexpected rule inside <statement>: {:?}", pair.clone().as_rule()).as_str()
        )),
    }
}
