// Comments are highlighted in red - they're important!
// This sample demonstrates Alabaster's minimal highlighting philosophy

use std::collections::HashMap;

#[deprecated(note = "use new_function instead")]
fn old_function() -> i32 {
    42
}

/// Documentation comments show the 4 semantic colors:
/// - Strings (green)
/// - Constants (magenta)
/// - Comments, you're reading one (dark theme: yellow, light theme: red)
/// - Definitions (blue - functions, types, classes)
fn calculate_fibonacci(n: u32) -> u64 {
    // Keywords like 'if', 'let', 'return' stay default color
    // Only semantic meaning gets colored
    if n <= 1 {
        return n as u64;
    }

    let mut cache = HashMap::new();
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
    name: String,        // Field names are plain
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

    fn is_adult(&self) -> bool {
        self.age >= 18
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

    let data = vec![1, 2, 3, 4, 5];
    let sum = data
        .iter()
        .filter(|x| *x % 2 == 0)
        .map(|x| x * 2)
        .sum::<i32>();
    println!("Sum of doubled evens: {}", sum);

    let user = User::new("Alice".to_string(), 30);
    let greeting = user.greet();
    let adult = user.is_adult();
    println!("{} (adult: {})", greeting, adult);

    // Test deprecated function - should appear crossed out
    let _old = old_function();
}
