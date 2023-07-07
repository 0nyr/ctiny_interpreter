use pest::iterators::Pairs;
use pest::error::Error;

use crate::syntax_parsing::parser::Rule;

pub mod nodes;
pub mod errors;
use errors::make_error;
use nodes::*;

const AST_ERROR_PREFIX: &str = "🔴 [AST building error]";

fn make_ast_error(pair: pest::iterators::Pair<Rule>, message: &str) -> Error<Rule> {
    make_error(pair, format!("{} {}", AST_ERROR_PREFIX, message).as_str())
}





macro_rules! unwrap_or_err_panic {
    ($func:expr) => {
        $func.unwrap_or_else(|error| { print!("{}\n", error); panic!(); })
    }
}

macro_rules! ok_build_node {
    ($pair:expr, $data:expr) => {
        Ok(
            Node {
                sp: SpanPosition {
                    start: $pair.as_span().start(),
                    end: $pair.as_span().end(),
                },
                data: $data,
            }
        )
    }
}








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

macro_rules! build_chained_operations {
    ($input_pair:expr, $rule:ident, $operator:expr) => {{
        let mut pairs = $input_pair.clone().into_inner();

        // Start by building the first operation.
        let mut operation_tree = build_expression(pairs.next().unwrap())?;

        // Then, for each remaining pair, build an operation where the left-hand
        // side is the result of the previous operations and the right-hand side
        // is the current operation.
        for pair in pairs {
            let right_operation = build_expression(pair)?;

            let common_span = SpanPosition {
                start: operation_tree.sp.start,
                end: right_operation.sp.end,
            };

            operation_tree = Node {
                sp: common_span,
                data: Expression::BinaryExpression(
                    BinaryExpression {
                        operator: $operator,
                        left: Box::new(operation_tree.data),
                        right: Box::new(right_operation.data),
                    }
                ),
            }
        }

        operation_tree
    }};
}
macro_rules! build_chained_operations_with_operators {
    ($input_pair:expr, $rule:ident, $($operator:expr),*) => {{
        let mut pairs = $input_pair.clone().into_inner();

        // Start by building the first operation.
        let mut operation_tree = build_expression(pairs.next().unwrap())?;

        // Then, for each remaining pair, build an operation where the left-hand
        // side is the result of the previous operations and the right-hand side
        // is the current operation. The operator is determined by the rule pair.
        for pair in pairs {
            // The pair should contain an operator and an expression.
            let mut inner_pairs = pair.into_inner();
            let operator = match BinaryOperator::from_rule(inner_pairs.next().unwrap())? {
                $( $operator => $operator, )*
                _ => return Err(make_ast_error(
                    pair.clone(), 
                    format!("🔴 Unexpected rule inside <{}>: {:?}", $rule, pair.clone().as_rule()).as_str()
                )),
            };
            let right_operation = build_expression(inner_pairs.next().unwrap())?;

            let common_span = SpanPosition {
                start: operation_tree.sp.start,
                end: right_operation.sp.end,
            };

            operation_tree = Node {
                sp: common_span,
                data: Expression::BinaryExpression(
                    BinaryExpression {
                        operator,
                        left: Box::new(operation_tree.data),
                        right: Box::new(right_operation.data),
                    }
                ),
            }
        }

        operation_tree
    }}
}




pub fn build_disjunction(pair: pest::iterators::Pair<Rule>) -> Result<Node<Expression>, Error<Rule>> {
    Ok(build_chained_operations!(pair, disjunction, BinaryOperator::LogicalOr))
}

pub fn build_conjunction(pair: pest::iterators::Pair<Rule>) -> Result<Node<Expression>, Error<Rule>> {
    let mut inner = pair.clone().into_inner().next().unwrap();
            
    let mut conjunctions = Vec::new();
    while inner.as_rule() == Rule::conjunction {
        conjunctions.push(build_expression(inner.clone())?);
        inner = match inner.into_inner().next() {
            Some(pair) => pair,
            None => break,
        };
    }
    if conjunctions.len() == 1 {
        return Ok(conjunctions.pop().unwrap());
    } else {
        let mut conjunction_tree = {
            let right_conjunction = conjunctions.pop().unwrap();
            let left_conjunction = conjunctions.pop().unwrap();

            // reconstruct the common span
            let common_span = SpanPosition {
                start: left_conjunction.sp.start,
                end: right_conjunction.sp.end,
            };

            Node {
                sp: common_span,
                data: Expression::BinaryExpression(
                    BinaryExpression {
                        operator: BinaryOperator::LogicalOr,
                        left: Box::new(left_conjunction.data),
                        right: Box::new(right_conjunction.data),
                    }
                ),
            }
        };
        while conjunctions.len() > 0 {
            let right_conjunction = conjunction_tree;
            let left_conjunction = conjunctions.pop().unwrap();

            // reconstruct the common span
            let common_span = SpanPosition {
                start: left_conjunction.sp.start,
                end: right_conjunction.sp.end,
            };
            
            conjunction_tree = Node {
                sp: common_span,
                data: Expression::BinaryExpression(
                    BinaryExpression {
                        operator: BinaryOperator::LogicalOr,
                        left: Box::new(left_conjunction.data),
                        right: Box::new(right_conjunction.data),
                    }
                ),
            }
        }
        return Ok(conjunction_tree);
    }
}

