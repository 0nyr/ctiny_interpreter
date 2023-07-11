use pest::error::Error;

use crate::syntax_parsing::parser::Rule;

use super::nodes::*;
use super::errors::make_ast_error;

// exported macros are available in the crate root (global scope)
use crate::unwrap_or_err_panic;
use crate::ok_build_node;

macro_rules! build_chained_operations {
    ($input_pair:expr, $( $operator:expr ),*) => {{
        let mut pairs = $input_pair.clone().into_inner();

        // Start by building the first operation. (leftmost)
        let mut left_operation = build_expression(pairs.next().unwrap())?;

        // Then, for each remaining pair, build an operation where the left-hand
        // side is the result of the previous operations and the right-hand side
        // is the current operation.
        // we are in the "(some_operator ~ relation)*" part
        // the current pair alternates between an operator and an expression
        while let (Some(pair_op), Some(pair_expr)) = (pairs.next(), pairs.next()) {
            // first pair must be an operator
            let operator_type = {
                let operator_type = BinaryOperator::from_str(pair_op.clone().as_str());
                match operator_type {
                    Some(real_operator_type) => {                    
                        match real_operator_type.as_str() {
                            $( x if x == $operator.as_str() => (real_operator_type), )* // do nothing
                            _ => return Err(make_ast_error(
                                $input_pair.clone(), 
                                format!(
                                    "🟣 matched operator {:?} not in the list of potential operators for rule {:?}", 
                                    real_operator_type, pair_op.clone().as_rule()).as_str()
                            )),
                        }
                    },
                    None => {
                        let vec_of_op_str = vec![$( $operator.as_str() ),*];
                        let str_of_possible_operators = format!("{:?}", vec_of_op_str);
                        return Err(make_ast_error(
                            $input_pair.clone(), 
                            format!("🟣 couldn't match pair_op with list of potential operators ({}) for rule {:?}", 
                                str_of_possible_operators, pair_op.clone().as_rule()).as_str()
                        ))
                    }
                }
            };

            // second pair must be an expression
            let right_operation = build_expression(pair_expr)?;

            let common_span = SpanPosition {
                start: left_operation.sp.start,
                end: right_operation.sp.end,
            };

            left_operation = Node {
                sp: common_span,
                data: Expression::BinaryExpression(
                    BinaryExpression {
                        operator: operator_type,
                        left: Box::new(left_operation.data),
                        right: Box::new(right_operation.data),
                    }
                ),
            }
        }
        
        // all pairs must be consumed by now
        if let Some(last_element) = pairs.next() {
            return Err(make_ast_error(
                $input_pair.clone(), 
                format!("🟣 The list of pairs must be even. The last element is: {:?}", 
                last_element.clone().as_rule()).as_str()
            ))
        }

        left_operation
    }};
}



