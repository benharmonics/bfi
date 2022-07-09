# BFI (Brainfuck Interpreter)

Interprets [Brainfuck](http://www.muppetlabs.com/~breadbox/bf/) scripts

## Usage:

```bash
bfi <FILE>
```

or

```bash
bfi <TEXT>
```

where `<TEXT>` is interpreted as Brainfuck, i.e.

```bash
bfi '++++++++++[>++++++++++>+++++++++++>+<<<-]>--.>++++.<-.++++++++.+++++.--------.>+++.<---.++++++++.>>.'
```

## Installation

*You'll need [Cargo](https://github.com/rust-lang/cargo/), which comes with Rust.*

Clone the repository, then

```bash
cargo install --path path/to/bfi
```
