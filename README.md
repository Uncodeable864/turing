![Turing Complete Status](https://img.shields.io/badge/turing%20complete-mostly-yellow?style=for-the-badge)

# Turing - A Simple Turing Machine

> It's a [turing machine](https://en.wikipedia.org/wiki/Turing_machine)!

## The basic concept

Each program has a "memory" declared by the command `INIT [size]` where `x` is the size of the memory (default is 3,000). It is a binary vallue in memory. You also have the program memory (`INIT PRGM [size]`). `turing` auto-creates a program with a length of 10,000, create a new one will override the previous one. After the exucution of intruction, `turing` will goto the next value in the `PRGM` line. `GOTO [x] (line)` can be used to move to a certian value. The `line` value will let you move to any given point in any line (for the default program line `PGRM` and for the default memory line: `MEM`). `LEFT (line)` and `RIGHT (line)` will move left and write respecivly in the `line` (or, if no line is specified, the `MEM` line.) `NEXT` does the same as `RIGHT`, and `PREV` does the same as `LEFT`. `SWITCH (x)` toggle the value at `x`, or the current location in the `MEM` line. `READ (x)` is like `SWITCH`, but will just read, and output the result.`WRITE [v] (x)` is `SWITCH` but will instead set the value to `v`.

### Logic

`IF [a] [f] (b) [loc] (antiLoc)` - Self-explainary, if performing `f, a` are the initial values, and `b` is an optional one that some operators may take in (like `EQUALS`). `loc` is just the line to goto if the value is true. If `antiLoc` is passed, it will go there if false, otherwisw it will just continue reading. By putting `a` or `b` in square brackets, you can "next" booloean operators. Curly brackets mean to get the value of that point in memory.

Most of the boolean operators work exactly how you would expect.

`[a] NOT`
`[a] EQUALS [b]`
`[a] AND [b]`
`[a] OR [b]`

### Writing to the standard output

At some point, you want your program to give you something usefull.
Print is easy with these 3 commands:

- `PRINT [x] (line)` - Prints the value at the line `line` at `x`, the default line is `MEM`. Output will be `0` or `1`.
- `BIN PRINT [i] [o] (line)` - Similar to `PRINT`, but does all the values from `i` to `o` (inclusive).
- `NUM PRINT [i] [o] (line) (singed)` - Like `BIN PRINT` but instead prints it as a numerical value (so `1100` would be printed as `12`), but is `signed` is `1` (it is `0` by default), the first digit is used to determine if the number is negitave (first value being `1`). If `signed` is `1`, then `1100` would be `-4`.
- `UTF PRINT [i] [o] (line)` - Like `BIN PRINT`, but with the UTF-8 standard.

## Comments

Comments are done by prepending a line with `#`, `//`, or `;--`. **_A COMMENT MAY NOT BE AT THE END OF A LINE, IT MUST BE ON ITS OWN LINE._**

## File type

`*.trng`, short for `turing`
