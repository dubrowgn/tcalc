# tcalc
A featureful calculator for when you want to do math in a terminal.

[![Build Status](https://travis-ci.org/dubrowgn/tcalc.svg?branch=master)](https://travis-ci.org/dubrowgn/tcalc)
[![crates.io](https://img.shields.io/crates/v/tcalc)](https://crates.io/crates/tcalc)

## Usage
tcalc supports two modes of usage: terminal and REPL.

```bash
# Terminal
$ tcalc "2**8 - 1" "ans >> 1"
255
127
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

## Installation

1. Install [Rust](https://www.rust-lang.org/en-US/install.html)
2. Run `cargo install tcalc` from your terminal

## Syntax

| Operator | Description                                                  |
|----------|--------------------------------------------------------------|
| ( )                                  | parens                           |
| - !                                  | negate, bitwise NOT              |
| \*\*                                 | exponentiation                   |
| * / %                                | multiplication, division, modulo |
| + -                                  | addition, subtraction            |
| ++ --                                | increment, decrement (suffix)    |
| << >>                                | left shift, right shift          |
| &                                    | bitwise AND                      |
| ^                                    | bitwise XOR                      |
| \|                                   | bitwise OR                       |
| =                                    | variable assignment              |
| += -= *= /= %= **= &= \|= ^= <<= >>= | compound assignment              |

| Numeric Format | Description                |
|----------------|----------------------------|
| [0-9]          | decimal literal            |
| [0-9].[0-9]    | fractional decimal literal |
| 0b[0-1]        | binary literal             |
| 0o[0-7]        | octal literal              |
| 0d[0-9]        | decimal literal            |
| 0d[0-9].[0-9]  | fractional decimal literal |
| 0x[0-9a-f]     | hexadecimal literal        |

| Variable | Description                   |
|----------|-------------------------------|
| e        | Euler's number (e)            |
| phi      | Golden ratio (φ)              |
| pi       | Archimedes' constant (π)      |
| ans      | Result of previous expression |

## REPL

| Command | Description              |
|---------|--------------------------|
| exit    | exit the REPL            |
| quit    | alias for exit           |
