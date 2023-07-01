use std::path::PathBuf;

// link modules
mod params;
mod syntax_parsing;
mod type_overflow_checking;

pub fn recursive_get_all_ctiny_file_paths(path: PathBuf) -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = Vec::new();
    if path.is_dir() {
        for entry in std::fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                paths.append(&mut recursive_get_all_ctiny_file_paths(path));
            } else {
                if path.extension().and_then(std::ffi::OsStr::to_str) == Some("ctiny") {
                    paths.push(path);
                }
            }
        }
    } else {
        if path.extension().unwrap() == "ctiny" {
            paths.push(path);
        }
    }
    return paths;
}

fn main() {
    crate::params::init();

    // get input files
    let mut input_paths: Vec<std::path::PathBuf> = Vec::new();
    if params::ARGV.files.is_some() {
        let files = params::ARGV.files.as_ref().unwrap();
        for (_, file) in files.iter().enumerate() {
            input_paths.push(PathBuf::from(file));
        }
    } else if params::ARGV.directories.is_some() {
        for path in params::ARGV.directories.as_ref().unwrap() {
            input_paths.push(PathBuf::from(path));
        }
    } else {
        // default
        input_paths.push(params::DEFAULT_INPUT_DIR_PATH.clone());
    }

    // test all provided paths
    for path in input_paths.clone() {
        if !path.exists() {
            panic!("🚩 The path doesn't exist: {}", path.to_str().unwrap());
        }
    }

    // if files are provided, no need to do anything
    // if directories are provided, get all files in the directories ending with .ctiny
    // also explore subdirectories for files ending with .ctiny
    // remove the directories from the input_paths and add the files
    if !input_paths.is_empty() {
        let mut new_input_paths: Vec<std::path::PathBuf> = Vec::new();
        for path in input_paths.clone() {
            recursive_get_all_ctiny_file_paths(path).iter().for_each(|path| {
                new_input_paths.push(path.clone());
            });
        }
        input_paths = new_input_paths;
    }

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
