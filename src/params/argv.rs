use clap::{Parser, ValueEnum, arg};

// NOTE: 'group = "file_input_group"' means that only one of the options in the group can be used
// the result is stored always in 'files_input', and on the option used (the other is None)
// NOTE: the "///" comments are used to generate the help message
/// Program arguments
/// NOTE : to add multiple files/folders duplicate the flags before the files/folders, 
/// like : cargo run -- -f /path/to/file1 -f /path/to/file2
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Argv {
    /// File path to input files
    #[arg(short, long, required = false, group = "file_input_group")]
    pub files: Option<Vec<String>>,

    /// The directory containing the input files
    #[arg(short, long, required = false, group = "file_input_group")]
    pub directories: Option<Vec<String>>, 

    /// The pipeline to run
    #[arg(value_enum, short, long, default_value = "syntax-and-ast-parsing")]
    pub pipeline: Pipeline,

    /// whether to print the AST or not
    #[arg(long, default_value = "false")]
    pub display_ast: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Pipeline {
    /// run the syntax parsing
    SyntaxAndASTParsing,
    // semantic analysis
    SyntaxASTAndInterpretation,
}

pub fn get_program_args() -> Argv {
    return Argv::parse();
}