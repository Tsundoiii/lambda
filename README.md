# Psil

A stack-based virtual machine-based programming language, like LISP but reversed. Heavy inspiration for interpreter design comes from [Crafting Intepreters](https://craftinginterpreters.com), although the language design itself is very different.

## Usage
The `psil` command-line tool can be invoked with no argument to enter REPL mode, or with one argument to run a file.

In Psil, arguments to functions or operands to operators are placed before their respective functions or operators.
```
$> 3 4 +
7
```

Variables can be declared using `bind`, which can be thought of as a function which takes in two arguments: a value and an identifier for that value.
```
$> 3 x bind; x 4 +
7
```

Values can also be printed (more useful in files than in REPL):
```
3 4 + print
```
```
7
```

## Known Issues
- Generally very unstable and error-prone
- No support for strings or functions yet lmao
- REPL does not yet remember variables from previous lines entered in the same sessions. Any variable usage must occur on the same line (ex. `$> 4 x bind; x x *`). This is not a problem for files.