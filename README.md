# C tiny programming paradigm project

### project instructions

1. (20 points) Write the EBNF style grammar for Tiny and Implement a parser for this grammar (should check for valid programs).

My EBNF style grammar is described in the file `ctiny_grammar.md`

To run my parser program written in **Rust**, use the following command: `cargo run -- --parse <file_path.ctiny>` or `cargo run -- -p <file_path.ctiny>`

2. (30 points) Implement a static type system to check for Overflows (integer and buffer). The input for your implementation would be a program written in Tiny and the output should dictate integer overflows or buffer overflows (if there is any in the input program).

To run my type overflow checker written in **Rust**, use the following command: `cargo run -- --check <file_path.ctiny>` or `cargo run -- -c <file_path.ctiny>`