pub fn build_literal(pair: pest::iterators::Pair<Rule>) -> Result<Node<Expression>, Error<Rule>> {
    let literal = pair.clone().into_inner().next().unwrap();
    let res = match literal.as_rule() {
        Rule::boolean => Expression::Literal(Literal::Bool(literal.as_str().parse().unwrap())),
        Rule::float => Expression::Literal(Literal::Float(literal.as_str().parse().unwrap())),
        Rule::char => Expression::Literal(Literal::Char(literal.as_str().chars().next().unwrap())),
        Rule::integer => Expression::Literal(Literal::Int(literal.as_str().parse().unwrap())),
        _ => {
            let message = format!("🔴 Unexpected rule in <literal> match tree: {:?}", literal.as_rule());
            return Err(make_ast_error(pair, &message))
        },
    };
    ok_build_node!(pair, res)
}

pub fn build_type_specifier(pair: pest::iterators::Pair<Rule>) -> Result<Node<TypeSpecifier>, Error<Rule>> {
    let res = match pair.as_str() {
        "bool" => TypeSpecifier::Bool,
        "float" => TypeSpecifier::Float,
        "char" => TypeSpecifier::Char,
        "int" => TypeSpecifier::Int,
        _ => {
            let message = format!("🔴 Unexpected <type_specifier>: {}", pair.clone().as_str());
            return Err(make_ast_error(pair, &message))
        },
    };
    ok_build_node!(pair, res)
}


pub fn build_expression(pair: pest::iterators::Pair<Rule>) -> Result<Node<Expression>, Error<Rule>> {
    let rule = pair.as_rule();
    match rule {
        Rule::expression => build_expression(pair.into_inner().next().unwrap()),
        Rule::disjunction => Ok(build_chained_operations!(pair, disjunction, BinaryOperator::LogicalOr)),
        Rule::conjunction => Ok(build_chained_operations!(pair, conjunction, BinaryOperator::LogicalAnd)),
        Rule::equality => Ok(build_chained_operations_with_operators!(pair, rule, BinaryOperator::Equal, BinaryOperator::NotEqual)),
        Rule::parenthesized_expression => build_expression(pair.into_inner().next().unwrap()),
        Rule::literal => build_literal(pair),
        Rule::function_call => {
            let mut inner = pair.clone().into_inner();
            let identifier = inner.next().unwrap().as_str().to_string();
            let arguments = inner.next().unwrap().into_inner()
                .map(|pair| unwrap_or_err_panic!(build_expression(pair)).data)
                .collect::<Vec<_>>();
            ok_build_node!(pair, Expression::FunctionCall(
                FunctionCall {
                    name: Identifier { name: identifier },
                    arguments: arguments,
                }
            ))
        },
        Rule::type_cast => {
            let mut inner = pair.clone().into_inner();
            let type_specifier = unwrap_or_err_panic!(build_type_specifier(inner.next().unwrap())).data;
            let expression = unwrap_or_err_panic!(build_expression(inner.next().unwrap())).data;
            ok_build_node!(pair, Expression::TypeCast(
                TypeCast {
                    type_specifier,
                    expression: Box::new(expression),
                }
            ))
        },
        Rule::get_value => {
            let mut inner = pair.clone().into_inner();
            let identifier = inner.next().unwrap().as_str().to_string();
            let index = match inner.next() {
                Some(pair) => Some(Box::new(unwrap_or_err_panic!(build_expression(pair)).data)),
                None => None,
            };
            ok_build_node!(pair, Expression::GetValue(
                GetValue {
                    identifier: Identifier { name: identifier },
                    index,
                }
            ))
        },
        // TODO: add all underlying expression rules...
        _ => {
            let message = format!("🔴 Unexpected rule in <expression> match tree: {:?}", rule);
            return Err(make_ast_error(pair, &message))
        },
    }
}