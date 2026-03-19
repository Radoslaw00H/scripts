use crate::read_f64;

pub fn run() {
    println!("\n--- Intermediate Algebra ---");
    println!("Expand Binomial squared: (ax + b)^2");
    
    let a = read_f64("Enter 'a' coefficient: ");
    let b = read_f64("Enter 'b' constant: ");
    
    let sign = if b < 0.0 { "-" } else { "+" };
    println!("\nExpression to expand: ({}x {} {})^2", a, sign, b.abs());
    
    println!("\nStep-by-step Solution:");
    println!("Formula: (ax + b)^2 = (a^2)x^2 + 2abx + b^2");
    
    let a_sq = a * a;
    let ab2 = 2.0 * a * b;
    let b_sq = b * b;
    
    println!("1. Term 1 (a^2): ({})^2 = {}", a, a_sq);
    println!("2. Term 2 (2ab): 2 * ({}) * ({}) = {}", a, b, ab2);
    println!("3. Term 3 (b^2): ({})^2 = {}", b, b_sq);
    
    let sign2 = if ab2 < 0.0 { "-" } else { "+" };
    
    println!("\nResult:");
    println!("{}x^2 {} {}x + {}", a_sq, sign2, ab2.abs(), b_sq);
    
    if a == 0.0 {
        println!("\nEdge Case Note: Since 'a' is 0, this is just a constant squared.");
    }
}
