use crate::read_f64;
use std::f64::consts::PI;

pub fn run() {
    println!("\n--- Mathematical Physics (Kinematics) ---");
    println!("Projectile Motion computations (ignoring air resistance).");
    
    let v0 = loop {
        let val = read_f64("Enter initial velocity v0 (m/s): ");
        if val < 0.0 { println!("Velocity cannot be negative here."); } else { break val; }
    };
    
    let deg = loop {
        let val = read_f64("Enter launch angle in degrees (0 to 90): ");
        if val < 0.0 || val > 90.0 { println!("Angle must be between 0 and 90."); } else { break val; }
    };
    
    let g = 9.81; // Gravity m/s^2
    let rad = deg * PI / 180.0;
    
    let vx = v0 * rad.cos();
    let vy = v0 * rad.sin();
    
    println!("\nStep-by-step Solution for v0 = {}m/s, θ = {}°:", v0, deg);
    println!("1. Velocity Components:");
    println!("   v_x = v0 * cos(θ) = {} * cos({}°) = {:.4} m/s", v0, deg, vx);
    println!("   v_y = v0 * sin(θ) = {} * sin({}°) = {:.4} m/s", v0, deg, vy);
    
    println!("\n2. Time of Flight (T): Time until projectile hits the ground (y=0)");
    println!("   T = 2 * v_y / g");
    let t_flight = 2.0 * vy / g;
    println!("   T = 2 * {:.4} / {} = {:.4} seconds", vy, g, t_flight);
    
    println!("\n3. Maximum Height (H): Occurs at T/2 when v_y = 0");
    println!("   H = v_y^2 / (2g)");
    let h_max = (vy * vy) / (2.0 * g);
    println!("   H = {:.4}^2 / {} = {:.4} meters", vy, 2.0*g, h_max);
    
    println!("\n4. Range (R): Horizontal distance traveled in time T");
    println!("   R = v_x * T");
    let range = vx * t_flight;
    println!("   R = {:.4} * {:.4} = {:.4} meters", vx, t_flight, range);
}
