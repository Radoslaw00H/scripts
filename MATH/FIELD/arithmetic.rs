use crate::read_f64;
use std::io::{self, Write};

pub fn run() {
    println!("\n--- Arithmetic ---");
    println!("Basic Operations: Addition, Subtraction, Multiplication, Division\n");

    let a = read_f64("Enter first number (a): ");
    
    let op = loop {
        print!("Enter operation (+, -, *, /): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim();
        if ["+", "-", "*", "/"].contains(&trimmed) {
            break trimmed.to_string();
        } else {
            println!("Invalid operation. Please enter one of (+, -, *, /).");
        }
    };
    
    let b = read_f64("Enter second number (b): ");
    
    println!("\nStep-by-step Solution:");
    
    match op.as_str() {
        "+" => {
            println!("Operation: Addition");
            println!("{} + {} = {}", a, b, a + b);
        },
        "-" => {
            println!("Operation: Subtraction");
            println!("{} - {} = {}", a, b, a - b);
        },
        "*" => {
            println!("Operation: Multiplication");
            println!("{} * {} = {}", a, b, a * b);
        },
        "/" => {
            println!("Operation: Division");
            if b == 0.0 {
                println!("Edge Case Detected: Denominator is 0.");
                println!("Result: Division by zero is undefined.");
            } else {
                println!("{} / {} = {}", a, b, a / b);
            }
        },
        _ => unreachable!(),
    }
}
