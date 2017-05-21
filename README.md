# tcalc
A terminal calculator

## Usage
tcalc supports two modes of usage: terminal and REPL.

```bash
# Terminal
$ tcalc "2**8 - 1"
255
```

```bash
# REPL
$ tcalc
> 2**8 - 1
  255
> ans >> 1
  127
> exit
```

## Precedence

| Operator | Description                      |
|----------|----------------------------------|
| ( )      | parens                           |
| - !      | negate, bitwise NOT              |
| \*\*     | exponentiation                   |
| * / %    | multiplication, division, modulo |
| + -      | addition, subtraction            |
| << >>    | left shift, right shift          |
| &        | bitwise AND                      |
| ^        | bitwise XOR                      |
| \|       | bitwise OR                       |

## REPL

| Commands | Description              |
|----------|--------------------------|
| exit     | exit the REPL            |
| quit     | alias for exit           |

## Built-in Variables

| Variable | Description                   |
|----------|-------------------------------|
| e        | Euler's number (e)            |
| pi       | Archimedes' constant (π)      |
| ans      | Result of previous expression |
