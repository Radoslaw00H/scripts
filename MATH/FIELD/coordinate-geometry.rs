use crate::read_f64;

pub fn run() {
    println!("\n--- Coordinate Geometry ---");
    println!("Calculate Distance, Midpoint, and Slope between two points A(x1, y1) and B(x2, y2)");
    
    println!("Point A:");
    let x1 = read_f64("  x1 = ");
    let y1 = read_f64("  y1 = ");
    
    println!("Point B:");
    let x2 = read_f64("  x2 = ");
    let y2 = read_f64("  y2 = ");
    
    println!("\nStep-by-step Solution for points A({}, {}) and B({}, {}):", x1, y1, x2, y2);
    
    // Distance
    let dx = x2 - x1;
    let dy = y2 - y1;
    let dist = (dx*dx + dy*dy).sqrt();
    println!("\n1. Distance d = √((x2 - x1)^2 + (y2 - y1)^2)");
    println!("   d = √( ({})^2 + ({})^2 )", dx, dy);
    println!("   d = {}", dist);
    
    // Midpoint
    let mx = (x1 + x2) / 2.0;
    let my = (y1 + y2) / 2.0;
    println!("\n2. Midpoint M = ((x1 + x2)/2, (y1 + y2)/2)");
    println!("   M = ({}, {})", mx, my);
    
    // Slope
    println!("\n3. Slope m = (y2 - y1) / (x2 - x1)");
    if dx == 0.0 {
        println!("   m = {} / 0", dy);
        println!("   Edge Case: The slope is undefined (vertical line).");
    } else {
        let m = dy / dx;
        println!("   m = {} / {} = {}", dy, dx, m);
    }
}
