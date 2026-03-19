use crate::read_f64;

pub fn run() {
    println!("\n--- Differential Calculus ---");
    println!("Power Rule Derivative for f(x) = ax^n");
    
    let a = read_f64("Enter coefficient a: ");
    let n = read_f64("Enter exponent n: ");
    
    println!("\nStep-by-step Solution:");
    println!("Function: f(x) = {}x^{}", a, n);
    println!("Power Rule: d/dx [ax^n] = (a * n)x^(n - 1)");
    
    let new_a = a * n;
    let new_n = n - 1.0;
    
    println!("\n1. Multiply coefficient by exponent: {} * {} = {}", a, n, new_a);
    println!("2. Subtract 1 from exponent: {} - 1 = {}", n, new_n);
    
    if new_n == 0.0 {
        println!("\nf'(x) = {}x^0 = {}", new_a, new_a);
        println!("Edge Case Note: The derivative of a linear term is a constant.");
    } else if n == 0.0 {
        println!("\nf'(x) = 0");
        println!("Edge Case Note: The derivative of a constant is 0.");
    } else {
        println!("\nf'(x) = {}x^{}", new_a, new_n);
    }
}
