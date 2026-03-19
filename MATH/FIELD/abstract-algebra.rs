use crate::read_f64;

pub fn run() {
    println!("\n--- Abstract Algebra ---");
    println!("Properties of a Finite Group under Modulo Addition (Z_n)");
    
    let n = loop {
        let val = read_f64("Enter modulo n (integer > 1): ");
        if val <= 1.0 || val.fract() != 0.0 { println!("Must be an integer > 1."); } else { break val as u32; }
    };
    
    println!("\nGenerating Addition Table for Z_{}:", n);
    print!("    + |");
    for i in 0..n { print!("{:>4}", i); }
    println!("\n{}", "-".repeat((8 + n * 4) as usize));
    
    for i in 0..n {
        print!("{:>5} |", i);
        for j in 0..n {
            print!("{:>4}", (i + j) % n);
        }
        println!();
    }
    
    println!("\nGroup Properties Confirmed:");
    println!("1. Closure: All results are within {{0, 1, ..., {}}}.", n - 1);
    println!("2. Identity: 0 is the identity element.");
    println!("3. Inverses: Every element a has an inverse (n - a) mod n.");
}
