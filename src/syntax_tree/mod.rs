use std::fmt::Binary;

use pest::iterators::Pairs;
use pest::error::Error;

use crate::syntax_parsing::parser::Rule;

pub mod nodes;
pub mod errors;
pub mod expressions;
pub mod statements;
use errors::make_ast_error;
use nodes::*;


















// pub fn convert_to_ast(pairs: Pairs<Rule>) -> Result<ProgramAST, Error<Rule>> {
//     let mut functions = vec![];

//     // keep a copy of the translation unit pair
//     let translation_unit_pair = pairs.clone().next().unwrap();
//     assert!(translation_unit_pair.as_rule() == Rule::translation_unit);

//     for pair in translation_unit_pair.clone().into_inner() {
//         match pair.as_rule() {
//             Rule::function_definition => {
//                 functions.push(convert_function(pair)?);
//             },
//             Rule::entry_point_function_definition => {
//                 let body = convert_block(pair.clone().into_inner().next().unwrap())?;
//                 functions.push(make_node(pair, FunctionDefinition::EntryPoint(body.inner)));
//             },
//             _ => return Err(make_ast_error(
//                 pair.clone(), 
//                 format!("🔴 Unexpected rule inside <translation_unit>: {:?}", pair.clone().as_rule()).as_str()
//             )),
//         }
//     }

//     let translation_unit_node = make_node(
//         translation_unit_pair.clone(), 
//         TranslationUnit { functions }
//     );
//     Ok(ProgramAST { translation_unit: translation_unit_node })
// }

// fn convert_function(pair: pest::iterators::Pair<Rule>) -> Result<Node<FunctionDefinition>, Error<Rule>> {
//     let mut inner = pair.clone().into_inner();
//     let return_type = convert_type_specifier(inner.next().unwrap())?;

//     let identifier = inner.next().unwrap().as_str().to_string();
//     let identifier_node = make_node(
//         pair.clone(), Identifier { name: identifier }
//     );

//     let function_parameters = inner.next().unwrap().into_inner()
//         .map(|pair| convert_declaration(pair))
//         .collect::<Result<Vec<_>, _>>()?;
//     let body = convert_block(inner.next().unwrap())?;

//     Ok(make_node(pair, FunctionDefinition::Function {
//         name: identifier_node,
//         return_type: return_type.inner,
//         params: function_parameters,
//         body: body.inner,
//     }))
// }

// fn convert_type_specifier(pair: pest::iterators::Pair<Rule>) -> Result<Node<TypeSpecifier>, Error<Rule>> {
//     let inner = match pair.as_str() {
//         "bool" => TypeSpecifier::Bool,
//         "float" => TypeSpecifier::Float,
//         "char" => TypeSpecifier::Char,
//         "int" => TypeSpecifier::Int,
//         _ => return Err(make_ast_error(
//             pair.clone(), 
//             format!("🔴 Unexpected <type_specifier>: {}", pair.clone().as_str()).as_str()
//         )),
//     };
//     Ok(make_node(pair, inner))
// }

// fn convert_block(pair: pest::iterators::Pair<Rule>) -> Result<Node<Block>, Error<Rule>> {
//     let mut declarations = Vec::new();
//     let mut statements = Vec::new();

//     for inner_pair in pair.clone().into_inner() {
//         match inner_pair.as_rule() {
//             Rule::multi_declaration => {
//                 declarations.extend(convert_multi_declaration(inner_pair)?.inner.declarations);
//             }
//             Rule::statement => {
//                 statements.push(convert_statement(inner_pair)?);
//             }
//             _ => unreachable!(),
//         }
//     }

//     let block = Block {
//         declarations,
//         statements,
//     };
//     Ok(make_node(pair, block))
// }

// fn convert_multi_declaration(pair: pest::iterators::Pair<Rule>) -> Result<Node<MultiDeclaration>, Error<Rule>> {
//     let first_declaration = convert_declaration(pair.clone().into_inner().next().unwrap())?;
    
//     let followup_declarations = pair.clone().into_inner()
//         .map(|pair| {
//             convert_followup_declaration(
//                 pair, 
//                 first_declaration.inner.type_specifier
//             )
//         })
//         .collect::<Result<Vec<_>, _>>()?;

//     // return all declarations
//     let mut declarations = vec![first_declaration];
//     declarations.extend(followup_declarations);
//     Ok(make_node(pair, MultiDeclaration { declarations }))
// }

