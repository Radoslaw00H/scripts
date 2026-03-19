use crate::read_f64;

fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

pub fn run() {
    println!("\n--- Advanced Probability ---");
    println!("Combinatorics: Permutations (nPr) and Combinations (nCr)");
    
    let n = loop {
        let val = read_f64("Enter total number of items (n) [max 20]: ");
        if val < 0.0 || val.fract() != 0.0 || val > 20.0 { println!("Invalid input."); } else { break val as u64; }
    };
    
    let r = loop {
        let val = read_f64(format!("Enter items to choose (r) [max {}]: ", n).as_str());
        if val < 0.0 || val.fract() != 0.0 || val as u64 > n { println!("Invalid input."); } else { break val as u64; }
    };
    
    println!("\nStep-by-step Solution:");
    
    let fact_n = factorial(n);
    let fact_nr = factorial(n - r);
    let fact_r = factorial(r);
    
    println!("n! = {}", fact_n);
    println!("(n-r)! = {}", fact_nr);
    println!("r! = {}", fact_r);
    
    let npr = fact_n / fact_nr;
    println!("\nPermutations (Order matters): P(n, r) = n! / (n-r)!");
    println!("P({}, {}) = {} / {} = {}", n, r, fact_n, fact_nr, npr);
    
    let ncr = npr / fact_r;
    println!("\nCombinations (Order doesn't matter): C(n, r) = n! / ((n-r)! * r!)");
    println!("C({}, {}) = {} / {} = {}", n, r, npr, fact_r, ncr);
}
