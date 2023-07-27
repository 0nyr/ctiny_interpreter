use crate::params;

use crate::abstract_syntax_tree::nodes::AST;

use self::errors::SemanticError;

pub mod errors;
pub mod type_casts;
pub mod overflow_checks;
pub mod operations;