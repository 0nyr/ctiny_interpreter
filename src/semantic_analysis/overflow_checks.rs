use pest::Span;

use super::errors::{SemanticError, IntOverflowError, SemanticErrorTrait, FloatOverflowError};



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
        false => Ok(result),
        true => Err(SemanticError::FloatOverflow(
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