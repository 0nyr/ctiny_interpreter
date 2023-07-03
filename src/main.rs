#[macro_use]
extern crate pest_derive;

// link modules
mod params;
mod data_loading;
mod syntax_parsing;
mod type_overflow_checking;
mod tests;


fn main() {
    crate::params::init();

    // get input file paths
    //let input_paths: Vec<PathBuf> = Vec::new();
    let input_paths = data_loading::get_input_files_from_params();

    // run the pipeline
    match params::ARGV.pipeline {
        params::argv::Pipeline::SyntaxParsing => {
            syntax_parsing::syntax_parsing(input_paths);
        },
        params::argv::Pipeline::TypeOverflowChecking => {
            type_overflow_checking::overflow_checking(input_paths);
        },
    }
    
}
