# WORKLOG

* [ ] Add symbol table to be able to recognize if an identifier is defined or not ! This sould ensure `cargo run -- -p syntax-parsing -f res/invalid/invalid_program_c.ctiny` detects the unknown identifier `a` at line 6.
* [ ] Add type system with overflow checking
* [ ] Correct the .txt grammar with modifications done inside .pest PEG file.

### Wed 5 Jul 2023

Removing block from statements, which simplifies things a lot.

### Mon 3 Jul 2023

Added full syntax-parsing using `pest`.

I added comments and made some corrections to the grammar. I had made some mistake in function parameters and declarations that lead to ambiguities.

The `syntax_parsing` pipeline is able to perform full tokenization. Symbol table is missing though, so undefine identifier errors are not handled yet.

* [ ] Add symbol table to be able to recognize if an identifier is defined or not ! This sould ensure `cargo run -- -p syntax-parsing -f res/invalid/invalid_program_c.ctiny` detects the unknown identifier `a` at line 6.
* [ ] Correct the .txt grammar with modifications done inside .pest PEG file.

### Sat 1 Jul 2023

Started working on the Rust code. I just set up the basics of the program like CLI and pipelines.

### Fri 30 Jun 2023

Started working on the project. I created my personal version of Ctiny, following what was described as Tiny and doing some design choices.
