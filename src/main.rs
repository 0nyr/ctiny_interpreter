#[macro_use]
extern crate pest_derive;

// link modules
mod params;
mod errors;
mod data_loading;
mod syntax_parsing;
mod abstract_syntax_tree;
mod semantic;
mod pipelines;
mod symbol_table;
mod interpretation;

#[cfg(test)]
mod tests;


fn main() {
    crate::params::init();

    // get input file paths
    //let input_paths: Vec<PathBuf> = Vec::new();
    let input_paths = data_loading::get_input_files_from_params();

    // run the pipeline
    match params::ARGV.pipeline {
        params::argv::Pipeline::SyntaxAndASTParsing => {
            pipelines::pipeline_syntax_and_ast(input_paths);
        },
        params::argv::Pipeline::SyntaxASTAndInterpretation => {
            pipelines::pipeline_syntax_ast_interpretation(input_paths);
        },
    }
    
}
