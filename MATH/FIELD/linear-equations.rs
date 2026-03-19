use crate::read_f64;

pub fn run() {
    println!("\n--- Linear Equations ---");
    println!("Standard form: bx + c = 0\n");

    let b = read_f64("Enter value for b (slope/coefficient): ");
    let c = read_f64("Enter value for c (constant): ");

    if b == 0.0 {
        if c == 0.0 { 
            println!("Infinite solutions."); 
        } else { 
            println!("No solutions."); 
        }
    } else {
        println!("x = {}", -c / b);
    }
}
