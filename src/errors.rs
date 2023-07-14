use pest::error::{Error, ErrorVariant};
use pest::iterators::Pair;

use crate::syntax_parsing::Rule;

#[macro_export]
macro_rules! unwrap_or_err_panic {
    ($func:expr) => {
        $func.unwrap_or_else(|error| { print!("{}\n", error); panic!(); })
    }
}

fn make_error(span: pest::Span, message: &str) -> Error<Rule> {
    Error::new_from_span(ErrorVariant::CustomError {
        message: message.to_string(),
    }, span)
}

const AST_ERROR_PREFIX: &str = "🔴 [AST building error]";
const SEMANTIC_ERROR_PREFIX: &str = "🔴 [Semantic error]";

pub fn make_ast_error_from_pair(pair: Pair<Rule>, message: &str) -> Error<Rule> {
    make_ast_error(pair.as_span(), message)
}

pub fn make_ast_error(span: pest::Span, message: &str) -> Error<Rule> {
    make_error(span, format!("{} {}", AST_ERROR_PREFIX, message).as_str())
}

pub fn make_semantic_error_from_pair(pair: Pair<Rule>, message: &str) -> Error<Rule> {
    make_semantic_error(pair.as_span(), message)
}

pub fn make_semantic_error(span: pest::Span, message: &str) -> Error<Rule> {
    make_error(span, format!("{} {}", SEMANTIC_ERROR_PREFIX, message).as_str())
}