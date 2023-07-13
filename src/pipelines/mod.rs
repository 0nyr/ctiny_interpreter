use crate::params;
use crate::syntax_parsing; // self allows to use the module name
use crate::syntax_parsing::Rule;
use crate::abstract_syntax_tree::build_translation_unit;
use crate::abstract_syntax_tree::nodes::AST;

pub fn parse_content_into_ast<'a>(
    file_content: &'a str,
    file_name: Option<&str>,
) -> AST<'a> {
    let optional_file_annotation = match file_name {
        Some(file_name) => format!(" [f: {}]", file_name),
        None => String::from(""),
    };

    // syntax parsing
    let rule = Rule::translation_unit;
    let pairs =  syntax_parsing::parse(rule, file_content)
        .unwrap_or_else(|error| { 
            log::error!("🚧 Syntax parsing ERROR{}: \n {}\n", optional_file_annotation, error);
            panic!();
        });

    let first_pair = pairs.into_iter().next().unwrap();
    assert_eq!(first_pair.as_rule(), rule);
    assert_eq!(first_pair.as_str(), file_content);

    // AST conversion
    // WARN: don't forget to change the method if needed
    let ast: AST = build_translation_unit(first_pair)
        .unwrap_or_else(|error| { 
            log::error!("🚧 AST ERROR{}: \n {}\n", optional_file_annotation, error);
            panic!(); 
        });
    
    if let Some(file_name) = file_name {
        log::info!("Syntax Parsing successful for file {}!", file_name);
    } else {
        log::info!("Syntax Parsing successful for file content!");
    }

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
        let ast = parse_content_into_ast(file_content_str, Some(file_name));
        log::info!("AST: {:#?}", ast);
    }
}