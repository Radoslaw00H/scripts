use crate::read_f64;

pub fn run() {
    println!("\n--- Differential Geometry ---");
    println!("Curvature κ of a parabola y = ax^2 + bx + c at point x");
    println!("κ = |y''| / (1 + (y')^2)^(3/2)\n");
    
    let a = read_f64("Enter a: ");
    let b = read_f64("Enter b: ");
    let c = read_f64("Enter c: ");
    let x = read_f64("Enter evaluation point x: ");
    
    let y = a*x*x + b*x + c;
    println!("\nStep-by-step Solution at (x={}, y={}):", x, y);
    
    println!("1. Function: y = {}x^2 + {}x + {}", a, b, c);
    
    let y_prime = 2.0 * a * x + b;
    println!("2. First derivative (slope): y' = 2ax + b");
    println!("   y'({}) = 2*{}*{} + {} = {}", x, a, x, b, y_prime);
    
    let y_double_prime = 2.0 * a;
    println!("3. Second derivative: y'' = 2a");
    println!("   y''({}) = 2*{} = {}", x, a, y_double_prime);
    
    let denominator = (1.0 + y_prime * y_prime).powf(1.5);
    println!("4. Curvature formula denominator: (1 + (y')^2)^(3/2)");
    println!("   (1 + ({})^2)^(1.5) = {}", y_prime, denominator);
    
    let curvature = y_double_prime.abs() / denominator;
    println!("5. Curvature κ = |{}| / {}", y_double_prime, denominator);
    println!("   κ = {}", curvature);
    
    if curvature == 0.0 {
        println!("Edge Case Note: Since curvature is 0, the curve is a straight line at this point.");
    } else {
        let radius = 1.0 / curvature;
        println!("   Radius of Curvature R = 1/κ = {}", radius);
    }
}
