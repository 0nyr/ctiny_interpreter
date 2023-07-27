# Ctiny grammar: Design choices and EBNF specification

## Design choices

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

* [PEST](https://pest.rs/) PEG parser. Note that I am a *contributor* of this Open Source project.

### My Ctiny design choices

* Empty function blocks are NOT allowed. This is due to the fact that there is no `void` type. Hence each function MUST have a non-void return.
* Any block has its declarations before any statements.
* Since we don't have any `void` type, any `return` must have an `<expression>` and any function must have a unique final return statement.
* Since ctiny is a simple language (and for security/overflow reasons), we don't support pointers. So functions cannot return arrays or take arrays as argument.
* The only available as assignment operator is `=`.
* Keywords `break`, `return` or `continue` not allowed inside While loops.
* Programs without a `main` function are not allowed.
* The `main` function must be the last function defined.
* Empty statements like `;;;` are not allowed.
* Array sizes must be positive.
* Chars for identifiers are only ASCII letters and digits. No special characters like '\0' handled by the grammar parser. But since our chars internal representation are 1 byte long, they can contain special characters after a cast. To be closer to C chars, I have choosen not to consider the conversion results as overflow (as long at the char u8 is positive and doesn't overflow its single byte).
* In order to detect infinite loop, and considering that the language is simple, with no definitions or return possible inside a While loop, there is a maximum for the number of iteration of a given while. This maximum can be set manually in the `.env`.
* At the end of a function, all variables (including values inside an array) must have been set a value. This is a direct implementation of the following statement from the subject: `Each variable must be assigned a value after its declaration.`

### Note about my EBNF notation

Notations:

* *non-terminals* between "<>" brackets

EBNF operators:

+ **{}*** : zero or more, up to infinity
+ **()1** : list of alternatives
+ **[]?** : optional sequence, pick one or none

Any other symbol should be understood as a *terminals*.


# My Ctiny EBNF grammar

> Note that my design choices and more information about my Ctiny grammar are described before.

### file layout

`<translation-unit>` -> { `<function-definition>` }* `<entry-point-function-definition>`

`<entry-point-function-definition>` -> `<int-type>` main () `<function-block>`

`<function-definition>` -> `<type-specifier>` `<identifier>` ( [ `<parameter-list>` ]? ) `<function-block>`

`<parameter-list>` -> `<parameter>` { , `<parameter>` }*

`<parameter>` -> `<type-specifier>` `<identifier>`

### types

`<type-specifier>` -> `<int-type>` | bool | float | char

`<int-type>` -> int

### identifiers

`<identifier>` -> `<letter>` { `<letter>` | `<digit>` }*

`<letter>` -> a | b | c | d | e | f | g | h | i | j | k | l | m | n | o | p | q | r | s | t | u | v | w | x | y | z | A | B | C | D | E | F | G | H | I | J | K | L | M | N | O | P | Q | R | S | T | U | V | W | X | Y | Z

`<digit>` -> 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9

### literals

`<literal>` -> `<integer>` | `<float>` | `<char>` | `<boolean>`

`<integer>` -> `<digit>` { `<digit>` }*

`<float>` -> `<integer>` . `<integer>`

`<char>` -> ' `<letter>` ' | ' `<digit>` '

`<boolean>` -> true | false

### function block

`<function-block>` -> { { `<declaration>` }* { `<statement>` }* `<return>` }

`<declaration>` -> `<type-specifier>` `<identifier>` [ [ `<integer>` ] ]? { , `<identifier>` [ [ `<integer>` ] ]? }* ;

`<statement>` -> `<assignment-statement>` | `<if-statement>` | `<while-statement>` | ;

`<return>` -> return `<expression>` ;

### sub-statements

`<assignment-statement>` -> `<identifier>` [ [ `<expression>` ] ]? = `<expression>` ;

`<if-statement>` -> if ( `<expression>` ) `<statement>` [ else `<statement>` ]?

`<while-statement>` -> while ( `<expression>` ) `<statement>`

### expressions

`<expression>` -> `<conjunction>` { || `<conjunction>` }*

`<conjunction>` -> `<equality>` { && `<equality>` }*

`<equality>` -> `<relation>` { `<equality-operator>` `<relation>` }*

`<equality-operator>` -> == | !=

`<relation>` -> `<addition>` [ `<relation-operator>` `<addition>` ]*

`<relation-operator>` -> < | > | <= | >=

`<addition>` -> `<term>` { `<addition-operator>` `<term>` }*

`<addition-operator>` -> + | -

`<term>` -> `<factor>` { `<multiplication-operator>` `<factor>` }*

`<multiplication-operator>` -> * | / | %

`<factor>` -> [ `<unary-operator>` ]? `<primary>`

`<unary-operator>` -> - | !

`<primary>` -> `<identifier>` [ [ `<expression>` ] ]? | `<literal>` | ( `<expression>` ) | `<function-call>` | `<type-cast>`

`<function-call>` -> `<identifier>` ( [ `<expression>` { , `<expression>` }* ]? )

`<type-cast>` -> ( `<type-specifier>` ) `<primary>`
