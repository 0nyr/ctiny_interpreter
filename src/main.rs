#[macro_use]
extern crate pest_derive;

// link modules
mod params;
mod data_loading;
mod syntax_parsing;
mod abstract_syntax_tree;
mod semantic_analysis;
mod pipelines;

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
        params::argv::Pipeline::TypeOverflowChecking => {
            pipelines::pipeline_syntax_and_ast(input_paths);
        },
    }
    
}
