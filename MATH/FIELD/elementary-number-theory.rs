use crate::read_f64;

fn is_prime(n: u64) -> bool {
    if n <= 1 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    let limit = (n as f64).sqrt() as u64;
    for i in (3..=limit).step_by(2) {
        if n % i == 0 { return false; }
    }
    true
}

pub fn run() {
    println!("\n--- Elementary Number Theory ---");
    let num_f = loop {
        let val = read_f64("Enter a whole positive number to analyze: ");
        if val < 0.0 || val.fract() != 0.0 {
            println!("Please enter a non-negative integer.");
        } else {
            break val;
        }
    };
    
    let n = num_f as u64;
    
    println!("\nStep-by-step Analysis for {}:", n);
    
    // Edge cases
    if n == 0 {
        println!("- 0 is neither prime nor composite.");
        println!("- 0 has infinite factors because every non-zero integer divides 0.");
        return;
    } else if n == 1 {
        println!("- 1 is mathematically neither prime nor composite.");
        println!("- 1 has only one factor: 1.");
        return;
    }
    
    // Primes testing
    if is_prime(n) {
        println!("- {} is a Prime number.", n);
        println!("- Its only factors are 1 and {}", n);
    } else {
        println!("- {} is a Composite number.", n);
        
        // Find factors
        let mut factors = Vec::new();
        for i in 1..=n {
            if n % i == 0 {
                factors.push(i);
            }
        }
        println!("- The factors of {} are: {:?}", n, factors);
        
        // Find prime factorization
        let mut temp = n;
        let mut prime_factors = Vec::new();
        let mut divisor = 2;
        while temp > 1 {
            while temp % divisor == 0 {
                prime_factors.push(divisor);
                temp /= divisor;
            }
            divisor += 1;
            if divisor * divisor > temp {
                if temp > 1 {
                    prime_factors.push(temp);
                    break;
                }
            }
        }
        println!("- Prime factorization: {} = {}", n, prime_factors.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" * "));
    }
}
