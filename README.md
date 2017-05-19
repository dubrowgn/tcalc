# tcalc
A terminal calculator

## Usage
tcalc supports two modes of usage: terminal and REPL.

```bash
# Terminal
$ tcalc "2^8-1"
255
```

```bash
# REPL
$ tcalc
> 2^8-1
  255
> exit
```

## Precedence

| Operator | Description                      |
|----------|----------------------------------|
| ( )      | parens                           |
| -        | negate                           |
| ^ \*\*   | exponentiation                   |
| * / %    | multiplication, division, modulo |
| + -      | addition, subtraction            |

## REPL

| Commands | Description              |
|----------|--------------------------|
| exit     | exit the REPL            |
| quit     | alias for exit           |
