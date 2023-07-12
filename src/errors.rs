use pest::error::{Error, ErrorVariant};
use pest::iterators::Pair;

use crate::syntax_parsing::Rule;

#[macro_export]
macro_rules! unwrap_or_err_panic {
    ($func:expr) => {
        $func.unwrap_or_else(|error| { print!("{}\n", error); panic!(); })
    }
}

fn make_error(pair: Pair<Rule>, message: &str) -> Error<Rule> {
    //let span = Span::new(pair.as_str(), pair.as_span().start(), pair.as_span().end())
    //    .unwrap();  // The unwrap is safe as we are creating a span from a pair, which is guaranteed to be a valid span.
    let span = pair.as_span();
    Error::new_from_span(ErrorVariant::CustomError {
        message: message.to_string(),
    }, span)
}

const AST_ERROR_PREFIX: &str = "🔴 [AST building error]";
const SEMANTIC_ERROR_PREFIX: &str = "🔴 [Semantic error]";

pub fn make_ast_error(pair: pest::iterators::Pair<Rule>, message: &str) -> Error<Rule> {
    make_error(pair, format!("{} {}", AST_ERROR_PREFIX, message).as_str())
}

pub fn make_semantic_error(pair: pest::iterators::Pair<Rule>, message: &str) -> Error<Rule> {
    make_error(pair, format!("{} {}", SEMANTIC_ERROR_PREFIX, message).as_str())
}