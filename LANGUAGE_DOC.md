## Fundemental commands

- `INIT [x]` - Create the memory space with a size of `x`. Clears memory.
- `GOTO [x]` - Sets the read position of the program's memory to location `x`.
- `JUMP [x]` - Jump's to the execution point `x`.
- `LOC [x]` - Set the line the command to the execution point number `x`. DO NOT use this is combination with other commands, it should be standalone. In addition, execution points must be declared first (so you can't jump to `LOC` points later in the program). **WARNING: You can only have 1,000 execution points in one program (but you can change where**
- Move commands: `LEFT` and `PREV` go backwards, while `RIGHT` and `PREV` go forwards (to be added)
- `WRITE [x] (y)` - Writes `x` to the current position. If `y` is specified, it will use `y` rather than the current position.
- `LOGIWRITE ([x]) (y)` - Similar to `WRITE`, but evaluates the expression in `x` first, and then sets the value.

- `SWITCH [x] (y)` - Similar to `WRITE` but switches the value from it's current position. (Unimplemented)
- `{[x]}` - Replaces itself with the value at position x`. Example `{2} EQUALS {1}`. (Unimplemented)
- `END` - Place at end of file, will end your program the moment it is seen

### Logic (uninplemented)

`IF [x] [loc] (antiLoc)` - he current location will jump to the execution point `loc` if the value of `x` is `1`. If `x` is `0`, it will continue reading OR (if `antiLoc` is passed), jump to the execution point `antiLoc`.

- `WBOOL ([a] (c) [b]) [loc]` - Runs the opperation `b` on `a` (and if not `NOT` `c`). Writes the value of that to the location `loc`. For example:

```
WBOOL (12 OR 14) 5
```

When it comes to opperators, it is worth noting that `a` and `b` are both memory addresses, NOT binary values.

- `[a] NOT` - The opposite of `a`, so `0` would give `1`, and `1` would give `0`.
- `[a] EQUALS [b]` - Gives a `1` if `a` and `b` are the same value (either `0` or `1`), and a `0` otherwise.
- `[a] AND [b]` - Gives a `1` if `a` and `b` are `1`, and a `0` otherwise.
- `[a] OR [b]` - Gives a `1` if `a` or `b` is one, and a `0` otherwise.

To use memory points use the `{[x]}` in-line command.

#### Logic example

`IF {1} AND {2} 1 2`
Explaination: Get the value at `1` and `2`, run `AND` on them. If the result is `1`, jump to execution point `1`, if the result is `0`, jump to execution point `2`.

#### Nesting logic

To nest login, simply set the value of a memory point, and use it in the `IF`.
Example:

```
// Setting memory point
LOGIWRITE 1 AND 0 2
// If statment
IF {2} AND {1} 1
```

### Writing to the standard output

- `BIT PRINT [x] (line)` - Prints the value at the line `line` at `x`, the default line is `MEM`. Output will be `0` or `1`.
- `BIN PRINT [i] [o] (line)` - Similar to `BIT PRINT`, but does all the values from `i` to `o` (inclusive).
- `NUM PRINT [i] [o] (line) (singed)` - Like `BIN PRINT` but instead prints it as a numerical value (so `1100` would be printed as `12`), but is `signed` is `1` (it is `0` by default), the first digit is used to determine if the number is negitave (first value being `1`). If `signed` is `1`, then `1100` would be `-4`.
- `UTF PRINT [i] [o] (line)` - Like `BIN PRINT`, but with the UTF-8 standard (Unimplemented).

## Comments

Comments are done by prepending a line with `#`, `//`, or `;--`. **_A COMMENT MAY NOT BE AT THE END OF A LINE, IT MUST BE ON ITS OWN LINE._**
