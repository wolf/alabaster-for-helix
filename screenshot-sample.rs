// Comments are highlighted in red - they're important!
// This sample demonstrates Alabaster's minimal highlighting philosophy

use std::collections::HashMap;

/// Documentation comments show the 4 semantic colors:
/// - Strings (green)
/// - Constants (magenta)
/// - Comments (red - you're reading one!)
/// - Definitions (blue - functions, types, classes)
fn calculate_fibonacci(n: u32) -> u64 {
    // Keywords like 'if', 'let', 'return' stay default color
    // Only semantic meaning gets colored
    if n <= 1 {
        return n as u64;
    }

    let mut cache: HashMap<u32, u64> = HashMap::new();
    cache.insert(0, 0);
    cache.insert(1, 1);

    // Operators and punctuation are dimmed
    for i in 2..=n {
        let prev1 = cache.get(&(i - 1)).unwrap();
        let prev2 = cache.get(&(i - 2)).unwrap();
        cache.insert(i, prev1 + prev2);
    }

    *cache.get(&n).unwrap()
}

struct User {
    name: String,        // Strings are green
    age: u32,            // Types are blue
    active: bool,        // Builtin types are blue
}

impl User {
    fn new(name: String, age: u32) -> Self {
        const MAX_AGE: u32 = 150;  // Constants are magenta

        if age > MAX_AGE {
            panic!("Invalid age: {}", age);
        }

        User {
            name,
            age,
            active: true,  // Boolean constant is magenta
        }
    }

    fn greet(&self) -> String {
        format!("Hello, {}! You are {} years old.", self.name, self.age)
    }
}

fn main() {
    let numbers = vec![0, 1, 2, 3, 5, 8, 13];
    let pi = 3.14159;  // Numbers are magenta

    println!("Fibonacci sequence:");
    for num in numbers {
        let result = calculate_fibonacci(num);
        println!("F({}) = {}", num, result);
    }

    let user = User::new("Alice".to_string(), 30);
    println!("{}", user.greet());
}
