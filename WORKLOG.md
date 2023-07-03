# WORKLOG

I have a concrete context-free grammar in EBNF form. I want to create a parser in Rust for this grammar. The goal of the parser is just to check whether or not an input program is valid. In case of syntaxt error, the parser should give the line of the error, as well as what is the error. How can I build such a basic parser in Rust ?

I have a base program layout that for now is just capable to take program file paths as program params, and call a parsing function that does nothing for now, and that I need to complete.

I want to use pest and [Parsing Expression Grammar (PEG)](https://en.wikipedia.org/wiki/Parsing_expression_grammar) 

Here is a file `src/syntax_parsing/mod.rs`:

```rust
use crate::params;

mod parser;

pub fn syntax_parsing(input_files: Vec<std::path::PathBuf>) {
    println!("{:#?}", params::argv::Pipeline::SyntaxParsing);

    // log all input files
    for file in input_files {
        log::info!("Input file: {}", file.to_str().unwrap());
    }

    // run syntax parsing on all input files
    for file in input_files {
        log::info!("Parsing file: {}", file.to_str().unwrap());
        let file_content = std::fs::read_to_string(file).unwrap();
        let ast = parser::parse(file_content);
        log::info!("AST: {:#?}", ast);
    }
}
```



### Ctiny support

* [X] declarations
* [X] ~~definitions~~ -> assignments
* [X]arithmetic operations
* [X] logical operations
* [X] int, float, char, bool
* [X] arrays
* [X] if-else
* [X] while loop
* [X] operations on arrays
* [X] entry point of the program must be `int main ()`
* [X] WARN: all declarations must happen before any assignment -> see `<block-statement>`
* [X] no global variables -> see `<translation-unit>`

### credits

My Ctiny grammar is inspired by:

* [The syntax of C in Backus-Naur Form](https://cs.wmich.edu/~gupta/teaching/cs4850/sumII06/The%20syntax%20of%20C%20in%20Backus-Naur%20form.htm), itself "adapted from Section A13 of  *The C programming language* , 2nd edition, by Brian W. Kernighan and Dennis M. Ritchie,Prentice Hall, 1988."
* *Programming Languages: Principles and Paradigms*, second edition, by Allen B. Tucker and Robert E. Noonan, 2007 (especially p38).

### My Ctiny design choices

* empty blocks are supported
* any block has its declarations before any statements.
* since we don't have any `void` type, any `return` must have an `<expression>`
* since ctiny is a simple language (and for security/overflow reasons), we don't support pointers. So functions with array argments must specify their size (writing `int functionA (int a[])` is not allowed by the grammar).
* since Ctiny should stay simple, it doesn't not allow any other assignment operator that `=`
* full `if-else` and `while` statements with `break` and `continue` support.
* programs without a `main` function are not allowed.
* in any program, the `main` function must be the last function defined.

### Note about my EBNF notation

Notations:

* *non-terminals* between "<>" brackets

EBNF operators:

+ **{}*** : zero or more, up to infinity
+ **()1** : list of alternatives
+ **[]?** : optional sequence, pick one or none

Any other symbol should be understood as a *terminals*.

### task

Please, help me implement `mod parser` in parser.rs using the following grammar for ctiny. I think you first need to help me convert my EBNF style grammar into PEG.





### Sat 1 Jul 2023

Started working on the Rust code. I just set up the basics of the program like CLI and pipelines.

### Fri 30 Jun 2023

Started working on the project. I created my personal version of Ctiny, following what was described as Tiny and doing some design choices.
