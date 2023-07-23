use pest::error::Error;

use crate::syntax_parsing::Rule;
use crate::abstract_syntax_tree::expressions::build_identifier;

use super::nodes::*;
use crate::errors::make_ast_error_from_pair;

// exported macros are available in the crate root (global scope)
use crate::unwrap_or_err_panic;
use crate::ok_build_node;


pub fn build_parameter_list(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Node<Declaration>>, Error<Rule>> {
    let mut parameters = Vec::new();

    for inner_pair in pair.into_inner() {
        let parameter_node = unwrap_or_err_panic!(build_declaration(inner_pair));
        
        // TODO: check that the parameter is not an array
        
        parameters.push(parameter_node);
    }
    Ok(parameters)
}

pub fn get_type_from_pair(pair: pest::iterators::Pair<Rule>) -> Result<TypeSpecifier, Error<Rule>> {
    let type_specifier = match TypeSpecifier::from_str(pair.clone().as_str()) {
        Some(type_specifier) => type_specifier,
        None => return Err(make_ast_error_from_pair(pair, "Invalid type specifier")),
    };
    Ok(type_specifier)
}

fn potential_array_size_from_pair(potential_pair: Option<pest::iterators::Pair<Rule>>) -> Result<Option<usize>, Error<Rule>> {
    match potential_pair {
        Some(actual_array_size_pair) => {
            let array_size_str = actual_array_size_pair.as_str();
            match array_size_str.parse::<usize>() {
                Ok(array_size) => Ok(Some(array_size)),
                Err(_) => return Err(make_ast_error_from_pair(
                    actual_array_size_pair, 
                    format!(
                        "🟠 Invalid array size (must be a positive integer): {}", 
                        array_size_str
                    ).as_str()
                )),
            }
        },
        None => Ok(None),
    }
}

pub fn build_declaration(pair: pest::iterators::Pair<Rule>) -> Result<Node<Declaration>, Error<Rule>> {
    let mut inner_pairs = pair.clone().into_inner();
    let first_pair = inner_pairs.next().unwrap();
    let second_pair = inner_pairs.next().unwrap();
    let potential_third_pair = inner_pairs.next();

    let declaration_type = unwrap_or_err_panic!(get_type_from_pair(first_pair));
    let identifier = unwrap_or_err_panic!(build_identifier(second_pair));
    let array_size: Option<usize> = unwrap_or_err_panic!(potential_array_size_from_pair(potential_third_pair));

    ok_build_node!(pair, Declaration {
        type_specifier: declaration_type,
        identifier,
        array_size,
    })
}

fn declaration_from_followup(
    pair: pest::iterators::Pair<Rule>,
    common_type: TypeSpecifier,
) -> Result<Node<Declaration>, Error<Rule>> {
    let mut inner_pairs = pair.clone().into_inner();
    let first_pair = inner_pairs.next().unwrap();
    let potential_array_pair = inner_pairs.next();
    
    let identifier = unwrap_or_err_panic!(build_identifier(first_pair));
    let array_size: Option<usize> = unwrap_or_err_panic!(potential_array_size_from_pair(potential_array_pair));
    
    ok_build_node!(pair, Declaration {
        type_specifier: common_type,
        identifier,
        array_size,
    })
}

pub fn build_multi_declaration(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Node<Declaration>>, Error<Rule>> {
    let mut declarations: Vec<Node<Declaration>> = Vec::new();

    // multiple declarations are separated by a comma
    // and we need to get their common type specifier from the first declaration
    let mut inner_pairs = pair.clone().into_inner();
    let first_pair = inner_pairs.next().unwrap();
    let first_declaration = unwrap_or_err_panic!(build_declaration(first_pair));
    let common_type = first_declaration.data.type_specifier;
    declarations.push(first_declaration);

    // iterate over the rest of the declarations
    for inner_pair in inner_pairs {
        let followup_declaration = unwrap_or_err_panic!(
            declaration_from_followup(inner_pair, common_type)
        );
        declarations.push(followup_declaration);
    }

    Ok(declarations)
}