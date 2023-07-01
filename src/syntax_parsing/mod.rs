use crate::params;

pub fn syntax_parsing(input_files: Vec<std::path::PathBuf>) {
    println!("{:#?}", params::argv::Pipeline::SyntaxParsing);

    // log all input files
    for file in input_files {
        log::info!("Input file: {}", file.to_str().unwrap());
    }
}