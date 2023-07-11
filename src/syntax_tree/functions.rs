use pest::error::Error;
use pest::iterators::Pairs;

use crate::syntax_parsing::parser::Rule;
use super::declarations::build_multi_declaration;
use crate::syntax_tree::expressions::identifier_from_pair;
use crate::syntax_tree::statements::{build_statement, multi_statement_vector_from_pair};

use super::nodes::*;
use super::errors::make_ast_error;

// exported macros are available in the crate root (global scope)
use crate::unwrap_or_err_panic;
use crate::ok_build_node;


pub fn build_block(pair: Pairs<Rule>) -> Result<Node<Block>, Error<Rule>> {
    let mut declarations = Vec::new();
    let mut statements = Vec::new();

    for inner_pair in pair {
        match inner_pair.as_rule() {
            Rule::multi_declaration => {
                declarations.extend(
                    unwrap_or_err_panic!(build_multi_declaration(inner_pair))
                );
            }
            Rule::multi_statement => {
                statements.extend(
                    unwrap_or_err_panic!(multi_statement_vector_from_pair(inner_pair))
                );
            }
            _ => unreachable!(),
        }
    }

    ok_build_node!(pair, Block {
        declarations,
        statements,
    })
}