pub fn run() {
    println!("\n--- Mathematical Logic and Set Theory ---");
    println!("Truth Tables for standard Logical Operators on variables P and Q");
    
    let table = [
        (true, true),
        (true, false),
        (false, true),
        (false, false)
    ];
    
    println!("\nStep-by-step Truth Table Generation:\n");
    println!("|   P   |   Q   | P AND Q | P OR Q | P XOR Q | P => Q | P <=> Q |");
    println!("|-------|-------|---------|--------|---------|--------|---------|");
    
    for (p, q) in table.iter() {
        let and_r = *p && *q;
        let or_r = *p || *q;
        let xor_r = *p ^ *q;
        let implies_r = !(*p) || *q;
        let iff_r = *p == *q;
        
        let p_str = if *p { "  T  " } else { "  F  " };
        let q_str = if *q { "  T  " } else { "  F  " };
        
        let and_str = if and_r { "   T   " } else { "   F   " };
        let or_str  = if or_r  { "   T  " } else { "   F  " };
        let xor_str = if xor_r { "    T   " } else { "    F   " };
        let imp_str = if implies_r { "   T  " } else { "   F  " };
        let iff_str = if iff_r { "    T   " } else { "    F   " };
        
        println!("|{}|{}|{}|{}|{}|{}|{}|", p_str, q_str, and_str, or_str, xor_str, imp_str, iff_str);
    }
    
    println!("\nLogic Rules:");
    println!("- AND is True only when both are True.");
    println!("- OR is True when at least one is True.");
    println!("- XOR is True when exactly one is True.");
    println!("- IMPLIES (=>) is False ONLY when True implies False (T => F).");
    println!("- IFF (<=>) is True only when both match.");
}
