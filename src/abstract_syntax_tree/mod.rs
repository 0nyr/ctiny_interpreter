use pest::iterators::Pair;
use pest::error::Error;

use crate::abstract_syntax_tree::functions::{build_entry_point_function, build_function_definition};
use crate::{ok_build_node, unwrap_or_err_panic};
use crate::syntax_parsing::Rule;

use errors::make_ast_error;
use nodes::*;

pub mod nodes;
pub mod errors;
pub mod expressions;
pub mod statements;
pub mod declarations;
pub mod functions;

pub fn build_translation_unit(pair: Pair<Rule>) -> Result<AST, Error<Rule>> {
    let mut inner_pairs = pair.clone().into_inner();
    // print!("nb of inner pairs: {}\n", inner_pairs.clone().count());

    // // print str of the two pairs
    // for inner_pair in inner_pairs.clone() {
    //     print!("inner pair: {}\n", inner_pair.as_str());
    // }

    let mut successive_pair_non_empty = Vec::new();

    // get all non empty pairs
    while let Some(current_inner_pair) = inner_pairs.next() {
        // check that the pair is not empty
        if !current_inner_pair.as_str().is_empty() {
            successive_pair_non_empty.push(current_inner_pair);
        }
    }

    // the last non_empty added pair is the entry point function
    let last_valid_pair = {
        let last_pair = successive_pair_non_empty.pop();
        match last_pair {
            Some(pair) => {
                match pair.as_rule() {
                    Rule::entry_point_function_definition => pair,
                    _ => return Err(make_ast_error(
                        pair, 
                        "Last function must be the entry point function."
                    )),
                }
            }
            None => return Err(
                make_ast_error(pair, "Empty program not allowed. Missing main function.")
            ),
        }
    };
    let entry_point_function = unwrap_or_err_panic!(build_entry_point_function(last_valid_pair));

    // build other functions
    let functions = {
        if successive_pair_non_empty.is_empty() {
            None
        } else {
            let mut tmp_functions = Vec::new();
            for valid_pair in successive_pair_non_empty {
                tmp_functions.push(
                    unwrap_or_err_panic!(build_function_definition(valid_pair))
                );
            }
            Some(tmp_functions)
        }
    };
    
    ok_build_node!(pair,
        TranslationUnit {
            functions,
            main_function: entry_point_function 
        }
    )
}
