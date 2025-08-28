# OxyPy Architecture

## Overview

OxyPy is built with a modular architecture that separates lexical analysis, parsing, and execution into distinct components.

## Project Structure

```
src/
├── main.rs          # Entry point and file processing
├── tokenizer.rs     # Lexical analysis and token definitions
├── AstTree.rs       # Abstract Syntax Tree and parser
├── runtime.rs       # Runtime execution engine
├── Environment.rs   # Variable and scope management
├── Functions.rs     # Built-in function implementations
└── Repl.rs          # Interactive REPL interface
```

## Component Details

### 1. Entry Point (`main.rs`)

The main module handles:
- Command-line argument processing
- File reading and execution
- REPL mode initialization

**Usage Modes:**
- **File Execution**: `cargo run filename.lang`
- **Interactive REPL**: `cargo run` (no arguments)

### 2. Lexical Analysis (`tokenizer.rs`)

The tokenizer breaks down source code into tokens:

**Token Types:**
- Keywords (`let`, `fn`, `class`, `if`, `while`, etc.)
- Identifiers and literals (strings, numbers, booleans)
- Operators (`+`, `-`, `*`, `/`, `%`, `==`, `!=`, etc.)
- Delimiters (`{`, `}`, `(`, `)`, `[`, `]`, etc.)

### 3. Parsing (`AstTree.rs`)

The parser constructs an Abstract Syntax Tree using recursive descent parsing:

**AST Node Types:**
- Variable declarations and assignments
- Function definitions with parameters
- Class declarations with fields and methods
- Expression trees with operator precedence
- Control flow structures (if/else, loops)

### 4. Runtime Execution (`runtime.rs`)

The runtime engine executes the AST:

**Capabilities:**
- Variable scoping through the Environment system
- Function calls (user-defined and built-in)
- Method calls on class instances with `self` parameter
- Control flow and expression evaluation
- Memory management for variables and class instances

### 5. Environment Management (`Environment.rs`)

The Environment system provides:

**Features:**
- Variable storage and retrieval
- Scope management for functions and classes
- Class definition and instance management
- Hierarchical scope resolution

### 6. Built-in Functions (`Functions.rs`)

Implements essential functions:

**Available Functions:**
- I/O operations (`print`, `println`)
- Data operations (`len`, `to_string`, `parse_int`)
- System operations (`current_time`)

### 7. REPL Interface (`Repl.rs`)

Interactive Read-Eval-Print Loop:

**Features:**
- Multi-line input support
- Brace matching for code blocks
- Real-time execution
- Exit commands (`exit`, `quit`)

## Execution Flow

1. **Input Processing**: Source code (file or REPL) is read
2. **Tokenization**: Code is broken into tokens
3. **Parsing**: Tokens are organized into an AST
4. **Execution**: AST nodes are executed with environment management
5. **Output**: Results are displayed or returned

## Error Handling

The interpreter handles various error types:
- Lexical errors (invalid tokens)
- Syntax errors (malformed code)
- Runtime errors (undefined variables, type mismatches)
- Semantic errors (invalid operations)

## Memory Management

- Variables are stored in hierarchical environments
- Automatic cleanup when scopes exit
- Class instances managed through the runtime system