fn build_conjunction(pair: pest::iterators::Pair<Rule>) -> Result<Node<Expression>, Error<Rule>> {
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

fn build_literal(pair: pest::iterators::Pair<Rule>) -> Result<Node<Expression>, Error<Rule>> {
    let literal = pair.clone().into_inner().next().unwrap();
    let res = match literal.as_rule() {
        Rule::boolean => Expression::Literal(Literal::Bool(literal.as_str().parse().unwrap())),
        Rule::float => Expression::Literal(Literal::Float(literal.as_str().parse().unwrap())),
        Rule::char => {
            // convert &str containing single quotes to char
            let char_literal = literal.as_str();
            let char_trimmed = char_literal.trim_start_matches('\'').trim_end_matches('\'');
            let real_char = char_trimmed.chars().next().unwrap();
            Expression::Literal(Literal::Char(real_char))},
        Rule::integer => Expression::Literal(Literal::Int(literal.as_str().parse().unwrap())),
        _ => {
            let message = format!("🔴 Unexpected rule in <literal> match tree: {:?}", literal.as_rule());
            return Err(make_ast_error(pair, &message))
        },
    };
    ok_build_node!(pair, res)
}

fn build_type_specifier(pair: pest::iterators::Pair<Rule>) -> Result<Node<TypeSpecifier>, Error<Rule>> {
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

// factor = { unary_operator? ~ primary }
fn build_factor(pair: pest::iterators::Pair<Rule>) -> Result<Node<Expression>, Error<Rule>> {
    let mut inner = pair.clone().into_inner();

    // we get the first pair, which is either an operator or a primary
    let first_pair = inner.next().unwrap();
    let first_pair_rule = first_pair.clone().as_rule();

    match first_pair_rule {
        Rule::unary_operator => {
            let unary_operator = UnaryOperator::from_str(first_pair.clone().as_str()).unwrap();
            let primary = inner.next().unwrap();
            let primary = build_expression(primary)?;
            let res = Expression::UnaryExpression(
                UnaryExpression {
                    operator: unary_operator,
                    expression: Box::new(primary.data),
                }
            );
            return ok_build_node!(pair, res);
        },
        Rule::primary => {
            let primary = build_expression(first_pair)?;
            return ok_build_node!(pair, primary.data);
        },
        _ => {
            let message = format!("🔴 Unexpected rule in <factor> match tree: {:?}", first_pair_rule);
            return Err(make_ast_error(pair, &message))
        },
    }
}

pub fn get_or_set_value_from_pair(pair: pest::iterators::Pair<Rule>) -> Result<GetOrSetValue, Error<Rule>> {
    let mut inner = pair.clone().into_inner();
    let identifier = inner.next().unwrap().as_str().to_string();
    let index = match inner.next() {
        Some(pair) => Some(Box::new(unwrap_or_err_panic!(build_expression(pair)).data)),
        None => None,
    };
    Ok(GetOrSetValue {
        identifier: Identifier { name: identifier },
        index,
    })
}

pub fn identifier_from_pair(pair: pest::iterators::Pair<Rule>) -> Result<Identifier, Error<Rule>> {
    let identifier = pair.clone().as_str().to_string();
    Ok(Identifier { name: identifier })
}

pub fn build_expression(pair: pest::iterators::Pair<Rule>) -> Result<Node<Expression>, Error<Rule>> {
    let rule = pair.as_rule();
    match rule {
        Rule::expression => build_expression(pair.into_inner().next().unwrap()),
        Rule::disjunction => Ok(build_chained_operations!(pair, BinaryOperator::LogicalOr)),
        Rule::conjunction => Ok(build_chained_operations!(pair, BinaryOperator::LogicalAnd)),
        Rule::equality => Ok(build_chained_operations!(pair, BinaryOperator::Equal, BinaryOperator::NotEqual)),
        Rule::relation => Ok(build_chained_operations!(pair, BinaryOperator::Less, BinaryOperator::LessOrEqual, BinaryOperator::Greater, BinaryOperator::GreaterOrEqual)),
        Rule::addition => Ok(build_chained_operations!(pair, BinaryOperator::Plus, BinaryOperator::Minus)),
        Rule::term => Ok(build_chained_operations!(pair, BinaryOperator::Multiply, BinaryOperator::Divide, BinaryOperator::Modulo)),
        Rule::factor => build_factor(pair),
        Rule::primary => build_expression(pair.into_inner().next().unwrap()),
        Rule::parenthesized_expression => build_expression(pair.into_inner().next().unwrap()),
        Rule::literal => build_literal(pair),
        Rule::function_call => {
            let mut inner = pair.clone().into_inner();
            let identifier = unwrap_or_err_panic!(identifier_from_pair(inner.next().unwrap()));

            // function call may have 0 or more arguments
            let arguments = {
                let mut arguments = Vec::new();
                for argument in inner {
                    arguments.push(unwrap_or_err_panic!(build_expression(argument)).data);
                }
                arguments
            };
            ok_build_node!(pair, Expression::FunctionCall(
                FunctionCall {
                    name: identifier,
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
        Rule::get_or_set_value => {
            let get_or_set_value = get_or_set_value_from_pair(pair.clone())?;
            ok_build_node!(pair, Expression::GetOrSetValue(get_or_set_value))
        },
        _ => {
            let message = format!("🔴 Unexpected rule in <expression> match tree: {:?}", rule);
            return Err(make_ast_error(pair, &message))
        },
    }
}