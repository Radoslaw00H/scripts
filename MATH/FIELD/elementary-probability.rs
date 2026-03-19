use crate::read_f64;

pub fn run() {
    println!("\n--- Elementary Probability ---");
    println!("Calculate basic probability for an event occurring.");
    
    let target_outcomes = loop {
        let val = read_f64("Enter number of target/winning outcomes: ");
        if val < 0.0 { println!("Cannot be negative."); } else { break val; }
    };
    
    let other_outcomes = loop {
        let val = read_f64("Enter number of other/losing outcomes: ");
        if val < 0.0 { println!("Cannot be negative."); } else { break val; }
    };
    
    let total = target_outcomes + other_outcomes;
    
    if total == 0.0 {
        println!("Error: Total possible outcomes cannot be 0.");
        return;
    }
    
    println!("\nStep-by-step Solution:");
    println!("1. Total Outcomes = Target ({}) + Other ({}) = {}", target_outcomes, other_outcomes, total);
    println!("2. Probability P(Event) = Target Outcomes / Total Outcomes");
    
    let p = target_outcomes / total;
    let percentage = p * 100.0;
    
    println!("P(Event) = {} / {} = {}", target_outcomes, total, p);
    println!("As a percentage: {}%", percentage);
    
    if p == 0.0 {
        println!("Edge Case Note: Impossible event.");
    } else if p == 1.0 {
        println!("Edge Case Note: Certain event.");
    } else if p == 0.5 {
        println!("Edge Case Note: 50/50 Chance (Coin-flip equivalent).");
    }
}
