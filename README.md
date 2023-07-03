# C tiny programming paradigm project

### project instructions

1. (20 points) Write the EBNF style grammar for Tiny and Implement a parser for this grammar (should check for valid programs).

My EBNF style grammar is described in the file `ctiny_grammar.txt` . You should read first `ctiny_grammar_info.md` so as to get insights on **Ctiny** as well as my notations and design choices.

To run my parser program written in **Rust**, use the following command: `cargo run -- --parse <file_path.ctiny>` or `cargo run -- -p <file_path.ctiny>`

2. (30 points) Implement a static type system to check for Overflows (integer and buffer). The input for your implementation would be a program written in Tiny and the output should dictate integer overflows or buffer overflows (if there is any in the input program).

To run my type overflow checker written in **Rust**, use the following command: `cargo run -- --check <file_path.ctiny>` or `cargo run -- -c <file_path.ctiny>`

## commands

#### run program

`cargo run -- -h`: display command line help

```shell
Usage: tiny_ebnf_grammar_parser [OPTIONS]

Options:
  -f, --files <FILES>              File path to input files
  -d, --directories <DIRECTORIES>  The directory containing the input files
  -p, --pipeline <PIPELINE>        The pipeline to run [default: syntax-parsing] [possible values: syntax-parsing, type-overflow-checking]
  -h, --help                       Print help (see more with '--help')
  -V, --version                    Print version
```

`cargo run -- -p syntax-parsing -f res/building_grammar/example_1.ctiny`: run syntax-parsing pipeline on provided `.ctiny` file.

`cargo run -- -p syntax-parsing -d res/invalid/`: run syntax-parsing pipeline on all `.ctiny` files inside the provided directory and sub-directories.

#### testing

`cargo test`: run all test (stdout is captured, i.e. not displayed)

`cargo test -- --nocapture`: run tests without capturing stdout. Since it can be messy, prefer to use `--nocapture` while running only a specific test.

`cargo test test_function_call -- --nocapture`: run a specific test called ``test_function_call`` and displays its stdout.
