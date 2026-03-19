use crate::read_f64;

pub fn run() {
    println!("\n--- Functions and Relations ---");
    println!("Rational Function: f(x) = (ax + b) / (cx + d)");
    
    let a = read_f64("Enter a: ");
    let b = read_f64("Enter b: ");
    let c = read_f64("Enter c: ");
    let d = read_f64("Enter d: ");
    
    println!("\nStep-by-step Solution for f(x) = ({}x + {}) / ({}x + {}):", a, b, c, d);
    
    // Domain
    println!("\n1. Domain (where denominator != 0):");
    if c == 0.0 {
        if d == 0.0 {
            println!("   Denominator is always 0. Function is undefined everywhere.");
            return;
        } else {
            println!("   Denominator is a non-zero constant ({}). Domain is strictly all real numbers.", d);
        }
    } else {
        let asymptote_x = -d / c;
        println!("   {}x + {} = 0  =>  x = {}", c, d, asymptote_x);
        println!("   Domain: All real numbers except x = {}", asymptote_x);
        println!("   Vertical Asymptote at x = {}", asymptote_x);
    }
    
    // Evaluate at point
    println!("\n2. Function Evaluation:");
    let x = read_f64("   Enter value x to evaluate f(x): ");
    
    let num = a*x + b;
    let den = c*x + d;
    
    if den == 0.0 {
        println!("   f({}) = {} / 0", x, num);
        println!("   Function is undefined at x = {}", x);
    } else {
        let val = num / den;
        println!("   f({}) = ({}*{} + {}) / ({}*{} + {})", x, a, x, b, c, x, d);
        println!("   f({}) = {} / {} = {}", x, num, den, val);
    }
}
