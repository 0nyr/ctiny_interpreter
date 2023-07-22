# Ctiny grammar

**Ctiny** is my version of the imperative language refered to as `Tiny` in the project description.

### Ctiny support

* [X] declarations
* [X] ~~definitions~~ -> assignments

> There is a "mistake" in the subject. In c, we have:
>
> declarations: `extern int x;`
>
> definitions: `int x=42;`
>
> assignments: `x=0;`
>
> But in Ctiny, we can only have declarations, then assignments. The first assignment is responsible implicitly to define the variable in memory.

* [X] arithmetic operations
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

Dependencies and libraries

* [PEST](https://pest.rs/) PEG parser.

### My Ctiny design choices

* empty function blocks are supported
* any block has its declarations before any statements.
* since we don't have any `void` type, any `return` must have an `<expression>` and any function must have a unique final return statement.
* since ctiny is a simple language (and for security/overflow reasons), we don't support pointers. So functions with array argments must specify their size (writing `int functionA (int a[])` is not allowed by the grammar).
* since Ctiny should stay simple, it doesn't not allow any other assignment operator that `=`
* since Ctiny should be simple, not `break`, `return` or `continue` keywords inside While loops.
* programs without a `main` function are not allowed.
* in any program, the `main` function must be the last function defined.
* empty statements like `;;;` are not allowed.
* array sizes must be positive
* chars for identifiers are only ASCII letters and digits. Not special characters like '\0' handled by the grammar parser. But since our chars internal representation are 1 byte long, they can contain special characters after a cast for instance. To be closer to C chars, I have choosen not to consider the conversion results as overflow (as long at the char u8 is positive and doesn't overflow).

### Note about my EBNF notation

Notations:

* *non-terminals* between "<>" brackets

EBNF operators:

+ **{}*** : zero or more, up to infinity
+ **()1** : list of alternatives
+ **[]?** : optional sequence, pick one or none

Any other symbol should be understood as a *terminals*.
