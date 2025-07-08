# Rusty calculator
The calculator supports:
* Arithmetic operators: +, -, *, /
* Negative numbers using the unary operator ‘-’
* Parentheses: ( and ) for grouping
* All whitespace to be treated as not significant
* Logging with configurable verbosity

## Usage
```
cargo run -- --verbosity=debug
```
or
```
cargo run -- -v=4
```

Then enter the expression in the terminal:
```
> -1 + 5 * (2 + 1) - 3
-1 + 5 * (2 + 1) - 3 = 11
```

Exit with: `Ctrl+C`

### Notes
* Supports only integer (i64) numbers
* Handles arithmetic overflow safely
* Returns friendly errors on invalid input or division by zero.

## Design choices

### Parsing structure
A recursive descent parser is used and it is based on the following grammar:
```
expression ::= term (("+" | "-") term)*
term       ::= factor (("*" | "/") factor)*
factor     ::= INT | "-" factor | "(" expression ")"
```

### `pest`
The `pest` crate (https://docs.rs/pest/latest/pest/) was also considered.
No specific reason that has not been used. Could be used in a future version.

### Translation
A simpler solution was considered: first translating the input into a complete list of tokens
and then parsing those tokens into an expression tree in a second pass.
This approach was rejected to avoid iterating through the input twice.

### `TokenTranslator` and the `Iterator` trait
Implementing the `Iterator` trait for `TokenTranslator` was also considered as a way to provide a more idiomatic interface.
However, the current design avoids this in favor of more flexible error handling.

### Early exits for wrong inputs
Two potentials early conditions are considered:
1) Invalid characters (e.g. unsupported symbols)
2) Unmatched parenthesis

Although detecting these issues early would allow for immediate failure, early exits were not implemented to maintain a single-pass design.

## Known limitations
### Overflow for i64::MIN
This calculator does not support evaluating the expression `-9223372036854775808` (`i64::MIN`).

This is due to a limitation of 2's complement integer representation in Rust:

The absolute value of `i64::MIN` (`9223372036854775808`) exceeds the maximum representable i64 value (`i64::MAX` = `9223372036854775807`).

As a result, attempting to parse or evaluate `-9223372036854775808` will lead to an overflow error.

### Unmatched parenthesis
```
1+((2*3)+2
```
Wrong inputs like the above will lead to unexpected behavior instead of an error like: "Unmatched parenethesis"

A possible solution would be to count the number of left and right parenethesis and return a necessary error when they don't match.