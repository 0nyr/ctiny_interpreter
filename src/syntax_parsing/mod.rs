use crate::params;

mod parser;

pub fn syntax_parsing(input_files: Vec<std::path::PathBuf>) {
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
        let ast = parser::parse(file_content);
        match ast {
            Ok(_) => {
                log::info!("Parsing successful for file {}!", file_name);
            },
            Err(e) => {
                log::error!("Parsing error: {}", e);
                continue;
            }
        }
            
    }
    
}