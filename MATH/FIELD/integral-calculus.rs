use crate::read_f64;

pub fn run() {
    println!("\n--- Integral Calculus ---");
    println!("Definite Integral of f(x) = ax^2 + bx + c using Analytic and Trapezoidal methods");
    
    let a = read_f64("Enter a: ");
    let b = read_f64("Enter b: ");
    let c = read_f64("Enter c: ");
    
    println!("\nIntegration bounds [x0, x1]:");
    let x0 = read_f64("Enter lower bound x0: ");
    let x1 = read_f64("Enter upper bound x1: ");
    
    println!("\nStep-by-step Solution for ∫({}x^2 + {}x + {}) dx from {} to {}:", a, b, c, x0, x1);
    
    // Analytic
    println!("\n1. Analytic Exact Integral (Antiderivative): F(x) = (a/3)x^3 + (b/2)x^2 + cx");
    let f = |x: f64| -> f64 { a*x*x + b*x + c };
    let anti = |x: f64| -> f64 { (a/3.0)*x.powi(3) + (b/2.0)*x.powi(2) + c*x };
    
    let eval_x1 = anti(x1);
    let eval_x0 = anti(x0);
    let exact_area = eval_x1 - eval_x0;
    
    println!("   F({}) = {}", x1, eval_x1);
    println!("   F({}) = {}", x0, eval_x0);
    println!("   Exact Area = F({}) - F({}) = {}", x1, x0, exact_area);
    
    // Trapezoidal
    let n = 1000.0;
    let dx = (x1 - x0) / n;
    println!("\n2. Numerical Approximation (Trapezoidal Rule with {} slices):", n);
    println!("   Δx = ({} - {}) / {} = {}", x1, x0, n, dx);
    
    let mut trap_area = 0.5 * (f(x0) + f(x1));
    for i in 1..1000 {
        let x_i = x0 + (i as f64) * dx;
        trap_area += f(x_i);
    }
    trap_area *= dx;
    
    println!("   Approximate Area = {}", trap_area);
    println!("   Error = {:.6}", (exact_area - trap_area).abs());
}
