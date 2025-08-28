# OxyPy Examples

## Hello World

```
fn main() {
    print("Hello, World!")
}

main()
```

## Basic Calculator

```
fn add(a: i32, b: i32) -> i32 {
    return a + b
}

fn multiply(a: i32, b: i32) -> i32 {
    return a * b
}

let result1 = add(10, 5)
let result2 = multiply(result1, 2)
println("Result: " + to_string(result2))
```

## RPG Character System

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

## Control Flow Examples

### Conditional Logic
```
fn check_grade(score: i32) -> string {
    if score >= 90 {
        return "A"
    } else if score >= 80 {
        return "B"
    } else if score >= 70 {
        return "C"
    } else {
        return "F"
    }
}

let grade = check_grade(85)
print("Grade: " + grade)
```

### Loop Examples
```
// Countdown timer
let countdown: i32 = 5
while countdown > 0 {
    print("T-minus " + to_string(countdown))
    countdown = countdown - 1
}
print("Liftoff!")

// Sum of numbers
let sum: i32 = 0
for i in ./[1,11,1] {
    sum = sum + i
}
print("Sum of 1-10: " + to_string(sum))
```

## Working with Lists

```
let numbers: list = [1, 2, 3, 4, 5]
print("List length: " + to_string(len(numbers)))

// Process list items (conceptual - depends on implementation)
for i in ./[0,len(numbers),1] {
    // Access list elements (syntax may vary based on implementation)
    print("Number: " + to_string(numbers[i]))
}
```

## Using Built-in Functions

```
// Current time
let now = current_time()
print("Current timestamp: " + to_string(now))

// String operations
let name = "Alice"
let name_length = len(name)
print("Name '" + name + "' has " + to_string(name_length) + " characters")

// Type conversion
let number_str = "123"
let number = parse_int(number_str)
let doubled = number * 2
print("Doubled: " + to_string(doubled))
```