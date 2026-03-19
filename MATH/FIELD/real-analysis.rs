use crate::read_f64;

pub fn run() {
    println!("\n--- Real Analysis ---");
    println!("Convergence of an Infinite Geometric Series sum(a*r^n) for n=0 to Infinity.");
    
    let a = read_f64("Enter initial term (a): ");
    let r = read_f64("Enter common ratio (r): ");
    
    println!("\nStep-by-step Analysis for sum({} * {}^n):", a, r);
    println!("The series converges if and only if |r| < 1.");
    
    println!("Here, |r| = |{}| = {}", r, r.abs());
    
    if r.abs() < 1.0 {
        println!("\n1. Convergence Test: |r| < 1 is SATISFIED. The series converges.");
        
        let sum = a / (1.0 - r);
        println!("2. Limit Sum = a / (1 - r)");
        println!("   Sum = {} / (1 - {}) = {} / {} = {}", a, r, a, 1.0-r, sum);
        
        println!("\nLimit Proof Concept (Epsilon-N behavior):");
        println!("As N -> Infinity, the partial sum S_N = a(1-r^N)/(1-r).");
        println!("Since |r| < 1, the limit of r^N as N approaches infinity shrinks exactly to 0.");
        println!("Therefore, S_N approaches exactly {}.", sum);
    } else {
        println!("\n1. Convergence Test: |r| >= 1 is TRIGGERED. The series diverges.");
        println!("Limit concept: The terms a*r^n do not approach 0 (or constant accumulation prevents convergence).");
        println!("The sum approaches Infinity (or oscillates).");
    }
}
