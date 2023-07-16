use std::collections::HashMap;

use crate::{abstract_syntax_tree::nodes::{Identifier, TypeSpecifier, Literal, Node}, semantic_analysis::errors::{SemanticError, UndeclaredVariableError, SemanticErrorTrait}};

#[derive(Debug)]
pub enum Variable {
    NormalVar(NormalVarData),
    ArrayVar(ArrayVarData),
}

// I want to be able to say if two variables are equal.
// this happens when the two variables are the same enum, with same data
impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Variable::NormalVar(self_data), Variable::NormalVar(other_data)) => {
                self_data == other_data
            },
            (Variable::ArrayVar(self_data), Variable::ArrayVar(other_data)) => {
                self_data == other_data
            },
            _ => false,
        }
    }
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
pub struct Scope {
    pub id: Identifier,
    variables: HashMap<Identifier, Variable>,
}

pub struct SymbolTable {
    scopes: HashMap<Identifier, Scope>,
}

// implementations
impl SymbolTable {
    pub fn new() -> Self {
        Self {
            scopes: HashMap::new()
        }
    }

    pub fn get_scope(&self, scope_id: &Node<Identifier>) -> Option<&Scope> {
        self.scopes.get(&scope_id.data)
    }

    pub fn add_scope(&mut self, scope: Scope) {
        self.scopes.insert(scope.id.clone(), scope);
    }
}

impl Scope {
    pub fn new(id: Identifier, variables: HashMap<Identifier, Variable>) -> Self {
        Self {
            id,
            variables,
        }
    }

    pub fn get_variable<'a>(&self, var_id_node: &Node<'a, Identifier>) -> Result<&Variable, SemanticError> {
        match self.variables.get(&var_id_node.data) {
            Some(var) => Ok(var),
            None => Err(
                SemanticError::UndeclaredVariable(
                    UndeclaredVariableError::init(
                        var_id_node.sp,
                        &format!("Undeclared variable: {}", var_id_node.data.name)
                    )
                )
            ),
        }
    }

    // The following function is used for testing purposes
    #[cfg(test)]
    pub fn get_variable_from_id(&self, var_id: &Identifier) -> Result<&Variable, String> {
        match self.variables.get(var_id) {
            Some(var) => Ok(var),
            None => Err(
                format!("Undeclared variable: {}", var_id.name)
            ),
        }
    }

    pub fn get_normal_variable<'a>(&self, var_id_node: &Node<'a, Identifier>) -> Result<&NormalVarData, SemanticError> {
        match self.get_variable(var_id_node)? {
            Variable::NormalVar(normal_var_data) => Ok(normal_var_data),
            Variable::ArrayVar(_) => Err(
                SemanticError::UndeclaredVariable(
                    UndeclaredVariableError::init(
                        var_id_node.sp,
                        &format!("Variable {} is an array, not a normal variable", var_id_node.data.name)
                    )
                )
            ),
        }
    }

    pub fn get_array_variable<'a>(&self, var_id_node: &Node<'a, Identifier>) -> Result<&ArrayVarData, SemanticError> {
        match self.get_variable(var_id_node)? {
            Variable::NormalVar(_) => Err(
                SemanticError::UndeclaredVariable(
                    UndeclaredVariableError::init(
                        var_id_node.sp,
                        &format!("Variable {} is a normal variable, not an array", var_id_node.data.name)
                    )
                )
            ),
            Variable::ArrayVar(array_var_data) => Ok(array_var_data),
        }
    }

    pub fn set_normal_variable<'a>(
        &self, 
        var_id_node: &Node<'a, Identifier>, 
        value: Literal
    ) -> Result<(), SemanticError> {
        match self.get_variable(var_id_node)? {
            Variable::NormalVar(normal_var_data) => {
                let value_type = value.as_type_specifier();
                if normal_var_data.type_specifier != value_type {
                    // TODO: for now we return an error, but in the future we should do automatic type conversion)
                    return Err(
                        SemanticError::UndeclaredVariable(
                            UndeclaredVariableError::init(
                                var_id_node.sp,
                                &format!("Variable {} is of type {:?}, but the value is of type {:?}", var_id_node.data.name, normal_var_data.type_specifier, value_type)
                            )
                        )
                    );
                } else {
                    // set the value

                }
                Ok(())
            },
            Variable::ArrayVar(_) => Err(
                SemanticError::UndeclaredVariable(
                    UndeclaredVariableError::init(
                        var_id_node.sp,
                        &format!("Variable {} is an array, not a normal variable", var_id_node.data.name)
                    )
                )
            ),
        }
    }
}

