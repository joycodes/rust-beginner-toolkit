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
    fn test_subtraction() {
        assert_eq!(parse_and_calculate("10 - 4").unwrap(), 6.0);
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(parse_and_calculate("3 * 7").unwrap(), 21.0);
    }

    #[test]
    fn test_division() {
        assert_eq!(parse_and_calculate("15 / 3").unwrap(), 5.0);
    }

    #[test]
    fn test_division_by_zero() {
        assert!(parse_and_calculate("10 / 0").is_err());
    }

    #[test]
    fn test_invalid_operator() {
        assert!(parse_and_calculate("5 % 3").is_err());
    }

    #[test]
    fn test_invalid_number() {
        assert!(parse_and_calculate("abc + 3").is_err());
    }

    #[test]
    fn test_wrong_format() {
        assert!(parse_and_calculate("5 +").is_err());
        assert!(parse_and_calculate("5 + 3 + 2").is_err());
    }
}