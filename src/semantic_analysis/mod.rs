use crate::params;

pub fn overflow_checking(input_files: Vec<std::path::PathBuf>) {
    println!("{:#?}", params::argv::Pipeline::TypeOverflowChecking);

    // log all input files
    for file in input_files {
        log::info!("Input file: {}", file.to_str().unwrap());
    }
}