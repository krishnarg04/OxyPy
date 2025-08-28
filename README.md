# OxyPy

A Rust-based interpreter for a custom programming language that supports object-oriented programming, control flow, arithmetic operations, and built-in functions.

## Quick Start

### Installation
```bash
git clone https://github.com/krishnarg04/OxyPy.git
cd OxyPy
cargo build --release
```

### Usage

**Interactive REPL Mode:**
```bash
./OxyPy
>> let x: i32 = 42
>> print(x)
42
>> exit
```

**Execute Files:**
```bash
./OxyPy run examples/hello.lang
```

## Features

- **Object-Oriented Programming**: Classes with fields and methods
- **Data Types**: Integers, floats, booleans, strings, and lists
- **Control Flow**: if/else statements, while loops, and for loops
- **Functions**: User-defined and built-in functions
- **Interactive REPL**: Write and test code interactively

## Documentation

- **[Language Syntax](docs/syntax.md)** - Complete language reference
- **[Examples](docs/examples.md)** - Sample programs and tutorials
- **[Architecture](docs/architecture.md)** - How the interpreter works
- **[Contributing](docs/contributing.md)** - Development guidelines

## License

This project is available under the [Apache-2.0 license](LICENSE).