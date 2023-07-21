use crate::params;

use crate::abstract_syntax_tree::nodes::AST;

use self::errors::SemanticError;

pub mod errors;
pub mod type_casts;
pub mod overflow_checks;
pub mod operations;

pub fn overflow_checking(input_files: Vec<std::path::PathBuf>) {
    println!("{:#?}", params::argv::Pipeline::TypeOverflowChecking);

    // log all input files
    for file in input_files {
        log::info!("Input file: {}", file.to_str().unwrap());
    }
}

pub fn semantic_analysis(ast: AST) -> Result<(), Vec<SemanticError>> {
    println!("{:#?}", params::argv::Pipeline::SemanticAnalysis);

    return Ok(());
}