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

        impl From<Error<Rule>> for $error_type {
            fn from(error: Error<Rule>) -> Self {
                Self {
                    error,
                }
            }
        }

        impl fmt::Display for $error_type {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.get_error())
            }
        }
    };
}

macro_rules! define_and_implement_semantic_error {
    ($error_type:ident) => {
        #[derive(Clone, Debug, Eq, Hash, PartialEq)]
        pub struct $error_type {
            error: Error<Rule>,
        }
        impl_semantic_error!($error_type);
    };
}

#[derive(Debug)]
pub enum SemanticError {
    UndeclaredVariable(UndeclaredVariableError),
    UnexpectedExpressionParsing(UnexpectedExpressionParsingError),
    NegativeArrayIndex(NegativeArrayIndexError),
    UnexpectedLiteralType(UnexpectedLiteralTypeError),
    IntToCharCastOverflow(IntToCharCastOverflowError),
    FloatToCharCastOverflow(FloatToCharCastOverflowError),
    FloatToIntCastOverflow(FloatToIntCastOverflowError),
    ASTBuilding(ASTBuildingError), // not direct semantic error, but used in semantic analysis
}

define_and_implement_semantic_error!(UndeclaredVariableError);
define_and_implement_semantic_error!(UnexpectedExpressionParsingError);
define_and_implement_semantic_error!(NegativeArrayIndexError);
define_and_implement_semantic_error!(UnexpectedLiteralTypeError);
define_and_implement_semantic_error!(IntToCharCastOverflowError);
define_and_implement_semantic_error!(FloatToCharCastOverflowError);
define_and_implement_semantic_error!(FloatToIntCastOverflowError);
define_and_implement_semantic_error!(ASTBuildingError);

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SemanticError::UndeclaredVariable(error) => write!(f, "{}", error),
            SemanticError::UnexpectedExpressionParsing(error) => write!(f, "{}", error),
            SemanticError::NegativeArrayIndex(error) => write!(f, "{}", error),
            SemanticError::UnexpectedLiteralType(error) => write!(f, "{}", error),
            SemanticError::IntToCharCastOverflow(error) => write!(f, "{}", error),
            SemanticError::FloatToCharCastOverflow(error) => write!(f, "{}", error),
            SemanticError::FloatToIntCastOverflow(error) => write!(f, "{}", error),
            SemanticError::ASTBuilding(error) => write!(f, "{}", error),
        }
    }
}
