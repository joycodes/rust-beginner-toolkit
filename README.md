# 🦀 Getting Started with Rust: A Beginner's Toolkit

## 1. Title & Objective

**Technology**: Rust Programming Language  
**Why Rust?**: Rust is a systems programming language that focuses on speed, memory safety, and parallelism. It's increasingly popular for web backends, blockchain development, game engines, and system utilities.  
**End Goal**: Create a simple command-line calculator that demonstrates Rust's syntax, error handling, and cargo build system.

---

## 2. Quick Summary of the Technology

**What is Rust?**
Rust is a multi-paradigm systems programming language developed by Mozilla. It emphasizes memory safety without garbage collection, making it ideal for performance-critical applications.

**Where is it used?**
- **Web backends** (Dropbox, Discord)
- **Operating systems** (parts of Linux kernel)
- **Blockchain** (Solana, Polkadot)
- **Game engines** (Bevy)
- **CLI tools** (ripgrep, bat, fd)

**Real-world example**: Discord rewrote their "Go to" feature in Rust, reducing memory usage and improving performance significantly.

---

## 3. System Requirements

**Operating System**: Linux, macOS, or Windows  
**Tools Required**:
- Terminal/Command Prompt
- Text editor (VS Code recommended with rust-analyzer extension)
- Internet connection for package downloads

**No additional packages needed** - Rust installer includes everything!

---

## 4. Installation & Setup Instructions

### Step 1: Install Rust
```bash
# For Unix-like systems (Linux/macOS)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# For Windows, download from: https://rustup.rs/
```

### Step 2: Configure your shell
```bash
source ~/.cargo/env
```

### Step 3: Verify installation
```bash
rustc --version
cargo --version
```

Expected output:
```
rustc 1.70.0 (90c541806 2023-05-31)
cargo 1.70.0 (ec8a8a0ca 2023-05-25)
```

### Step 4: Create your first project
```bash
cargo new rust_calculator
cd rust_calculator
```

---

## 5. Minimal Working Example

### What the example does:
A simple command-line calculator that can perform basic arithmetic operations (addition, subtraction, multiplication, division) and demonstrates Rust's ownership system, error handling, and pattern matching.

### Code Structure:
```
rust_calculator/
├── Cargo.toml
└── src/
    └── main.rs
```

### Cargo.toml
```toml
[package]
name = "rust_calculator"
version = "0.1.0"
edition = "2021"

[dependencies]
```

### src/main.rs
```rust
use std::io;

fn main() {
    println!("🦀 Welcome to Rust Calculator!");
    println!("Enter calculations like: 5 + 3 or type 'quit' to exit");
    
    loop {
        println!("\nEnter your calculation:");
        
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        let input = input.trim();
        
        // Check if user wants to quit
        if input.to_lowercase() == "quit" {
            println!("Goodbye! 👋");
            break;
        }
        
        // Parse and calculate
        match parse_and_calculate(input) {
            Ok(result) => println!("Result: {}", result),
            Err(error) => println!("Error: {}", error),
        }
    }
}

fn parse_and_calculate(input: &str) -> Result<f64, String> {
    // Split input by spaces
    let parts: Vec<&str> = input.split_whitespace().collect();
    
    // Check if we have exactly 3 parts: number operator number
    if parts.len() != 3 {
        return Err("Format should be: number operator number".to_string());
    }
    
    // Parse first number
    let num1: f64 = parts[0]
        .parse()
        .map_err(|_| format!("'{}' is not a valid number", parts[0]))?;
    
    // Get operator
    let operator = parts[1];
    
    // Parse second number  
    let num2: f64 = parts[2]
        .parse()
        .map_err(|_| format!("'{}' is not a valid number", parts[2]))?;
    
    // Perform calculation based on operator
    match operator {
        "+" => Ok(num1 + num2),
        "-" => Ok(num1 - num2),
        "*" => Ok(num1 * num2),
        "/" => {
            if num2 == 0.0 {
                Err("Cannot divide by zero!".to_string())
            } else {
                Ok(num1 / num2)
            }
        }
        _ => Err(format!("Unsupported operator: {}", operator)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(parse_and_calculate("5 + 3").unwrap(), 8.0);
    }

    #[test]
    fn test_division_by_zero() {
        assert!(parse_and_calculate("10 / 0").is_err());
    }
}
```

### Running the Example:
```bash
# Compile and run
cargo run

# Run tests
cargo test

# Build optimized version
cargo build --release
```

