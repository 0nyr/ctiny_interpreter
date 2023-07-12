use pest::error::Error;
use pest::iterators::Pair;

use crate::syntax_parsing::Rule;

use super::declarations::build_parameter_list;
use super::declarations::build_multi_declaration;
use super::declarations::get_type_from_pair;
use super::expressions::identifier_from_pair;
use crate::abstract_syntax_tree::statements::build_multi_statement;
use super::nodes::*;

// exported macros are available in the crate root (global scope)
use crate::unwrap_or_err_panic;
use crate::ok_build_node;


pub fn build_block(pair: Pair<Rule>) -> Result<Node<Block>, Error<Rule>> {
    let mut declarations = Vec::new();
    let mut statements = Vec::new();

    for inner_pair in pair.clone().into_inner() {
        match inner_pair.as_rule() {
            Rule::multi_declaration => {
                declarations.extend(
                    unwrap_or_err_panic!(build_multi_declaration(inner_pair))
                );
            }
            Rule::multi_statement => {
                statements.extend(
                    unwrap_or_err_panic!(build_multi_statement(inner_pair))
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

pub fn build_function_definition(pair: Pair<Rule>) -> Result<Node<Function>, Error<Rule>> {
    let mut inner_pairs = pair.clone().into_inner();
    let first_pair = inner_pairs.next().unwrap();
    let second_pair = inner_pairs.next().unwrap();
    let third_pair = inner_pairs.next().unwrap();
    let potential_fourth_pair = inner_pairs.next();

    let type_specifier = unwrap_or_err_panic!(get_type_from_pair(first_pair));
    let identifier = unwrap_or_err_panic!(identifier_from_pair(second_pair));
    
    if let Some(actual_fourth_pair) = potential_fourth_pair {
        // if there is as fourth pair, then parse the parameters
        let parameters = unwrap_or_err_panic!(build_parameter_list(third_pair));
        let body = unwrap_or_err_panic!(build_block(actual_fourth_pair));
        return ok_build_node!(pair, 
            Function {
                name: identifier,
                return_type: type_specifier,
                params: Some(parameters),
                body: body,
            }
        );
    } else {
        // no parameters, so the third pair is the body
        let body = unwrap_or_err_panic!(build_block(third_pair));
        return ok_build_node!(pair,
            Function {
                name: identifier,
                return_type: type_specifier,
                params: None,
                body: body,
            }
        );
    }
}

pub fn build_entry_point_function(pair: Pair<Rule>) -> Result<Node<Function>, Error<Rule>> {
    let mut inner_pairs = pair.clone().into_inner();
    //log::info!("inner_pairs len: {:?}", inner_pairs.clone().count());
    //log::info!("pairs to string: {:?}", inner_pairs.clone().as_str());
    let first_pair = inner_pairs.next().unwrap();

    let identifier = Identifier { name: "main".to_string() };
    let body = unwrap_or_err_panic!(build_block(first_pair));

    ok_build_node!(pair, 
        Function {
            name: identifier,
            return_type: TypeSpecifier::Int,
            params: None,
            body: body,
        }
    )
}