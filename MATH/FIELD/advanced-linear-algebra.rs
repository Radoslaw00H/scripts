use crate::read_f64;

pub fn run() {
    println!("\n--- Advanced Linear Algebra ---");
    println!("Calculate Eigenvalues of a 2x2 Matrix");
    println!("[ a  b ]");
    println!("[ c  d ]\n");
    
    let a = read_f64("Enter a: ");
    let b = read_f64("Enter b: ");
    let c = read_f64("Enter c: ");
    let d = read_f64("Enter d: ");
    
    println!("\nMatrix A:");
    println!("[ {:>5} {:>5} ]", a, b);
    println!("[ {:>5} {:>5} ]", c, d);
    
    println!("\nStep-by-step Solution:");
    println!("Characteristic equation: det(A - λI) = 0");
    println!("λ^2 - Tr(A)λ + det(A) = 0");
    
    let trace = a + d;
    let det = (a * d) - (b * c);
    
    println!("Trace Tr(A) = a + d = {} + {} = {}", a, d, trace);
    println!("Determinant det(A) = ad - bc = ({}*{}) - ({}*{}) = {}", a, d, b, c, det);
    
    println!("\nSolving quadratic equation: λ^2 - ({})λ + ({}) = 0", trace, det);
    let discriminant = (trace * trace) - 4.0 * det;
    println!("Δ = Tr(A)^2 - 4*det(A) = {}", discriminant);
    
    if discriminant > 0.0 {
        let sqrt_delta = discriminant.sqrt();
        let lambda1 = (trace + sqrt_delta) / 2.0;
        let lambda2 = (trace - sqrt_delta) / 2.0;
        println!("\nTwo distinct real eigenvalues:");
        println!("λ1 = {}", lambda1);
        println!("λ2 = {}", lambda2);
    } else if discriminant == 0.0 {
        let lambda = trace / 2.0;
        println!("\nOne repeated real eigenvalue:");
        println!("λ = {}", lambda);
    } else {
        println!("\nComplex eigenvalues:");
        let real = trace / 2.0;
        let imag = (-discriminant).sqrt() / 2.0;
        println!("λ1 = {} + {}i", real, imag);
        println!("λ2 = {} - {}i", real, imag);
    }
}
