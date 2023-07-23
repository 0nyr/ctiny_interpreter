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
    UndeclaredFunction(UndeclaredFunctionError),
    ArgumentNumberMismatch(ArgumentNumberMismatchError),
    UnexpectedExpressionParsing(UnexpectedExpressionParsingError),
    UnexpectedStatementParsing(UnexpectedStatementParsingError),
    NegativeArrayIndex(NegativeArrayIndexError),
    UnexpectedLiteralType(UnexpectedLiteralTypeError),
    UnexpectedTypeCast(UnexpectedTypeCastError),
    DivisionByZero(DivisionByZeroError),

    // cast overflow 
    IntToCharCastOverflow(IntToCharCastOverflowError),
    FloatToCharCastOverflow(FloatToCharCastOverflowError),
    FloatToIntCastOverflow(FloatToIntCastOverflowError),

    // operation overflow 
    IntOverflow(IntOverflowError),
    FloatOverflow(FloatOverflowError),
    CharOverflow(CharOverflowError),
    BoolOverflow(BoolOverflowError),

    // loop
    MaxLoopIteration(MaxLoopIterationError),

    // inherited from previous errors
    ASTBuilding(ASTBuildingError), // not direct semantic error, but used in semantic analysis
    SyntaxParsing(SyntaxParsingError),

    // declaration
    Redeclaration(RedeclarationError),
}

define_and_implement_semantic_error!(UndeclaredVariableError);
define_and_implement_semantic_error!(UndeclaredFunctionError);
define_and_implement_semantic_error!(ArgumentNumberMismatchError);
define_and_implement_semantic_error!(UnexpectedExpressionParsingError);
define_and_implement_semantic_error!(NegativeArrayIndexError);
define_and_implement_semantic_error!(UnexpectedLiteralTypeError);
define_and_implement_semantic_error!(IntToCharCastOverflowError);
define_and_implement_semantic_error!(FloatToCharCastOverflowError);
define_and_implement_semantic_error!(FloatToIntCastOverflowError);
define_and_implement_semantic_error!(ASTBuildingError);
define_and_implement_semantic_error!(UnexpectedTypeCastError);
define_and_implement_semantic_error!(IntOverflowError);
define_and_implement_semantic_error!(FloatOverflowError);
define_and_implement_semantic_error!(CharOverflowError);
define_and_implement_semantic_error!(BoolOverflowError);
define_and_implement_semantic_error!(DivisionByZeroError);
define_and_implement_semantic_error!(UnexpectedStatementParsingError);
define_and_implement_semantic_error!(SyntaxParsingError);
define_and_implement_semantic_error!(MaxLoopIterationError);
define_and_implement_semantic_error!(RedeclarationError);

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SemanticError::UndeclaredVariable(error) => write!(f, "{}", error),
            SemanticError::UndeclaredFunction(error) => write!(f, "{}", error),
            SemanticError::ArgumentNumberMismatch(error) => write!(f, "{}", error),
            SemanticError::UnexpectedExpressionParsing(error) => write!(f, "{}", error),
            SemanticError::NegativeArrayIndex(error) => write!(f, "{}", error),
            SemanticError::UnexpectedLiteralType(error) => write!(f, "{}", error),
            SemanticError::IntToCharCastOverflow(error) => write!(f, "{}", error),
            SemanticError::FloatToCharCastOverflow(error) => write!(f, "{}", error),
            SemanticError::FloatToIntCastOverflow(error) => write!(f, "{}", error),
            SemanticError::ASTBuilding(error) => write!(f, "{}", error),
            SemanticError::UnexpectedTypeCast(error) => write!(f, "{}", error),
            SemanticError::IntOverflow(error) => write!(f, "{}", error),
            SemanticError::FloatOverflow(error) => write!(f, "{}", error),
            SemanticError::CharOverflow(error) => write!(f, "{}", error),
            SemanticError::BoolOverflow(error) => write!(f, "{}", error),
            SemanticError::DivisionByZero(error) => write!(f, "{}", error),
            SemanticError::UnexpectedStatementParsing(error) => write!(f, "{}", error),
            SemanticError::SyntaxParsing(error) => write!(f, "{}", error),
            SemanticError::MaxLoopIteration(error) => write!(f, "{}", error),
            SemanticError::Redeclaration(error) => write!(f, "{}", error),
        }
    }
}
