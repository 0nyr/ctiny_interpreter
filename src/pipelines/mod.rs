use crate::interpretation::interpret_function::interpret_translation_unit;
use crate::params;
use crate::semantic::errors::{SemanticError, SyntaxParsingError, ASTBuildingError};
use crate::symbol_table::build_static_symbol_table;
use crate::syntax_parsing; // self allows to use the module name
use crate::syntax_parsing::Rule;
use crate::abstract_syntax_tree::build_translation_unit;
use crate::abstract_syntax_tree::nodes::AST;

pub fn parse_content_into_ast<'a>(
    file_content: &'a str,
    file_name: Option<&str>,
) -> Result<AST<'a>, SemanticError> {
    // syntax parsing
    let rule = Rule::translation_unit;
    let pairs =  syntax_parsing::parse(rule, file_content)
        .map_err(|error| {
            SemanticError::SyntaxParsing(
                SyntaxParsingError::from(error)
            )
    })?;

    let first_pair = pairs.into_iter().next().unwrap();
    assert_eq!(first_pair.as_rule(), rule);
    assert_eq!(first_pair.as_str(), file_content);

    // AST conversion
    // WARN: don't forget to change the method if needed
    let ast: AST = build_translation_unit(first_pair)
        .map_err(|error| {
            SemanticError::ASTBuilding(
                ASTBuildingError::from(error)
            )
        })?;
    
    if let Some(file_name) = file_name {
        log::info!("Syntax Parsing successful for file {}!", file_name);
    } else {
        log::info!("Syntax Parsing successful for file content!");
    }

    Ok(ast)
}

/// For all input files:
///     1. Perform syntax parsing
///     2. Construct an AST
pub fn pipeline_syntax_and_ast(
    input_files: Vec<std::path::PathBuf>
) {
    println!("Pipeline: {:#?}", params::argv::Pipeline::SyntaxAndASTParsing);

    // run syntax parsing on all input files
    for file in &input_files {
        log::info!("Parsing file: {}", file.to_str().unwrap());
        let file_name = file.file_name().unwrap().to_str().unwrap();
        let file_content = std::fs::read_to_string(file).unwrap();
        let file_content_str = file_content.as_str();
        let ast = parse_content_into_ast(file_content_str, Some(file_name));
        match ast {
            Ok(ast) => {
                if params::ARGV.display_ast {
                    log::info!("AST: {:#?}", ast);
                }
            },
            Err(error) => {
                log::error!("🚧 Syntax Parsing ERROR: \n {}\n", error);
                continue;
            },
        }
    }
}

/// For all input files:
///     1. Perform syntax parsing
///     2. Construct an AST
///     3. Perform interpretation
pub fn pipeline_syntax_ast_interpretation(input_files: Vec<std::path::PathBuf>) {
    println!("Pipeline: {:#?}", params::argv::Pipeline::SyntaxASTAndInterpretation);

    // run syntax parsing on all input files
    for file in &input_files {
        log::info!("Parsing file: {}", file.to_str().unwrap());

        // syntax parsing into AST
        let file_name = file.file_name().unwrap().to_str().unwrap();
        let file_content = std::fs::read_to_string(file).unwrap();
        let file_content_str = file_content.as_str();
        let ast = match parse_content_into_ast(file_content_str, Some(file_name)) {
            Ok(ast) => {
                if params::ARGV.display_ast {
                    log::info!("AST: {:#?}", ast);
                }
                ast
            },
            Err(error) => {
                log::error!("🚧 Syntax or AST Parsing ERROR: \n {}\n", error);
                continue;
            },
        };

        // build symbol table
        let mut symbol_table = build_static_symbol_table(&ast);

        // interpretation
        let res = interpret_translation_unit(
            &ast,
            &mut symbol_table,
        );
        match res {
            Ok(program_return_value) => {
                log::info!(")Program return value: {}", program_return_value.data);
            },
            Err(error) => {
                log::error!("🚧 Interpretation ERROR: \n {}\n", error);
                continue;
            },
        }
    }
}