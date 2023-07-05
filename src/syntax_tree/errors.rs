use pest::error::{Error, ErrorVariant};
use pest::Span;
use pest::iterators::Pair;

use crate::syntax_parsing::parser::Rule;

pub fn make_error(pair: Pair<Rule>, message: &str) -> Error<Rule> {
    //let span = Span::new(pair.as_str(), pair.as_span().start(), pair.as_span().end())
    //    .unwrap();  // The unwrap is safe as we are creating a span from a pair, which is guaranteed to be a valid span.
    let span = pair.as_span();
    Error::new_from_span(ErrorVariant::CustomError {
        message: message.to_string(),
    }, span)
}
