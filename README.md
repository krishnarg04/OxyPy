# OxyPy

A Rust-based interpreter for a custom programming language that supports object-oriented programming, control flow, arithmetic operations, and built-in functions.

## Features

- **Object-Oriented Programming**: Classes with fields and methods, including `self` parameter support
- **Data Types**: Integers (i32, i64), floats (f32, f64), booleans, strings, and lists
- **Control Flow**: if/else statements, while loops, and for loops with range syntax
- **Functions**: User-defined functions with parameters and return values
- **Built-in Functions**: `print`, `println`, `len`, `current_time`, `to_string`, `parse_int`
- **Arithmetic Operations**: Addition, subtraction, multiplication, division, modulo, and logical NOT
- **Comparison Operations**: Equal, not equal, greater than, less than, greater/less than or equal
- **Logical Operations**: AND, OR operations
- **Comment Support**: Single-line comments using `//`

## Language Syntax

### Variables and Data Types
```
let name: string = "John"
let age: i32 = 25
let height: f32 = 5.9
let isStudent: bool = true
let numbers: list = [1, 2, 3, 4, 5]
```

### Functions
```
fn greet(name: string) -> string {
    return "Hello, " + name
}

fn add(a: i32, b: i32) -> i32 {
    return a + b
}
```

### Classes
```
class Person {
    public {
        name: string
        age: i32
    }
    
    public {
        fn greet(self) -> string {
            return "Hello, my name is " + self.name
        }
        
        fn set_age(self, new_age: i32) {
            self.age = new_age
        }
    }
}

let person = Person { name: "Alice", age: 30 }
print(person.greet())
```

### Control Flow
```
// If-else statements
if age >= 18 {
    print("Adult")
} else {
    print("Minor")
}

// While loops
let i: i32 = 0
while i < 5 {
    print(i)
    i = i + 1
}

// For loops with ranges
for i in 0..5 {
    print(i)
}
```

### Built-in Functions
- `print(args...)` - Print values to stdout
- `println(args...)` - Print values to stdout with newline
- `len(string|list)` - Get length of string or list
- `current_time()` - Get current timestamp
- `to_string(value)` - Convert value to string
- `parse_int(string)` - Parse string to integer

## Project Structure

```
src/
├── main.rs          # Entry point and file processing
├── tokenizer.rs     # Lexical analysis and token definitions
├── AstTree.rs       # Abstract Syntax Tree and parser
├── runtime.rs       # Runtime execution engine
├── Environment.rs   # Variable and scope management
└── Functions.rs     # Built-in function implementations
```

## How It Works

### 1. Lexical Analysis (Tokenizer)
The tokenizer (`tokenizer.rs`) breaks down source code into tokens, handling:
- Keywords (`let`, `fn`, `class`, `if`, `while`, etc.)
- Identifiers and literals (strings, numbers, booleans)
- Operators (`+`, `-`, `*`, `/`, `%`, `==`, `!=`, etc.)
- Delimiters (`{`, `}`, `(`, `)`, `[`, `]`, etc.)

### 2. Parsing (AST Construction)
The parser (`AstTree.rs`) constructs an Abstract Syntax Tree from tokens:
- Parses variable declarations and assignments
- Handles function definitions with parameters
- Processes class declarations with fields and methods
- Builds expression trees with proper operator precedence
- Manages control flow structures (if/else, loops)

### 3. Runtime Execution
The runtime engine (`runtime.rs`) executes the AST:
- Manages variable scoping through the Environment system
- Handles function calls (both user-defined and built-in)
- Executes method calls on class instances with `self` parameter
- Processes control flow and expression evaluation
- Manages memory for variables and class instances

### 4. Environment Management
The Environment system (`Environment.rs`) provides:
- Variable storage and retrieval
- Scope management for functions and classes
- Class definition and instance management
- Hierarchical scope resolution

## Installation and Usage

### Prerequisites
- Rust (2024 edition or later)
- Cargo package manager

### Building the Project
```bash
git clone <your-repo-url>
cd OxyPy
cargo build --release
```

### Running Programs
```bash
# Run a source file
cargo run <filename>

# Example
cargo run examples/hello.lang
```

### Example Program
Create a file called `example.lang`:
```
// Simple RPG character system
class Character {
    public {
        name: string
        health: i32
        level: i32
    }
    
    public {
        fn greet(self) -> string {
            return "Hello, I am " + self.name
        }
        
        fn level_up(self) {
            self.level = self.level + 1
            self.health = self.health + 10
            print("Level up! Now level " + to_string(self.level))
        }
        
        fn take_damage(self, damage: i32) {
            self.health = self.health - damage
            if self.health <= 0 {
                print(self.name + " has been defeated!")
            }
        }
    }
}

let hero = Character { name: "Warrior", health: 100, level: 1 }
print(hero.greet())

hero.level_up()
hero.take_damage(30)
print("Health remaining: " + to_string(hero.health))
```

Run it with:
```bash
cargo run example.lang
```

## Development

The interpreter is built with a modular architecture:
- **Tokenizer**: Converts source code into tokens
- **Parser**: Builds AST from tokens using recursive descent parsing
- **Runtime**: Executes AST nodes with environment management
- **Built-ins**: Provides essential functions for I/O and data manipulation

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Submit a pull request

## License

This project is open source and available under the [Apache-2.0 license ](https://github.com/krishnarg04/OxyPy?tab=Apache-2.0-1-ov-file#Apache-2.0-1-ov-file).

## Future Enhancements

- [ ] Module system and imports
- [ ] Error handling with try/catch
- [ ] More built-in data structures (maps, sets)
- [ ] File I/O operations
- [ ] Standard library expansion
- [ ] Performance optimizations
- [ ] Better error messages and debugging support