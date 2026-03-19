use crate::read_f64;

pub fn run() {
    println!("\n--- Topology ---");
    println!("Euler Characteristic (χ) of a Polyhedron surface.");
    println!("Formula: χ = V - E + F");
    
    let v = loop {
        let val = read_f64("Enter number of Vertices (V > 0): ");
        if val <= 0.0 || val.fract() != 0.0 { println!("Invalid."); } else { break val as u64; }
    };
    
    let e = loop {
        let val = read_f64("Enter number of Edges (E > 0): ");
        if val <= 0.0 || val.fract() != 0.0 { println!("Invalid."); } else { break val as u64; }
    };
    
    let f = loop {
        let val = read_f64("Enter number of Faces (F > 0): ");
        if val <= 0.0 || val.fract() != 0.0 { println!("Invalid."); } else { break val as u64; }
    };
    
    println!("\nStep-by-step Topological Solution:");
    println!("χ = V - E + F");
    println!("χ = {} - {} + {}", v, e, f);
    
    let chi: i64 = (v as i64) - (e as i64) + (f as i64);
    println!("χ = {}", chi);
    
    println!("\nTopological Evaluation:");
    if chi == 2 {
        println!("Since χ = 2, this object is topologically equivalent (homeomorphic) to a Sphere.");
        println!("It is a simple connected convex polyhedron (e.g. Cube, Tetrahedron, Dodecahedron).");
    } else if chi == 0 {
        println!("Since χ = 0, this surface is topologically equivalent to a Torus (a donut shape with 1 hole).");
    } else if chi < 0 && chi % 2 == 0 {
        let holes = (2 - chi) / 2;
        println!("Since χ = {}, this surface is topologically equivalent to an n-holed Torus with {} holes (genus {}).", chi, holes, holes);
    } else {
        println!("This characteristic ({}) typically represents a non-orientable surface or open manifold.", chi);
    }
}
