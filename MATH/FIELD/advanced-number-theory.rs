use crate::read_f64;

fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 { return (b, 0, 1); }
    let (g, x1, y1) = ext_gcd(b % a, a);
    let x = y1 - (b / a) * x1;
    let y = x1;
    (g, x, y)
}

pub fn run() {
    println!("\n--- Advanced Number Theory ---");
    println!("Extended Euclidean Algorithm: Find GCD(a, b) and Bezout coefficients x, y");
    println!("Such that: ax + by = GCD(a, b)\n");
    
    let a = read_f64("Enter integer a: ") as i64;
    let b = read_f64("Enter integer b: ") as i64;
    
    let (g, x, y) = ext_gcd(a, b);
    
    println!("\nStep-by-step Solution:");
    println!("GCD({}, {}) = {}", a, b, g);
    println!("Bezout coefficients:");
    println!("x = {}", x);
    println!("y = {}", y);
    
    println!("\nVerification: ({} * {}) + ({} * {}) = {} + {} = {}", a, x, b, y, a*x, b*y, a*x + b*y);
    
    if g == 1 && b > 1.0 as i64 {
        let inv = (x % b + b) % b; // Ensure positive modulo
        println!("\nBonus: Since GCD is 1, a has a modular inverse modulo b.");
        println!("Modular inverse of {} mod {} is {}", a, b, inv);
    }
}
