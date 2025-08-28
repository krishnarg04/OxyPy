# OxyPy Language Syntax

## Variables and Data Types

### Basic Types
```
let name: string = "John"
let age: i32 = 25
let height: f32 = 5.9
let isStudent: bool = true
let numbers: list = [1, 2, 3, 4, 5]
```

### Supported Data Types
- **Integers**: `i32`, `i64`
- **Floats**: `f32`, `f64` 
- **Boolean**: `bool`
- **String**: `string`
- **List**: `list`

## Functions

### Function Definition
```
fn greet(name: string) -> string {
    return "Hello, " + name
}

fn add(a: i32, b: i32) -> i32 {
    return a + b
}
```

### Function Calls
```
let message = greet("Alice")
let sum = add(10, 20)
```

## Classes

### Class Definition
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
```

### Class Usage
```
let person = Person { name: "Alice", age: 30 }
print(person.greet())
person.set_age(31)
```

## Control Flow

### If-Else Statements
```
if ( age >= 18 ){
    print("Adult")
} else {
    print("Minor")
}
```

### While Loops
```
let i: i32 = 0
while ( i < 5 ){
    print(i)
    i = i + 1
}
```

### For Loops with Ranges
```
for (i in ./[0,5,1]) {
    print(i)
}
```

## Built-in Functions

- `print(args...)` - Print values to stdout
- `println(args...)` - Print values to stdout with newline
- `len(string|list)` - Get length of string or list
- `current_time()` - Get current timestamp
- `to_string(value)` - Convert value to string
- `parse_int(string)` - Parse string to integer

## Operators

### Arithmetic Operations
- `+` Addition
- `-` Subtraction
- `*` Multiplication
- `/` Division
- `%` Modulo
- `!` Logical NOT

### Comparison Operations
- `==` Equal
- `!=` Not equal
- `>` Greater than
- `<` Less than
- `>=` Greater than or equal
- `<=` Less than or equal

### Logical Operations
- `&&` AND
- `||` OR

## Comments

Single-line comments using `//`:
```
// This is a comment
let x: i32 = 42  // End-of-line comment
```