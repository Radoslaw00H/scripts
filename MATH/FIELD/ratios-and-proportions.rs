use crate::read_f64;

pub fn run() {
    println!("\n--- Ratios and Proportions ---");
    println!("Solve for missing value x in the proportion: a / b = c / d");
    println!("(Enter '0' for the unknown variable x. The other 3 must be non-zero.)");
    
    let a = read_f64("Enter a: ");
    let b = read_f64("Enter b: ");
    let c = read_f64("Enter c: ");
    let d = read_f64("Enter d: ");
    
    let zeros = [a, b, c, d].iter().filter(|&&x| x == 0.0).count();
    
    if zeros != 1 {
        println!("Error: Exactly ONE variable must be 0 to represent the unknown 'x'.");
        return;
    }
    
    println!("\nStep-by-step Solution:");
    if a == 0.0 {
        println!("Equation: x / {} = {} / {}", b, c, d);
        println!("Cross multiply: x * {} = {} * {}", d, b, c);
        println!("x = ({} * {}) / {} = {}", b, c, d, (b * c) / d);
    } else if b == 0.0 {
        println!("Equation: {} / x = {} / {}", a, c, d);
        println!("Cross multiply: {} * {} = x * {}", a, d, c);
        println!("x = ({} * {}) / {} = {}", a, d, c, (a * d) / c);
    } else if c == 0.0 {
        println!("Equation: {} / {} = x / {}", a, b, d);
        println!("Cross multiply: {} * {} = {} * x", a, d, b);
        println!("x = ({} * {}) / {} = {}", a, d, b, (a * d) / b);
    } else if d == 0.0 {
        println!("Equation: {} / {} = {} / x", a, b, c);
        println!("Cross multiply: {} * x = {} * {}", a, b, c);
        println!("x = ({} * {}) / {} = {}", b, c, a, (b * c) / a);
    }
}
