use crate::read_f64;

pub fn run() {
    println!("\n--- Quadratic Equations ---");
    println!("Standard form: ax^2 + bx + c = 0\n");

    let a = loop {
        let val = read_f64("Enter value for a: ");
        if val == 0.0 { println!("Not a quadratic equation (a=0). Please enter non-zero a."); } else { break val; }
    };
    let b = read_f64("Enter value for b: ");
    let c = read_f64("Enter value for c: ");

    let delta = b * b - 4.0 * a * c;
    println!("Δ = {}", delta);

    if delta > 0.0 {
        let sqrt_delta = delta.sqrt();
        let x1 = (-b - sqrt_delta) / (2.0 * a);
        let x2 = (-b + sqrt_delta) / (2.0 * a);
        println!("x1 = {}", x1);
        println!("x2 = {}", x2);
    } else if delta == 0.0 {
        let x0 = -b / (2.0 * a);
        println!("x0 = {}", x0);
    } else {
        println!("Since Δ < 0, there are no real roots.");
    }
}
