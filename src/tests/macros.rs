


#[macro_export]
macro_rules! build_test {
    ($rule:expr, $build_function:ident,  $( $input_str:literal),* ) => {

        let input_strs = {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($input_str);
            )*
            temp_vec
        };

        for test_str in input_strs {
            // Syntax parsing
            let pairs = CTinyParser::parse($rule, test_str).unwrap();

            let first_pair = pairs.into_iter().next().unwrap();
            assert_eq!(first_pair.as_rule(), $rule);
            assert_eq!(first_pair.as_str(), test_str);

            // AST conversion
            // WARN: don't forget to change the method if needed
            let ast = $build_function(first_pair)
                .unwrap_or_else(|error| { 
                    print!("AST ERROR for {}: \n {}\n", test_str, error); 
                    panic!(); 
                });
            print!("AST for string \"{}\": \n {:#?} \n\n", test_str, ast);
        }
    }
}