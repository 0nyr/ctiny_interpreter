use crate::params;
use crate::syntax_parsing; // self allows to use the module name
use crate::syntax_parsing::Rule;
use crate::abstract_syntax_tree::build_translation_unit;
use crate::abstract_syntax_tree::nodes::AST;

fn parse_content_into_ast<'a>(
    file_content: &'a str,
    file_name: &str
) -> AST {
    // syntax parsing
    let rule = Rule::translation_unit;
    let pairs =  syntax_parsing::parse(rule, file_content)
        .unwrap_or_else(|error| { 
            log::error!("🚧 Syntax parsing ERROR [f: {}]: \n {}\n", file_name, error);
            panic!();
        });

    let first_pair = pairs.into_iter().next().unwrap();
    assert_eq!(first_pair.as_rule(), rule);
    assert_eq!(first_pair.as_str(), file_content);

    // AST conversion
    // WARN: don't forget to change the method if needed
    let ast: AST = build_translation_unit(first_pair)
        .unwrap_or_else(|error| { 
            log::error!("🚧 AST ERROR [f: {}]: \n {}\n", file_name, error);
            panic!(); 
        });
    
    log::info!("Syntax Parsing successful for file {}!", file_name);
    ast
}

/// For all input files:
///     1. Perform syntax parsing
///     2. Construct an AST
///     3. Perform semantic analysis
pub fn pipeline_syntax_and_ast(input_files: Vec<std::path::PathBuf>) {
    println!("Pipeline: {:#?}", params::argv::Pipeline::SyntaxAndASTParsing);

    // run syntax parsing on all input files
    for file in &input_files {
        log::info!("Parsing file: {}", file.to_str().unwrap());
        let file_name = file.file_name().unwrap().to_str().unwrap();
        let file_content = std::fs::read_to_string(file).unwrap();
        let file_content_str = file_content.as_str();
        let ast = parse_content_into_ast(file_content_str, file_name);
        log::info!("AST: {:#?}", ast);
    }
}