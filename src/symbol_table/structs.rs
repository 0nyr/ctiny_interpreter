use std::collections::HashMap;

use crate::{abstract_syntax_tree::nodes::{Identifier, TypeSpecifier, Literal, Function}, semantic_analysis::errors::SemanticError};



pub enum Variable {
    NormalVar(NormalVarData),
    ArrayVar(ArrayVarData),
}

#[derive(Debug, PartialEq)]
pub struct NormalVarData {
    pub id: Identifier,
    pub type_specifier: TypeSpecifier,
    pub value: Option<Literal>,
}

#[derive(Debug, PartialEq)]
pub struct ArrayVarData {
    pub id: Identifier,
    pub type_specifier: TypeSpecifier,
    pub size: usize,
    pub values: HashMap<usize, Literal>,
}

// in Ctiny, a scope is equivalent to a block of a function
pub struct Scope<'a> {
    pub id: Identifier,
    variables: HashMap<Identifier, Variable>,
    pub function: &'a Function<'a>,
}

pub struct SymbolTable<'a> {
    pub scopes: HashMap<Identifier, Scope<'a>>,
}

// implementations
impl SymbolTable<'_> {
    pub fn get_scope(&self, scope_id: &Identifier) -> Option<&Scope> {
        self.scopes.get(scope_id)
    }
}

// TODO: I need the pair or span of the variable to make the error
// impl Scope<'_> {
//     pub fn get_variable(&self, var_id: &Node<'a, Identifier>) -> Result<&Variable, SemanticError> {
//         match self.variables.get(var_id) {
//             Some(var) => Ok(var),
//             None => Err(
//                 SemanticError::UndeclaredVariable(
//                     SemanticError::UndeclaredVariableError::init(
//                         var_id.get_pair().unwrap(),
//                         &format!("Undeclared variable: {}", var_id.get_name())
//                     )
//                 )
//             ),
//         }
//     }
// }