// fn convert_declaration(pair: pest::iterators::Pair<Rule>) -> Result<Node<Declaration>, Error<Rule>> {    
//     let mut inner = pair.clone().into_inner();
//     let type_specifier = convert_type_specifier(inner.next().unwrap())?;
//     let identifier = inner.next().unwrap().as_str().to_string();
    
//     // get array size if it exists
//     let array_size = match inner.next() {
//         Some(pair) => pair.as_str().parse().ok(),
//         None => None,
//     };

//     Ok(make_node(pair.clone(), Declaration {
//         type_specifier: type_specifier.inner,
//         identifier: make_node(pair.clone(), Identifier { name: identifier }),
//         array_size,
//     }))
// }

// fn convert_followup_declaration(
//     pair: pest::iterators::Pair<Rule>,
//     inner_type_specifier: TypeSpecifier,
// ) -> Result<Node<Declaration>, Error<Rule>> {
//     let mut inner = pair.clone().into_inner();
//     let identifier = inner.next().unwrap().as_str().to_string();
    
//     // get array size if it exists
//     let array_size = match inner.next() {
//         Some(pair) => pair.as_str().parse().ok(),
//         None => None,
//     };

//     Ok(make_node(pair.clone(), Declaration {
//         type_specifier: inner_type_specifier,
//         identifier: make_node(pair.clone(), Identifier { name: identifier }),
//         array_size,
//     }))
// }

// pub fn convert_statement(pair: pest::iterators::Pair<Rule>) -> Result<Node<Statement>, Error<Rule>> {
//     let inner = match pair.as_rule() {
//         Rule::assignment_statement => convert_assignment_statement(pair),
//         Rule::if_statement => convert_if_statement(pair),
//         Rule::while_statement => convert_while_statement(pair),
//         Rule::jump_statement => convert_jump_statement(pair),
//         _ => return Err(make_ast_error(
//             pair.clone(), 
//             format!("🔴 Unexpected <statement>: {:?}", pair.clone().as_rule()).as_str()
//         )),
//     };
//     inner
// }

// fn convert_assignment_statement(pair: pest::iterators::Pair<Rule>) -> Result<Node<Statement>, Error<Rule>> {
//     let mut inner = pair.clone().into_inner();
//     let identifier = inner.next().unwrap().as_str().to_string();
//     let expression = convert_expression(inner.next().unwrap())?;

//     Ok(make_node(pair.clone(), Statement::Assignment(
//         make_node(pair.clone(), AssignmentStatement {
//             identifier: make_node(pair.clone(), Identifier { name: identifier }),
//             expression: expression,
//         })
//     )))
// }

// fn convert_if_statement(pair: pest::iterators::Pair<Rule>) -> Result<Node<Statement>, Error<Rule>> {
//     let mut inner = pair.clone().into_inner();
//     let condition = convert_expression(inner.next().unwrap())?;
//     let if_body = convert_statement(inner.next().unwrap())?;
//     let else_body = inner.next().map(convert_statement).transpose()?;

//     Ok(make_node(pair.clone(), Statement::If(
//         make_node(pair.clone(), IfStatement {
//             condition: condition,
//             if_body: Box::new(if_body),
//             else_body: else_body.map(Box::new),
//         })
//     )))
// }

// fn convert_while_statement(pair: pest::iterators::Pair<Rule>) -> Result<Node<Statement>, Error<Rule>> {
//     let mut inner = pair.clone().into_inner();
//     let condition = convert_expression(inner.next().unwrap())?;
//     let body = convert_statement(inner.next().unwrap())?;

//     Ok(make_node(pair.clone(), Statement::While(
//         make_node(pair.clone(), WhileStatement {
//             condition: condition,
//             body: Box::new(body),
//         })
//     )))
// }

// fn convert_jump_statement(pair: pest::iterators::Pair<Rule>) -> Result<Node<Statement>, Error<Rule>> {
//     let jump = pair.clone().into_inner().next().unwrap();
//     let statement = match jump.as_rule() {
//         Rule::continue_statement => Statement::Jump(make_node(jump, JumpStatement::Continue)),
//         Rule::break_statement => Statement::Jump(make_node(jump, JumpStatement::Break)),
//         Rule::return_statement => {
//             let expression = convert_expression(jump.clone().into_inner().next().unwrap())?;
//             Statement::Jump(make_node(jump.clone(), JumpStatement::Return(expression)))
//         },
//         _ => unreachable!(),
//     };
//     Ok(make_node(pair, statement))
// }

