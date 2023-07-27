# C tiny programming paradigm project

### project instructions

1. *(20 points) Write the EBNF style grammar for Tiny and Implement a parser for this grammar (should check for valid programs).*

My EBNF style grammar is described in the file `ctiny_grammar.md` . You should read the full file first so as to get insights on **Ctiny** as well as my notations and design choices.

To run the **parser** for syntax error and invalid program detection, use the following command: `cargo run -- -p syntax-and-ast-parsing -f <file.ctiny>`. This also works with `-d <dir/**/*.ctiny>` for parsing a full directory.

2. *(30 points) Implement a static type system to check for Overflows (integer and buffer). The input for your implementation would be a program written in Tiny and the output should dictate integer overflows or buffer overflows (if there is any in the input program).*

To run the **interpreter** for overflow checking and other runtime error detection, use the following command: `cargo run -- -p syntax-ast-and-interpretation <file.ctiny>`. This also works with `-d <dir/**/*.ctiny>` for parsing a full directory.

## commands

> To run developer commands on the project root, you should install a rust compiler and tool chain. Follow instructions [here](https://www.rust-lang.org/tools/install). It's one command on linux.
>
> **NOTE:** In case you **don't want to install Rust**, you can use the compiled release directly. Replace `cargo run --`, with `./ctiny_interpreter` instead in the following commands. Go to the example section for live examples.

#### run program

`cargo run -- -h`: display command line help

```shell
Usage: tiny_ebnf_grammar_parser [OPTIONS]

Options:
  -f, --files <FILES>              File path to input files
  -d, --directories <DIRECTORIES>  The directory containing the input files
  -p, --pipeline <PIPELINE>        The pipeline to run [default: syntax-and-ast-parsing] [possible values: syntax-and-ast-parsing, syntax-ast-and-interpretation]
      --display-ast                whether to print the AST or not
  -h, --help                       Print help (see more with '--help')
  -V, --version                    Print version
```

`cargo run -- -p syntax-and-ast-parsing -f res/building_grammar/example_1.ctiny`: run syntax-parsing (including AST building) pipeline on provided `.ctiny` file.

`cargo run -- -p syntax-ast-and-interpretation -d res/invalid/`: run syntax-parsing, AST and interpretation (for overflow checking) pipeline on all `.ctiny` files inside the provided directory and sub-directories.

#### testing

`cargo test`: run all test (stdout is captured, i.e. not displayed). Hundred of tests are ensuring the quality of the code and handling of corner cases.

```shell
[...]
test result: ok. 261 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
```

`cargo test -- --nocapture`: run tests without capturing stdout. Since it can be messy, prefer to use `--nocapture` while running only a specific test.

`cargo test test_function_call -- --nocapture`: run a specific test called ``test_function_call`` and displays its stdout.

## Execution example of test files

> The `res/` directory contains some example programs for illustrations. But you can find more precise corner cases inside `src/tests`, which contains 250+ tests.

##### Run interpreter on valid programs

```shell
 ❮onyr ★ kenzael❯ ❮ctiny_programing_paradigm_project❯❯ ./ctiny_interpreter -p syntax-ast-and-interpretation -d res/valid/
Loading .env file... Initializing logger... [2023-07-27T17:36:14 UTC][INFO ctiny_interpreter::params]  🚀 starting program...
Pipeline: SyntaxASTAndInterpretation
[2023-07-27T17:36:14 UTC][INFO ctiny_interpreter::pipelines] Parsing file: res/valid/valid_program_c.ctiny
[2023-07-27T17:36:14 UTC][INFO ctiny_interpreter::pipelines] Syntax Parsing successful for file valid_program_c.ctiny!
[2023-07-27T17:36:14 UTC][INFO ctiny_interpreter::pipelines] )Program return value: 30 (Int)
[2023-07-27T17:36:14 UTC][INFO ctiny_interpreter::pipelines] Parsing file: res/valid/valid_program_a.ctiny
[2023-07-27T17:36:14 UTC][INFO ctiny_interpreter::pipelines] Syntax Parsing successful for file valid_program_a.ctiny!
[2023-07-27T17:36:14 UTC][INFO ctiny_interpreter::pipelines] )Program return value: 0 (Int)
[2023-07-27T17:36:14 UTC][INFO ctiny_interpreter::pipelines] Parsing file: res/valid/valid_program_d.ctiny
[2023-07-27T17:36:14 UTC][INFO ctiny_interpreter::pipelines] Syntax Parsing successful for file valid_program_d.ctiny!
[2023-07-27T17:36:14 UTC][INFO ctiny_interpreter::pipelines] )Program return value: 9 (Int)
[2023-07-27T17:36:14 UTC][INFO ctiny_interpreter::pipelines] Parsing file: res/valid/valid_program_e.ctiny
[2023-07-27T17:36:14 UTC][INFO ctiny_interpreter::pipelines] Syntax Parsing successful for file valid_program_e.ctiny!
[2023-07-27T17:36:14 UTC][INFO ctiny_interpreter::pipelines] )Program return value: 55 (Int)
[2023-07-27T17:36:14 UTC][INFO ctiny_interpreter::pipelines] Parsing file: res/valid/valid_program_b.ctiny
[2023-07-27T17:36:14 UTC][INFO ctiny_interpreter::pipelines] Syntax Parsing successful for file valid_program_b.ctiny!
[2023-07-27T17:36:14 UTC][INFO ctiny_interpreter::pipelines] )Program return value: 0 (Int)
```

##### Run parser on invalid program

```shell
 ❮onyr ★ kenzael❯ ❮ctiny_programing_paradigm_project❯❯ ./ctiny_interpreter -p syntax-and-ast-parsing -d res/invalid_syntax/
Loading .env file... Initializing logger... [2023-07-27T17:39:02 UTC][INFO ctiny_interpreter::params]  🚀 starting program...
Pipeline: SyntaxAndASTParsing
[2023-07-27T17:39:02 UTC][INFO ctiny_interpreter::pipelines] Parsing file: res/invalid_syntax/invalid_program_a.ctiny
[2023-07-27T17:39:02 UTC][ERROR ctiny_interpreter::pipelines] 🚧 Syntax Parsing ERROR: 
  --> 6:5
  |
6 |     char c; //error: declaration must be at the beginning
  |     ^---
  |
  = expected function_return or statement

[2023-07-27T17:39:02 UTC][INFO ctiny_interpreter::pipelines] Parsing file: res/invalid_syntax/invalid_program_c.ctiny
[2023-07-27T17:39:02 UTC][ERROR ctiny_interpreter::pipelines] 🚧 Syntax Parsing ERROR: 
  --> 6:5
  |
6 |     int a; // error: definition not allowed after assignment
  |     ^---
  |
  = expected function_return or statement

[2023-07-27T17:39:02 UTC][INFO ctiny_interpreter::pipelines] Parsing file: res/invalid_syntax/invalid_program_e.ctiny
[2023-07-27T17:39:02 UTC][ERROR ctiny_interpreter::pipelines] 🚧 Syntax Parsing ERROR: 
  --> 8:1
  |
8 | } // error: missing return
  | ^---
  |
  = expected function_return or statement

[2023-07-27T17:39:02 UTC][INFO ctiny_interpreter::pipelines] Parsing file: res/invalid_syntax/invalid_program_d.ctiny
[2023-07-27T17:39:02 UTC][ERROR ctiny_interpreter::pipelines] 🚧 Syntax Parsing ERROR: 
  --> 1:1
  |
1 | int x; // error: global not allowed
  | ^---
  |
  = expected translation_unit

[2023-07-27T17:39:02 UTC][INFO ctiny_interpreter::pipelines] Parsing file: res/invalid_syntax/invalid_program_b.ctiny
[2023-07-27T17:39:02 UTC][ERROR ctiny_interpreter::pipelines] 🚧 Syntax Parsing ERROR: 
  --> 2:5
  |
2 |     int a = 2; // error: declaration should be in next lines
  |     ^---
  |
  = expected function_return, multi_declaration, or statement

```

##### Run interpreter on runtime error programs

```shell
 ❮onyr ★ kenzael❯ ❮ctiny_programing_paradigm_project❯❯ ./ctiny_interpreter -p syntax-ast-and-interpretation -d res/invalid_runtime
Loading .env file... Initializing logger... [2023-07-27T17:41:56 UTC][INFO ctiny_interpreter::params]  🚀 starting program...
Pipeline: SyntaxASTAndInterpretation
[2023-07-27T17:41:56 UTC][INFO ctiny_interpreter::pipelines] Parsing file: res/invalid_runtime/runtime_error_a.ctiny
[2023-07-27T17:41:56 UTC][INFO ctiny_interpreter::pipelines] Syntax Parsing successful for file runtime_error_a.ctiny!
[2023-07-27T17:41:56 UTC][ERROR ctiny_interpreter::pipelines] 🚧 Interpretation ERROR: 
  --> 6:9
  |
6 |     b = a; // runtime error: implicit conversion from int to char causing overflow
  |         ^
  |
  = 🔴 [Semantic error] Int literal out of range for char: 32767

[2023-07-27T17:41:56 UTC][INFO ctiny_interpreter::pipelines] Parsing file: res/invalid_runtime/runtime_error_d.ctiny
[2023-07-27T17:41:56 UTC][INFO ctiny_interpreter::pipelines] Syntax Parsing successful for file runtime_error_d.ctiny!
[2023-07-27T17:41:56 UTC][ERROR ctiny_interpreter::pipelines] 🚧 Interpretation ERROR: 
  --> 1:1
  |
1 | a
  | ^
  |
  = 🔴 [Semantic error] Variable a has not been assigned a value.

[2023-07-27T17:41:56 UTC][INFO ctiny_interpreter::pipelines] Parsing file: res/invalid_runtime/runtime_error_b.ctiny
[2023-07-27T17:41:56 UTC][INFO ctiny_interpreter::pipelines] Syntax Parsing successful for file runtime_error_b.ctiny!
[2023-07-27T17:41:56 UTC][ERROR ctiny_interpreter::pipelines] 🚧 Interpretation ERROR: 
  --> 1:1
  |
1 | a
  | ^
  |
  = 🔴 [Semantic error] Array a has not been assigned a value at index 0.

[2023-07-27T17:41:56 UTC][INFO ctiny_interpreter::pipelines] Parsing file: res/invalid_runtime/runtime_error_e.ctiny
[2023-07-27T17:41:56 UTC][INFO ctiny_interpreter::pipelines] Syntax Parsing successful for file runtime_error_e.ctiny!
[2023-07-27T17:41:56 UTC][ERROR ctiny_interpreter::pipelines] 🚧 Interpretation ERROR: 
  --> 9:9
  |
9 |     x = foo(1, 'a', 3, 4); // runtime error: foo has too many arguments
  |         ^---------------^
  |
  = 🔴 [Semantic error] Expected 3 arguments, got 4 for function foo

[2023-07-27T17:41:56 UTC][INFO ctiny_interpreter::pipelines] Parsing file: res/invalid_runtime/runtime_error_c.ctiny
[2023-07-27T17:41:56 UTC][INFO ctiny_interpreter::pipelines] Syntax Parsing successful for file runtime_error_c.ctiny!
[2023-07-27T17:41:56 UTC][ERROR ctiny_interpreter::pipelines] 🚧 Interpretation ERROR: 
  --> 7:5
  |
7 |     while(i > 0) {
  | ...
9 |     }␊
  |     ^
  |
  = 🔴 [Semantic error] Maximum number of loop iteration reached (max: 1000).
```
