use crate::read_f64;

pub fn run() {
    println!("\n--- Elementary Geometry ---");
    println!("Select shape:");
    println!("1. Rectangle (Area & Perimeter)");
    println!("2. Circle (Area & Circumference)");
    
    // I am manually reading a choice here using read_f64 for simplicity
    let choice = read_f64("Enter choice (1 or 2): ");
    
    if choice == 1.0 {
        println!("\n--- Rectangle ---");
        let w = loop {
            let val = read_f64("Enter width (must be >= 0): ");
            if val < 0.0 { println!("Width cannot be negative."); } else { break val; }
        };
        let h = loop {
            let val = read_f64("Enter height (must be >= 0): ");
            if val < 0.0 { println!("Height cannot be negative."); } else { break val; }
        };
        
        println!("\nStep-by-step Solution:");
        println!("Area = width * height = {} * {} = {}", w, h, w * h);
        println!("Perimeter = 2 * (width + height) = 2 * ({}) = {}", w + h, 2.0 * (w + h));
        
        if w == h && w > 0.0 {
            println!("Edge Case Note: Since width = height, this is a Square.");
        }
    } else if choice == 2.0 {
        println!("\n--- Circle ---");
        let r = loop {
            let val = read_f64("Enter radius (must be >= 0): ");
            if val < 0.0 { println!("Radius cannot be negative."); } else { break val; }
        };
        
        let pi = std::f64::consts::PI;
        println!("\nStep-by-step Solution:");
        println!("Area = π * r^2 = π * ({})^2 ≈ {}", r, pi * r * r);
        println!("Circumference = 2 * π * r = 2 * π * {} ≈ {}", r, 2.0 * pi * r);
    } else {
        println!("Invalid shape choice.");
    }
}
