use crate::read_f64;

pub fn run() {
    println!("\n--- Differential Equations ---");
    println!("Euler's Method for Initial Value Problem: dy/dt = k*y (Exponential Growth/Decay)");
    
    let k = read_f64("Enter rate constant (k): ");
    let y0 = read_f64("Enter initial value y(0): ");
    let t_end = loop {
        let val = read_f64("Enter end time (t_end > 0): ");
        if val <= 0.0 { println!("Must be positive."); } else { break val; }
    };
    let steps = loop {
        let val = read_f64("Enter number of steps (integer > 0): ");
        if val <= 0.0 || val.fract() != 0.0 { println!("Must be positive integer."); } else { break val as u32; }
    };
    
    let dt = t_end / (steps as f64);
    println!("\nStep-by-step Euler Approximation (dt = {}):", dt);
    
    let mut y = y0;
    let mut t = 0.0;
    println!("Step 0: t = {:.4} | y = {:.6}", t, y);
    
    for i in 1..=steps {
        let dy_dt = k * y;
        let dy = dy_dt * dt;
        y += dy;
        t += dt;
        if steps <= 20 || i % (steps/10).max(1) == 0 || i == steps {
            println!("Step {}: t = {:.4} | y ≈ {:.6}  [dy/dt = {:.4}, dy = {:.4}]", i, t, y, dy_dt, dy);
        }
    }
    
    let exact = y0 * (k * t_end).exp();
    println!("\nComparison at t = {}:", t_end);
    println!("Euler's Approximation: y ≈ {:.6}", y);
    println!("Exact Analytic Solution: y = {:.6}", exact);
    println!("Error: {:.6}", (exact - y).abs());
}
