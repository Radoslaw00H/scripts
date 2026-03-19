use crate::read_f64;
use std::io::{self, Write};
use std::collections::HashSet;

pub fn run() {
    println!("\n--- Intro Discrete Mathematics ---");
    println!("Set Operations: Union, Intersection, Difference");
    
    println!("Enter elements for Set A (space-separated integers): ");
    io::stdout().flush().unwrap();
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).unwrap();
    
    let set_a: HashSet<i64> = input1.split_whitespace()
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();
        
    println!("Enter elements for Set B (space-separated integers): ");
    io::stdout().flush().unwrap();
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).unwrap();
    
    let set_b: HashSet<i64> = input2.split_whitespace()
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();
        
    let mut vec_a: Vec<i64> = set_a.iter().copied().collect(); vec_a.sort();
    let mut vec_b: Vec<i64> = set_b.iter().copied().collect(); vec_b.sort();
    
    println!("\nStep-by-step Solution:");
    println!("Set A = {:?}", vec_a);
    println!("Set B = {:?}", vec_b);
    
    // Union
    let mut union_set: Vec<i64> = set_a.union(&set_b).copied().collect();
    union_set.sort();
    println!("\n1. Union (A ∪ B) - Elements in A or B:");
    println!("   A ∪ B = {:?}", union_set);
    
    // Intersection
    let mut intersection_set: Vec<i64> = set_a.intersection(&set_b).copied().collect();
    intersection_set.sort();
    println!("\n2. Intersection (A ∩ B) - Elements in both A and B:");
    println!("   A ∩ B = {:?}", intersection_set);
    
    // Difference
    let mut diff_ab: Vec<i64> = set_a.difference(&set_b).copied().collect();
    diff_ab.sort();
    println!("\n3. Difference (A \\ B) - Elements only in A:");
    println!("   A \\ B = {:?}", diff_ab);
    
    let mut diff_ba: Vec<i64> = set_b.difference(&set_a).copied().collect();
    diff_ba.sort();
    println!("4. Difference (B \\ A) - Elements only in B:");
    println!("   B \\ A = {:?}", diff_ba);
}
