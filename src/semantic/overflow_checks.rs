use pest::Span;

use super::errors::{SemanticError, IntOverflowError, SemanticErrorTrait, FloatOverflowError, DivisionByZeroError};

pub fn safe_int_add<'a>(left: i16, right: i16, common_span: Span<'a>) -> Result<i16, SemanticError> {
    let result = left.checked_add(right);
    match result {
        Some(result) => Ok(result),
        None => Err(SemanticError::IntOverflow(
            IntOverflowError::init(
                common_span,
                format!(
                    "Int addition overflow: {} + {}", left, right
                ).as_str()
            )
        )),
    }
}

pub fn safe_float_add<'a>(left: f32, right: f32, common_span: Span<'a>) -> Result<f32, SemanticError> {
    let result = left + right;
    match result.is_finite() {
        true => Ok(result),
        false => Err(SemanticError::FloatOverflow(
            FloatOverflowError::init(
                common_span,
                format!(
                    "Float addition overflow: {} + {} would result in infinite value.", 
                    left, right
                ).as_str()
            )
        )),
    }
}

pub fn safe_int_subtract<'a>(left: i16, right: i16, common_span: Span<'a>) -> Result<i16, SemanticError> {
    let result = left.checked_sub(right);
    match result {
        Some(result) => Ok(result),
        None => Err(SemanticError::IntOverflow(
            IntOverflowError::init(
                common_span,
                format!("Int subtraction overflow: {} - {}", left, right).as_str(),
            ),
        )),
    }
}

pub fn safe_float_subtract<'a>(left: f32, right: f32, common_span: Span<'a>) -> Result<f32, SemanticError> {
    let result = left - right;
    match result.is_finite() {
        true => Ok(result),
        false => Err(SemanticError::FloatOverflow(
            FloatOverflowError::init(
                common_span,
                format!(
                    "Float subtraction overflow: {} - {} would result in infinite value.",
                    left, right
                )
                .as_str(),
            ),
        )),
    }
}

pub fn safe_int_multiply<'a>(left: i16, right: i16, common_span: Span<'a>) -> Result<i16, SemanticError> {
    let result = left.checked_mul(right);
    match result {
        Some(result) => Ok(result),
        None => Err(SemanticError::IntOverflow(
            IntOverflowError::init(
                common_span,
                format!("Int multiplication overflow: {} * {}", left, right).as_str(),
            ),
        )),
    }
}

pub fn safe_float_multiply<'a>(left: f32, right: f32, common_span: Span<'a>) -> Result<f32, SemanticError> {
    let result = left * right;
    match result.is_finite() {
        true => Ok(result),
        false => Err(SemanticError::FloatOverflow(
            FloatOverflowError::init(
                common_span,
                format!(
                    "Float multiplication overflow: {} * {} would result in infinite value.",
                    left, right
                )
                .as_str(),
            ),
        )),
    }
}

pub fn safe_int_divide<'a>(left: i16, right: i16, common_span: Span<'a>) -> Result<i16, SemanticError> {
    if right == 0 {
        Err(SemanticError::DivisionByZero(
            DivisionByZeroError::init(
                common_span,
                "Division by zero",
            ),
        ))
    } else {
        let result = left.checked_div(right);
        match result {
            Some(result) => Ok(result),
            None => Err(SemanticError::IntOverflow(
                IntOverflowError::init(
                    common_span,
                    format!("Int division overflow: {} / {}", left, right).as_str(),
                ),
            )),
        }
    }
}

pub fn safe_float_divide<'a>(left: f32, right: f32, common_span: Span<'a>) -> Result<f32, SemanticError> {
    if right == 0.0 {
        Err(SemanticError::DivisionByZero(
            DivisionByZeroError::init(
                common_span,
                "Division by zero",
            ),
        ))
    } else {
        let result = left / right;
        match result.is_finite() {
            true => Ok(result),
            false => Err(SemanticError::FloatOverflow(
                FloatOverflowError::init(
                    common_span,
                    format!(
                        "Float division overflow: {} / {} would result in infinite value.",
                        left, right
                    )
                    .as_str(),
                ),
            )),
        }
    }
}

pub fn safe_int_modulo<'a>(left: i16, right: i16, common_span: Span<'a>) -> Result<i16, SemanticError> {
    if right == 0 {
        Err(SemanticError::DivisionByZero(
            DivisionByZeroError::init(
                common_span,
                "Modulo by zero",
            ),
        ))
    } else {
        let result = left.checked_rem(right);
        match result {
            Some(result) => Ok(result),
            None => Err(SemanticError::IntOverflow(
                IntOverflowError::init(
                    common_span,
                    format!("Int modulo overflow: {} % {}", left, right).as_str(),
                ),
            )),
        }
    }
}
