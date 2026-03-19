use crate::read_f64;

pub fn run() {
    println!("\n--- Complex Analysis ---");
    println!("Complex Number Arithmetic: z1 and z2");
    
    println!("First complex number z1 = a + bi:");
    let a = read_f64("  Enter real part (a): ");
    let b = read_f64("  Enter imaginary part (b): ");
    
    println!("Second complex number z2 = c + di:");
    let c = read_f64("  Enter real part (c): ");
    let d = read_f64("  Enter imaginary part (d): ");
    
    println!("\nStep-by-step Solution:");
    println!("z1 = {} + {}i", a, b);
    println!("z2 = {} + {}i", c, d);
    
    // Magnitude
    let mag_z1 = (a*a + b*b).sqrt();
    println!("\n1. Magnitude |z1| = √(a^2 + b^2)");
    println!("   |z1| = √({}^2 + {}^2) = {}", a, b, mag_z1);
    
    // Addition
    let sum_real = a + c;
    let sum_imag = b + d;
    println!("\n2. Addition (z1 + z2) = (a + c) + (b + d)i");
    println!("   z1 + z2 = {} + {}i", sum_real, sum_imag);
    
    // Multiplication
    let mul_real = (a * c) - (b * d);
    let mul_imag = (a * d) + (b * c);
    println!("\n3. Multiplication (z1 * z2) = (ac - bd) + (ad + bc)i");
    println!("   Real part = ({}*{}) - ({}*{}) = {}", a, c, b, d, mul_real);
    println!("   Imaginary part = ({}*{}) + ({}*{}) = {}", a, d, b, c, mul_imag);
    println!("   z1 * z2 = {} + {}i", mul_real, mul_imag);
}
