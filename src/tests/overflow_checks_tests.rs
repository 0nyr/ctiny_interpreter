use crate::semantic_analysis::overflow_checks::*;

macro_rules! build_safe_operation_test {
    ($test_name:ident, $function:ident, $left:expr, $right:expr, $debug_expr:expr) => {
        #[test]
        fn $test_name() {
            // positive test
            let left = $left;
            let right = $right;
            let debug_str = stringify!($debug_str);
            let common_debug_span = pest::Span::new(debug_str, 0, debug_str.len()).unwrap();

            let result = $function(left, right, common_debug_span);

            match result {
                Ok(value) => {
                    assert_eq!(
                        value, 
                        $debug_expr, 
                        "Result value <{}> not identical to provided debug expression <{}>",
                        value, $debug_expr
                    );
                    println!("Result value <{}> identical to provided debug expression <{}>", value, $debug_expr);
                }
                Err(error) => {
                    // print error and panic
                    panic!(
                        "Error <{}> occured while evaluating expression <{}>",
                        error, $debug_expr
                    );
                }
            }
        }
    };
    ($test_name:ident, $function:ident, $left:expr, $right:expr, $debug_str:expr, false) => {
        // negative case
        #[test]
        fn $test_name() {
            let left = $left;
            let right = $right;
            let debug_str = $debug_str;
            let common_debug_span = pest::Span::new(debug_str, 0, debug_str.len()).unwrap();

            let result = $function(left, right, common_debug_span);

            match result {
                Ok(value) => {
                    // no error, but we expected one
                    panic!(
                        "No error occured while evaluating expression <{}>, but we expected one. Result value: <{}>",
                        debug_str, value
                    );
                }
                Err(error) => {
                    // expected error, just print it
                    println!("Expected error occured while evaluating expression <{}>: <{}>", debug_str, error);
                }
            }
        }
    };
}

// int addition tests
build_safe_operation_test!(
    test_safe_int_add_positive,
    safe_int_add,
    1, 
    1,
    1+1
);

build_safe_operation_test!(
    test_safe_int_add_overflow,
    safe_int_add,
    i16::MAX, 
    i16::MAX,
    "i16::MAX+i16::MAX",
    false
);

// float addition tests
build_safe_operation_test!(
    test_safe_float_add_positive,
    safe_float_add,
    1.0, 
    1.0,
    1.0+1.0
);

build_safe_operation_test!(
    test_safe_float_add_overflow,
    safe_float_add,
    f32::MAX, 
    f32::MAX,
    "f32::MAX+f32::MAX",
    false
);

// int subtraction tests
build_safe_operation_test!(
    test_safe_int_subtract_positive,
    safe_int_subtract,
    10, 
    5,
    10-5
);

build_safe_operation_test!(
    test_safe_int_subtract_underflow,
    safe_int_subtract,
    i16::MIN, 
    1,
    "i16::MIN-1",
    false
);

// float subtraction tests
build_safe_operation_test!(
    test_safe_float_subtract_positive,
    safe_float_subtract,
    10.0, 
    5.0,
    10.0-5.0
);

build_safe_operation_test!(
    test_safe_float_subtract_overflow,
    safe_float_subtract,
    f32::MIN, 
    f32::MAX,
    "f32::MIN-f32::MAX",
    false
);

// int multiplication tests
build_safe_operation_test!(
    test_safe_int_multiply_positive,
    safe_int_multiply,
    10, 
    5,
    10*5
);

build_safe_operation_test!(
    test_safe_int_multiply_overflow,
    safe_int_multiply,
    i16::MAX, 
    2,
    "i16::MAX*2",
    false
);

// float multiplication tests
build_safe_operation_test!(
    test_safe_float_multiply_positive,
    safe_float_multiply,
    10.0, 
    5.0,
    10.0*5.0
);

build_safe_operation_test!(
    test_safe_float_multiply_overflow,
    safe_float_multiply,
    f32::MAX, 
    2.0,
    "f32::MAX*2.0",
    false
);

// int division tests
build_safe_operation_test!(
    test_safe_int_divide_positive,
    safe_int_divide,
    10, 
    5,
    10/5
);

build_safe_operation_test!(
    test_safe_int_divide_zero,
    safe_int_divide,
    10, 
    0,
    "10/0",
    false
);

// float division tests
build_safe_operation_test!(
    test_safe_float_divide_positive,
    safe_float_divide,
    10.0, 
    5.0,
    10.0/5.0
);

build_safe_operation_test!(
    test_safe_float_divide_zero,
    safe_float_divide,
    10.0, 
    0.0,
    "10.0/0.0",
    false
);

// int modulo tests
build_safe_operation_test!(
    test_safe_int_modulo_positive,
    safe_int_modulo,
    10, 
    3,
    10%3
);

build_safe_operation_test!(
    test_safe_int_modulo_zero,
    safe_int_modulo,
    10, 
    0,
    "10%0",
    false
);
