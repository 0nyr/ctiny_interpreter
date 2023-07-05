use pest::iterators::Pairs;
use pest::error::Error;

use crate::syntax_parsing::parser::Rule;

pub mod nodes;
pub mod errors;
use errors::make_error;
use nodes::*;


pub fn convert_to_ast(pairs: Pairs<Rule>) -> Result<Node<Program>, Error<Rule>> {
    let mut functions = vec![];

    // keep a copy of the translation unit pair
    let translation_unit_pair = pairs.clone().next().unwrap();
    assert!(translation_unit_pair.as_rule() == Rule::translation_unit);

    for pair in translation_unit_pair.clone().into_inner() {
        match pair.as_rule() {
            Rule::function_definition => {
                let mut inner = pair.clone().into_inner();
                let return_type = convert_type_specifier(inner.next().unwrap())?;
                let identifier = inner.next().unwrap().as_str().to_string();
                let function_parameters = inner.next().unwrap().into_inner()
                    .map(|pair| convert_declaration(pair))
                    .collect::<Result<Vec<_>, _>>()?;
                let body = convert_block_statement(inner.next().unwrap())?;
                functions.push(make_node(pair, FunctionDefinition::Function {
                    name: identifier,
                    return_type: return_type.inner,
                    params: function_parameters,
                    body: body.inner,
                }));
            },
            Rule::entry_point_function_definition => {
                let body = convert_block_statement(pair.clone().into_inner().next().unwrap())?;
                functions.push(make_node(pair, FunctionDefinition::EntryPoint(body.inner)));
            },
            _ => return Err(make_error(
                pair.clone(), 
                format!("🔴 Unexpected rule inside <translation_unit>: {:?}", pair.clone().as_rule()).as_str()
            )),
        }
    }

    Ok(make_node(translation_unit_pair, Program::TranslationUnit(functions)))
}

fn convert_type_specifier(pair: pest::iterators::Pair<Rule>) -> Result<Node<TypeSpecifier>, Error<Rule>> {
    let inner = match pair.as_str() {
        "bool" => TypeSpecifier::Bool,
        "float" => TypeSpecifier::Float,
        "char" => TypeSpecifier::Char,
        "int" => TypeSpecifier::Int,
        _ => return Err(make_error(
            pair.clone(), 
            format!("🔴 Unexpected <type_specifier>: {}", pair.clone().as_str()).as_str()
        )),
    };
    Ok(make_node(pair, inner))
}

fn convert_declaration(pair: pest::iterators::Pair<Rule>) -> Result<Node<Declaration>, Error<Rule>> {
    let mut inner = pair.clone().into_inner();
    let type_specifier = convert_type_specifier(inner.next().unwrap())?;
    let identifier = inner.next().unwrap().as_str().to_string();
    Ok(make_node(pair, Declaration {
        type_specifier: type_specifier.inner,
        identifier: identifier,
    }))
}

fn convert_block_statement(pair: pest::iterators::Pair<Rule>) -> Result<Node<StatementBlock>, Error<Rule>> {
    let statements = pair.clone().into_inner()
        .map(|pair| convert_statement(pair))
        .collect::<Result<Vec<_>, _>>()?;
    let inner_statements = statements.into_iter().map(|node| node.inner).collect();
    Ok(make_node(pair, StatementBlock::Statements(inner_statements)))
}

fn convert_statement(pair: pest::iterators::Pair<Rule>) -> Result<Node<Statement>, Error<Rule>> {
    let inner = match pair.as_rule() {
        Rule::assignment_statement => {
            let mut inner = pair.clone().into_inner();
            let identifier = inner.next().unwrap().as_str().to_string();
            let expression = convert_expression(inner.next().unwrap())?;
            Statement::Assignment(AssignmentStatement {
                identifier: identifier,
                expression: expression.inner,
            })
        },
        _ => return Err(make_error(
            pair.clone(), 
            format!("🔴 Unexpected <statement>: {:?}", pair.clone().as_rule()).as_str()
        )),
    };
    Ok(make_node(pair, inner))
}

fn convert_expression(pair: pest::iterators::Pair<Rule>) -> Result<Node<Expression>, Error<Rule>> {
    let inner = match pair.as_rule() {
        Rule::identifier => Expression::Identifier(pair.as_str().to_string()),
        Rule::literal => {
            let value = pair.as_str().to_string();
            if value == "true" || value == "false" {
                Expression::Literal(Literal::Boolean(value == "true"))
            } else if value.contains('.') {
                Expression::Literal(Literal::Float(value.parse().unwrap()))
            } else if value.starts_with("'") {
                Expression::Literal(Literal::Char(value.chars().nth(1).unwrap()))
            } else {
                Expression::Literal(Literal::Integer(value.parse().unwrap()))
            }
        },
        _ => {
            let message = format!("🔴 Unexpected <expression>: {:?}", pair.as_rule());
            return Err(make_error(pair, &message))
        },
    };
    Ok(make_node(pair, inner))
}
