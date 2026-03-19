use crate::read_f64;
use std::io::{self, Write};
use std::collections::HashMap;

pub fn run() {
    println!("\n--- Elementary Statistics ---");
    println!("Enter a list of numbers separated by spaces (e.g., '1 2 2 3 4 5'): ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let mut numbers: Vec<f64> = input.split_whitespace()
        .filter_map(|s| s.parse::<f64>().ok())
        .collect();
        
    if numbers.is_empty() {
        println!("No valid numbers entered.");
        return;
    }
    
    println!("\nDataset: {:?}", numbers);
    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    println!("Sorted array: {:?}", numbers);
    
    let n = numbers.len() as f64;
    let sum: f64 = numbers.iter().sum();
    let mean = sum / n;
    
    println!("\n1. Mean (Average)");
    println!("Sum = {}", sum);
    println!("Mean = {} / {} = {}", sum, n, mean);
    
    println!("\n2. Median (Middle value)");
    let mid = numbers.len() / 2;
    if numbers.len() % 2 == 0 {
        let left = numbers[mid - 1];
        let right = numbers[mid];
        println!("Even number of elements. Middle values are {} and {}.", left, right);
        println!("Median = ({} + {}) / 2 = {}", left, right, (left + right) / 2.0);
    } else {
        println!("Odd number of elements. Middle value is index {}.", mid);
        println!("Median = {}", numbers[mid]);
    }
    
    println!("\n3. Range (Max - Min)");
    let min = numbers[0];
    let max = numbers.last().unwrap();
    println!("Range = {} - {} = {}", max, min, max - min);
    
    println!("\n4. Mode (Most frequent value)");
    let mut counts = HashMap::new();
    for &num in &numbers {
        let key = num.to_string();
        *counts.entry(key).or_insert(0) += 1;
    }
    let max_count = counts.values().copied().max().unwrap_or(0);
    let mut modes = Vec::new();
    for (val, count) in &counts {
        if *count == max_count {
            modes.push(val.clone());
        }
    }
    if max_count == 1 {
        println!("No numbers repeated. There is no unique mode.");
    } else {
        println!("Mode(s) appearing {} times: {}", max_count, modes.join(", "));
    }
}
