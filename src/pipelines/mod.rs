use pest::error::Error;

use crate::params;
use crate::syntax_parsing::parser;
use crate::syntax_parsing::parser::Rule;
use crate::syntax_tree::nodes::{Node, ProgramAST};
//use crate::syntax_tree::convert_to_ast;

fn parse_content_into_ast<'a>(
    file_content: &'a str,
    file_name: &str
) -> Result<(), Error<Rule>> {
    let parsing_result = parser::parse(file_content);
    match parsing_result {
        // Ok(pairs) => {
        //     log::info!("Basic Parsing successful [f: {}]!", file_name);
        //     convert_to_ast(pairs)
        // },
        Ok(_) => {
            // TODO: to be removed
            Ok(())
        },
        Err(e) => {
            log::error!("Basic Parsing error f: {}]: {}", file_name, e);
            Err(e)
        }
    }
}

/// For all input files:
///     1. Perform syntax parsing
///     2. Construct an AST
///     3. Perform semantic analysis
pub fn parsing_and_semantic_analysis(input_files: Vec<std::path::PathBuf>) {
    println!("{:#?}", params::argv::Pipeline::SyntaxParsing);

    // log all input files
    for file in &input_files {
        log::info!("Input file: {}", (*file).to_str().unwrap());
    }

    // run syntax parsing on all input files
    for file in &input_files {
        log::info!("Parsing file: {}", file.to_str().unwrap());
        let file_name = file.file_name().unwrap().to_str().unwrap();
        let file_content = std::fs::read_to_string(file).unwrap();
        let file_content_str = file_content.as_str();
        let ast = parse_content_into_ast(file_content_str, file_name);

        // for now, just print the AST
        match ast {
            Ok(ast) => {
                log::info!("AST: {:#?}", ast);
            },
            Err(e) => {
                log::error!("AST building error [f: {}]: {}", file_name, e);
            }
        }
            
    }
    
}