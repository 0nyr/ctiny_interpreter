use pest::error::Error;
use pest::Span;
use std::fmt;

use crate::syntax_parsing::Rule;
use crate::errors::make_semantic_error;

pub trait SemanticErrorTrait {
    fn init<'a>(span: Span<'a>, message: &str) -> Self where Self: Sized;
    fn get_error(&self) -> Error<Rule>;
}

macro_rules! impl_semantic_error {
    ($error_type:ty) => {
        impl SemanticErrorTrait for $error_type {
            fn init<'a>(span: Span<'a>, message: &str) -> Self {
                Self {
                    error: make_semantic_error(span, message),
                }
            }

            fn get_error(&self) -> Error<Rule> {
                self.error.clone()
            }
        }

        impl fmt::Display for $error_type {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.error)
            }
        }
    };
}

pub enum SemanticError {
    UndeclaredVariable(UndeclaredVariableError),
}


#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct UndeclaredVariableError {
    error: Error<Rule>,
}
impl_semantic_error!(UndeclaredVariableError);
