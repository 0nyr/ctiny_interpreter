use pest::error::Error;

use crate::syntax_parsing::Rule;

use super::nodes::*;
use crate::errors::make_ast_error_from_pair;

// exported macros are available in the crate root (global scope)
use crate::unwrap_or_err_panic;
use crate::ok_build_node;

// in this situation, pest removes whitespace and everything in between the pairs.
// So I just need to combine the spans of the two pairs, without checking if they are
// consecutive or not.
macro_rules! merge_spans_no_check {
    ($span1:expr, $span2:expr) => {{
        let input = $span1.get_input();
        let start = $span1.start();
        let end = $span2.end();
        pest::Span::new(input, start, end)
    }};
}

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
                            _ => return Err(make_ast_error_from_pair(
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
                        return Err(make_ast_error_from_pair(
                            $input_pair.clone(), 
                            format!("🟣 couldn't match pair_op with list of potential operators ({}) for rule {:?}", 
                                str_of_possible_operators, pair_op.clone().as_rule()).as_str()
                        ))
                    }
                }
            };

            // second pair must be an expression
            let right_operation = build_expression(pair_expr)?;

            // let common_span = Span::new(
            //     left_operation.sp.start(),
            //     right_operation.sp.end(),
            // ).expect(format!(
            //     "🔴 Couldn't build a span from the left and right operations. str: {}, start: {}, end: {}",
            //     $input_pair.as_str(),
            //     left_operation.sp.start(),
            //     right_operation.sp.end(),
            // ).as_str());

            let common_span = merge_spans_no_check!(
                &left_operation.sp,
                &right_operation.sp
            ).ok_or(make_ast_error_from_pair(
                $input_pair.clone(), 
                format!("🔴 Couldn't build a span from the left and right operations. 
                    str_len: {}, left_start: {}, left_end: {}, right_start: {}, right_end: {}",
                    $input_pair.as_str().len(),
                    left_operation.sp.start(),
                    left_operation.sp.end(),
                    right_operation.sp.start(),
                    right_operation.sp.end(),
                ).as_str())
            )?;
            
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
            return Err(make_ast_error_from_pair(
                $input_pair.clone(), 
                format!("🟣 The list of pairs must be even. The last element is: {:?}", 
                last_element.clone().as_rule()).as_str()
            ))
        }

        left_operation
    }};
}

fn build_literal(pair: pest::iterators::Pair<Rule>) -> Result<Node<Expression>, Error<Rule>> {
    let literal = pair.clone().into_inner().next().unwrap();
    let res = match literal.as_rule() {
        Rule::boolean => Expression::Literal(Literal::Bool(literal.as_str().parse().unwrap())),
        Rule::float => {
            // need to check for potential overflow
            let float_value_for_test: f64 = literal.as_str().parse().unwrap();
            if float_value_for_test > std::f32::MAX as f64 {
                let message = format!(
                    "🔴 Float literal {} is too big. Max possible value is {}.", 
                    float_value_for_test, 
                    std::f32::MAX
                );
                return Err(make_ast_error_from_pair(pair, &message))
            } else if float_value_for_test < std::f32::MIN as f64 {
                let message = format!(
                    "🔴 Float literal {} is too small. Min possible value is {}.", 
                    float_value_for_test, 
                    std::f32::MIN
                );
                return Err(make_ast_error_from_pair(pair, &message))
            }
            // return correct value as f32
            Expression::Literal(Literal::Float(literal.as_str().parse().unwrap()))
        },
        Rule::char => {
            // convert &str containing single quotes to char
            let char_literal = literal.as_str();
            let char_trimmed = char_literal.trim_start_matches('\'').trim_end_matches('\'');
            let real_char = char_trimmed.chars().next().unwrap();
            let real_char_ascii = real_char as u8;
            Expression::Literal(Literal::Char(real_char_ascii))},
        Rule::integer => {
            // need to check for potential overflow
            let int_value_for_test: i64 = literal.as_str().parse().unwrap();
            if int_value_for_test > std::i16::MAX as i64 {
                let message = format!(
                    "🔴 Integer literal {} is too big. Max possible value is {}.", 
                    int_value_for_test, 
                    std::i16::MAX
                );
                return Err(make_ast_error_from_pair(pair, &message))
            } else if int_value_for_test < std::i16::MIN as i64 {
                let message = format!(
                    "🔴 Integer literal {} is too small. Min possible value is {}.", 
                    int_value_for_test, 
                    std::i16::MIN
                );
                return Err(make_ast_error_from_pair(pair, &message))
            }
            // return correct value as i32
            Expression::Literal(Literal::Int(literal.as_str().parse().unwrap()))
        },
        _ => {
            let message = format!("🔴 Unexpected rule in <literal> match tree: {:?}", literal.as_rule());
            return Err(make_ast_error_from_pair(pair, &message))
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
            return Err(make_ast_error_from_pair(pair, &message))
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
            return Err(make_ast_error_from_pair(pair, &message))
        },
    }
}

pub fn build_get_or_set_value(pair: pest::iterators::Pair<Rule>) -> Result<Node<GetOrSetValue>, Error<Rule>> {
    let mut inner = pair.clone().into_inner();
    let identifier = unwrap_or_err_panic!(build_identifier(inner.next().unwrap()));
    let index = match inner.next() {
        Some(pair) => Some(Box::new(unwrap_or_err_panic!(build_expression(pair)))),
        None => None,
    };
    ok_build_node!(pair, GetOrSetValue {
        identifier,
        index,
    })
}

pub fn build_identifier(pair: pest::iterators::Pair<Rule>) -> Result<Node<Identifier>, Error<Rule>> {
    let identifier = pair.clone().as_str().to_string();
    ok_build_node!(pair, Identifier { name: identifier })
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
            let identifier = unwrap_or_err_panic!(build_identifier(inner.next().unwrap()));

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
            let get_or_set_value = build_get_or_set_value(pair.clone())?;
            ok_build_node!(pair, Expression::GetOrSetValue(get_or_set_value.data))
        },
        _ => {
            let message = format!("🔴 Unexpected rule in <expression> match tree: {:?}", rule);
            return Err(make_ast_error_from_pair(pair, &message))
        },
    }
}