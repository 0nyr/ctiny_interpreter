use pest::error::Error;

use crate::syntax_parsing::parser::Rule;
use crate::syntax_tree::statements::{build_statement, multi_statement_vector_from_pair};

use super::nodes::*;
use super::errors::make_ast_error;