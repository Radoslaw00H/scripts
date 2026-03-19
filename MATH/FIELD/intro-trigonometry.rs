use crate::read_f64;
use std::f64::consts::PI;

pub fn run() {
    println!("\n--- Intro Trigonometry ---");
    println!("Sine, Cosine, Tangent evaluated at an angle.");
    
    println!("Choose input unit:");
    println!("1. Degrees");
    println!("2. Radians");
    let choice = loop {
        let val = read_f64("Enter choice (1 or 2): ");
        if val == 1.0 || val == 2.0 { break val; } else { println!("Please enter 1 or 2."); }
    };
    
    let input_angle = read_f64("Enter angle: ");
    let (deg, rad) = if choice == 1.0 {
        (input_angle, input_angle * PI / 180.0)
    } else {
        (input_angle * 180.0 / PI, input_angle)
    };
    
    println!("\nStep-by-step Solution:");
    println!("Angle in Degrees: {}°", deg);
    println!("Angle in Radians: {} rad", rad);
    
    let sin_v = rad.sin();
    let cos_v = rad.cos();
    
    println!("\n1. Sine (Opposite / Hypotenuse)");
    println!("   sin({}°) = {:.6}", deg, sin_v);
    
    println!("\n2. Cosine (Adjacent / Hypotenuse)");
    println!("   cos({}°) = {:.6}", deg, cos_v);
    
    println!("\n3. Tangent (Opposite / Adjacent = Sine / Cosine)");
    if cos_v.abs() < 1e-10 {
        println!("   tan({}°) is Undefined (Vertical asymptote, cos is 0).", deg);
    } else {
        let tan_v = rad.tan();
        println!("   tan({}°) = {:.6}", deg, tan_v);
    }
}
