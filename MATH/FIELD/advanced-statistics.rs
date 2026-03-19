use crate::read_f64;
use std::io::{self, Write};

pub fn run() {
    println!("\n--- Advanced Statistics ---");
    println!("Variance and Standard Deviation of a Sample Dataset");
    println!("Enter a list of numbers separated by spaces:");
    
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let numbers: Vec<f64> = input.split_whitespace()
        .filter_map(|s| s.parse::<f64>().ok())
        .collect();
        
    if numbers.len() < 2 {
        println!("Error: Sample variance requires at least 2 numbers.");
        return;
    }
    
    let n = numbers.len() as f64;
    let sum: f64 = numbers.iter().sum();
    let mean = sum / n;
    
    println!("\nStep-by-step Solution:");
    println!("1. Calculate Mean (μ) = {} / {} = {}", sum, n, mean);
    
    println!("2. Calculate squared deviations from the mean (x - μ)^2:");
    let mut sum_sq_diff = 0.0;
    for &x in &numbers {
        let diff = x - mean;
        let sq_diff = diff * diff;
        println!("   ({} - {})^2 = {:.4}", x, mean, sq_diff);
        sum_sq_diff += sq_diff;
    }
    
    println!("3. Sum of squared deviations = {}", sum_sq_diff);
    
    let sample_variance = sum_sq_diff / (n - 1.0);
    println!("\n4. Sample Variance (s^2) = Sum / (n - 1)");
    println!("   s^2 = {} / {} = {}", sum_sq_diff, n - 1.0, sample_variance);
    
    let population_variance = sum_sq_diff / n;
    println!("   Population Variance (σ^2) = Sum / n = {}", population_variance);
    
    let sample_std_dev = sample_variance.sqrt();
    let pop_std_dev = population_variance.sqrt();
    
    println!("\n5. Standard Deviation (Square root of Variance)");
    println!("   Sample Standard Deviation (s) = {}", sample_std_dev);
    println!("   Population Standard Deviation (σ) = {}", pop_std_dev);
}
