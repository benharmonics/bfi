# BFI (Brainfuck Interpreter)

Interprets [Brainfuck](http://www.muppetlabs.com/~breadbox/bf/) scripts

Thanks to [www.brainfuck.org](www.brainfuck.org) for many of the tests I've blatantly stolen.

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
