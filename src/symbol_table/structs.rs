use std::collections::HashMap;

use crate::{abstract_syntax_tree::nodes::{Identifier, TypeSpecifier, Value, Node}, semantic::{errors::{SemanticError, UndeclaredVariableError, SemanticErrorTrait, RedeclarationError}, type_casts::{get_index_value_from_value_node, cast_to_type}}};

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
    value: Option<Value>,
}

impl NormalVarData {
    pub fn new(id: Identifier, type_specifier: TypeSpecifier) -> Self {
        Self {
            id,
            type_specifier,
            value: None,
        }
    }

    pub fn set_value<'a>(&mut self, value_node: Node<'a, Value>) -> Result<(), SemanticError> {
        // check that the type of the value is the same as the type of the variable
        let value_type = value_node.data.as_type_specifier();
        if self.type_specifier != value_type {
            let casted_value_node = cast_to_type(
                value_node, self.type_specifier
            )?;
            self.value = Some(casted_value_node.data);
            Ok(())
        } else {
            self.value = Some(value_node.data);
            Ok(())
        }
    }

    pub fn get_value(&self) -> Option<&Value> {
        self.value.as_ref()
    }
}

#[derive(Debug, PartialEq)]
pub struct ArrayVarData {
    pub id: Identifier,
    pub type_specifier: TypeSpecifier,
    pub size: usize,
    values: HashMap<usize, Value>,
}

impl ArrayVarData {
    pub fn new(id: Identifier, type_specifier: TypeSpecifier, size: usize) -> Self {
        Self {
            id,
            type_specifier,
            size,
            values: HashMap::new(),
        }
    }

    pub fn set_value<'a>(
        &mut self, 
        index_node: Node<'a, Value>,
        value_node: Node<'a, Value>, 
    ) -> Result<(), SemanticError> {
        let usable_index = get_index_value_from_value_node(index_node)?;
        // cast the value to the type of the array
        let casted_value_node = cast_to_type(
            value_node, self.type_specifier
        )?;
        self.values.insert(usable_index, casted_value_node.data);
        Ok(())
    }

    pub fn get_value(&self, index: usize) -> Option<&Value> {
        self.values.get(&index)
    }
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

    pub fn get_mut_scope(&mut self, scope_id: &Node<Identifier>) -> Option<&mut Scope> {
        self.scopes.get_mut(&scope_id.data)
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

    pub fn get_variable(&self, var_id_node: &Node<Identifier>) -> Result<&Variable, SemanticError> {
        let potential_var = self.variables.get(&var_id_node.data);
        match potential_var {
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

    pub fn get_mut_variable<'a>(&mut self, var_id_node: &Node<'a, Identifier>) -> Result<&mut Variable, SemanticError> {
        let potential_mut_var = self.variables.get_mut(&var_id_node.data);
        match potential_mut_var {
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

    /// Get a variable value.
    /// If the variable is an array, get the value at the given index.
    /// 
    /// This function make use get_normal_variable and get_array_variable.
    /// 
    /// The function check that the variable is an array if the index is given.
    /// The function check that the index is a positive integer.
    pub fn get_variable_value<'a>(
        &self, 
        var_id_node: &Node<'a, Identifier>, 
        potential_index: Option<Node<'a, Value>>
    ) -> Result<Node<'a, Value>, SemanticError> {
        // check if the variable is an array or a normal variable
        // if the index is given, make sure its value is a positive integer
        match potential_index {
            Some(index) => {
                // case: array
                let index_span = index.sp;
                let index_value = get_index_value_from_value_node(index)?;
                let array_var_data = self.get_array_variable(var_id_node)?;
                match array_var_data.get_value(index_value) {
                    Some(value) => Ok(
                        Node {
                            sp: index_span,
                            data: value.clone(),
                        }
                    ),
                    None => Err(
                        SemanticError::UndeclaredVariable(
                            UndeclaredVariableError::init(
                                index_span,
                                &format!("Array <{}> does not have a value at index {}", var_id_node.data.name, index_value)
                            )
                        )
                    ),
                }
            },
            None => {
                // case: normal variable
                let normal_var_data = self.get_normal_variable(var_id_node)?;
                match normal_var_data.get_value() {
                    Some(value) => Ok(
                        Node {
                            sp: var_id_node.sp,
                            data: value.clone(),
                        }
                    ),
                    None => Err(
                        SemanticError::UndeclaredVariable(
                            UndeclaredVariableError::init(
                                var_id_node.sp,
                                &format!("Variable <{}> does not have a value", var_id_node.data.name)
                            )
                        )
                    ),
                }
            },
        }
    }

    pub fn set_normal_variable_value<'a>(
        &mut self, 
        var_id_node: &Node<'a, Identifier>, 
        value_node: Node<'a, Value>,
    ) -> Result<(), SemanticError> {
        match self.get_mut_variable(var_id_node)? {
            Variable::NormalVar(normal_var_data) => {
                // set the value
                normal_var_data.set_value(value_node)?;
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

    pub fn set_array_variable_value<'a>(
        &mut self, 
        var_id_node: &Node<'a, Identifier>,
        index_node: Node<'a, Value>,
        value_node: Node<'a, Value>,
    ) -> Result<(), SemanticError> {
        match self.get_mut_variable(var_id_node)? {
            Variable::NormalVar(_) => Err(
                SemanticError::UndeclaredVariable(
                    UndeclaredVariableError::init(
                        var_id_node.sp,
                        &format!("Variable {} is a normal variable, not an array", var_id_node.data.name)
                    )
                )
            ),
            Variable::ArrayVar(array_var_data) => {
                // set the value
                array_var_data.set_value(index_node, value_node)?;
                Ok(())
            },
        }
    }

    // this function adds a new variable to the scope
    // It checks that the variable is not already declared in the scope
    pub fn add_variable<'a>(&mut self, variable_node: Node<'a, Variable>) -> Result<(), SemanticError> {
        let var_id = match &variable_node.data {
            Variable::NormalVar(normal_var_data) => normal_var_data.id.clone(),
            Variable::ArrayVar(array_var_data) => array_var_data.id.clone(),
        };
        match self.variables.get(&var_id) {
            Some(_) => Err(
                SemanticError::Redeclaration(
                    RedeclarationError::init(
                        variable_node.sp,
                        &format!("Variable {} is already declared in this scope", var_id.name)
                    )
                )
            ),
            None => {
                // add the variable to the scope
                self.variables.insert(var_id, variable_node.data);
                Ok(())
            },
        }
    }
}

