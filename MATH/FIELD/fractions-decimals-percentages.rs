use crate::read_f64;

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

pub fn run() {
    println!("\n--- Fractions, Decimals, Percentages ---");
    println!("Enter a fraction to convert and simplify.");
    let num = read_f64("Numerator: ");
    let den = loop {
        let val = read_f64("Denominator (non-zero): ");
        if val == 0.0 { println!("Denominator cannot be 0."); } else { break val; }
    };

    println!("\nStep-by-step Solution:");
    println!("Original Fraction: {} / {}", num, den);
    
    // Decimal
    let dec = num / den;
    println!("As a Decimal: {} ÷ {} = {}", num, den, dec);
    
    // Percentage
    let perc = dec * 100.0;
    println!("As a Percentage: {} * 100 = {}%", dec, perc);
    
    // Simplification (if integers)
    if num.fract() == 0.0 && den.fract() == 0.0 {
        let num_abs = num.abs() as u64;
        let den_abs = den.abs() as u64;
        let divisor = gcd(num_abs, den_abs);
        if divisor > 1 {
            let sign = if (num < 0.0) ^ (den < 0.0) { "-" } else { "" };
            println!("Simplifying fraction by dividing numerator and denominator by Greatest Common Divisor (GCD): {}", divisor);
            println!("Simplified: {}{} / {}", sign, num_abs / divisor, den_abs / divisor);
        } else {
            println!("Fraction is already in its simplest form.");
        }
    } else {
        println!("Numerator or denominator is not an integer, skipping fraction simplification.");
    }
}