### Expected Output:
```
🦀 Welcome to Rust Calculator!
Enter calculations like: 5 + 3 or type 'quit' to exit

Enter your calculation:
10 + 5
Result: 15

Enter your calculation:
20 / 4
Result: 5

Enter your calculation:
quit
Goodbye! 👋
```

---

## 6. AI Prompt Journal

### Prompt 1: Basic Setup
**Prompt**: "How do I install Rust programming language and create my first project?"

**AI Response Summary**: Provided rustup installation command and cargo new project creation steps.

**Evaluation**: Very helpful - got me up and running quickly with the standard toolchain.

### Prompt 2: Project Structure
**Prompt**: "Create a simple Rust program that demonstrates ownership, error handling, and basic syntax for beginners"

**AI Response Summary**: Suggested a calculator program with Result types, pattern matching, and proper error handling.

**Evaluation**: Excellent suggestion - covers multiple Rust concepts in one practical example.

### Prompt 3: Error Handling
**Prompt**: "How does error handling work in Rust with Result and Option types?"

**AI Response Summary**: Explained Result<T, E> for recoverable errors and Option<T> for nullable values, with examples of match and ? operator.

**Evaluation**: Clear explanation that helped implement robust error handling in the calculator.

### Prompt 4: Testing
**Prompt**: "How do I write and run tests in Rust?"

**AI Response Summary**: Showed #[cfg(test)] module structure, assert! macros, and cargo test command.

**Evaluation**: Perfect - learned how to add unit tests to verify calculator functions.

---

## 7. Common Issues & Fixes

### Issue 1: "rustc: command not found"
**Problem**: Rust not properly installed or PATH not configured.
**Solution**: 
```bash
# Re-source your shell configuration
source ~/.cargo/env

# Or restart your terminal
```

### Issue 2: "borrowed value does not live long enough"
**Problem**: Rust's borrow checker preventing invalid memory access.
**Solution**: Understand ownership rules - use `String::new()` for owned data, `&str` for borrowed references.

### Issue 3: "cannot move out of borrowed content"
**Problem**: Trying to move a value that's borrowed.
**Solution**: Use `.clone()` or `.to_string()` to create owned copies when needed.

### Issue 4: Package download failures
**Problem**: Network issues or corporate firewall blocking crates.io
**Solution**: 
```bash
# Configure cargo to use alternative registry
cargo install --registry crates-io package_name
```

### Helpful Resources for Troubleshooting:
- [Rust Users Forum](https://users.rust-lang.org/)
- [Stack Overflow Rust Tag](https://stackoverflow.com/questions/tagged/rust)
- [Rust Discord Community](https://discord.gg/rust-lang)

---

## 8. References

### Official Documentation
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/) - Comprehensive official guide
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learning through practical examples
- [Cargo Book](https://doc.rust-lang.org/cargo/) - Package manager documentation

### Video Tutorials
- [Rust Crash Course by Traversy Media](https://www.youtube.com/watch?v=zF34dRivLOw)
- [Introduction to Rust by Microsoft](https://docs.microsoft.com/en-us/learn/paths/rust-first-steps/)

### Interactive Learning
- [Rustlings](https://github.com/rust-lang/rustlings) - Small exercises to get used to Rust syntax
- [Rust Playground](https://play.rust-lang.org/) - Online Rust compiler

### Community Resources
- [This Week in Rust](https://this-week-in-rust.org/) - Weekly newsletter
- [Awesome Rust](https://github.com/rust-unofficial/awesome-rust) - Curated list of Rust projects
- [r/rust](https://reddit.com/r/rust) - Reddit community

### Helpful Blog Posts
- [Why Rust?](https://www.parity.io/blog/why-rust) by Parity Technologies
- [Rust vs C++ Performance](https://benchmarksgame-team.pages.debian.net/benchmarksgame/fastest/rust-gpp.html)
- [Learning Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/) - Deep dive into ownership

---

## 🎉 Next Steps

After completing this toolkit:

1. **Explore the Standard Library**: Learn about Vec, HashMap, and other collections
2. **Build CLI Tools**: Try creating file processors or network utilities
3. **Web Development**: Explore frameworks like Rocket or Axum
4. **Systems Programming**: Build lower-level tools and understand memory management
5. **Join the Community**: Contribute to open-source Rust projects

**Happy Coding with Rust! 🦀**
