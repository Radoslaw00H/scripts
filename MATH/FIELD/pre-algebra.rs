use crate::read_f64;

pub fn run() {
    println!("\n--- Pre Algebra ---");
    println!("Calculate perimeter and area of a simple generic triangle using Heron's formula.");
    
    let a = read_f64("Enter side a: ");
    let b = read_f64("Enter side b: ");
    let c = read_f64("Enter side c: ");
    
    println!("\nStep-by-step Solution:");
    
    // Triangle inequality check
    if a + b <= c || a + c <= b || b + c <= a {
        println!("Error: These side lengths cannot form a valid triangle according to the Triangle Inequality Theorem.");
        return;
    }
    
    let p = a + b + c;
    println!("1. Perimeter = a + b + c = {} + {} + {} = {}", a, b, c, p);
    
    let s = p / 2.0;
    println!("2. Semi-perimeter (s) = Perimeter / 2 = {} / 2 = {}", p, s);
    
    println!("3. Heron's Formula for Area:");
    println!("   Area = √(s * (s-a) * (s-b) * (s-c))");
    
    let sa = s - a;
    let sb = s - b;
    let sc = s - c;
    println!("   (s-a) = {} - {} = {}", s, a, sa);
    println!("   (s-b) = {} - {} = {}", s, b, sb);
    println!("   (s-c) = {} - {} = {}", s, c, sc);
    
    let area_sq = s * sa * sb * sc;
    let area = area_sq.sqrt();
    
    println!("   Area = √({} * {} * {} * {})", s, sa, sb, sc);
    println!("   Area = √({}) = {}", area_sq, area);
    
    if a == b && b == c {
        println!("\nNote: This is an Equilateral triangle.");
    } else if a == b || b == c || a == c {
        println!("\nNote: This is an Isosceles triangle.");
    } else {
        println!("\nNote: This is a Scalene triangle.");
    }
}
