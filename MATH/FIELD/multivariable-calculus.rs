use crate::read_f64;

pub fn run() {
    println!("\n--- Multivariable Calculus ---");
    println!("Gradient Vector ∇f of f(x, y) = ax^2 + bxy + cy^2");
    
    let a = read_f64("Enter coefficient a: ");
    let b = read_f64("Enter coefficient b: ");
    let c = read_f64("Enter coefficient c: ");
    
    println!("\nEvaluate Gradient at point (x0, y0):");
    let x = read_f64("Enter x0: ");
    let y = read_f64("Enter y0: ");
    
    println!("\nStep-by-step Solution for f(x, y) = {}x^2 + {}xy + {}y^2 at ({}, {}):", a, b, c, x, y);
    
    // Partial derivative with respect to x
    println!("\n1. Partial derivative wrt x (∂f/∂x): Treat y as a constant.");
    println!("   ∂f/∂x = d/dx({}x^2) + d/dx({}y * x) + d/dx({}y^2)", a, b, c);
    println!("   ∂f/∂x = {}x + {}y", 2.0*a, b);
    
    let df_dx_val = 2.0*a*x + b*y;
    println!("   Evaluating at ({}, {}): {}*{} + {}*{} = {}", x, y, 2.0*a, x, b, y, df_dx_val);
    
    // Partial derivative with respect to y
    println!("\n2. Partial derivative wrt y (∂f/∂y): Treat x as a constant.");
    println!("   ∂f/∂y = d/dy({}x^2) + d/dy({}x * y) + d/dy({}y^2)", a, b, c);
    println!("   ∂f/∂y = {}x + {}y", b, 2.0*c);
    
    let df_dy_val = b*x + 2.0*c*y;
    println!("   Evaluating at ({}, {}): {}*{} + {}*{} = {}", x, y, b, x, 2.0*c, y, df_dy_val);
    
    println!("\n3. Gradient Vector ∇f = <∂f/∂x, ∂f/∂y>");
    println!("   ∇f({}, {}) = <{}, {}>", x, y, df_dx_val, df_dy_val);
    
    if df_dx_val == 0.0 && df_dy_val == 0.0 {
        println!("Edge Case Note: ∇f is the zero vector. This is a critical point (potential min/max/saddle point).");
    }
}
