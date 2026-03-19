use crate::read_f64;

pub fn run() {
    println!("\n--- Intro Linear Algebra ---");
    println!("2x2 Matrix Determinant and Inverse");
    println!("Matrix format:");
    println!("[ a  b ]");
    println!("[ c  d ]\n");
    
    let a = read_f64("Enter a: ");
    let b = read_f64("Enter b: ");
    let c = read_f64("Enter c: ");
    let d = read_f64("Enter d: ");
    
    println!("\nMatrix A:");
    println!("[ {:>5} {:>5} ]", a, b);
    println!("[ {:>5} {:>5} ]", c, d);
    
    println!("\nStep 1: Calculate Determinant, det(A) = ad - bc");
    let det = (a * d) - (b * c);
    println!("det(A) = ({} * {}) - ({} * {})", a, d, b, c);
    println!("det(A) = {} - {} = {}", a*d, b*c, det);
    
    println!("\nStep 2: Calculate Inverse");
    if det == 0.0 {
        println!("Edge Case Note: det(A) is 0. The matrix is Singular (Non-invertible).");
        println!("There is no inverse matrix.");
    } else {
        println!("Inverse A⁻¹ = (1/det(A)) * [  d -b ]");
        println!("                           [ -c  a ]");
        let factor = 1.0 / det;
        println!("\nA⁻¹ = ({}) * [ {:>5} {:>5} ]", factor, d, -b);
        println!("             [ {:>5} {:>5} ]", -c, a);
        
        println!("\nFinal Inverse Matrix A⁻¹:");
        println!("[ {:>5.2} {:>5.2} ]", factor * d, factor * -b);
        println!("[ {:>5.2} {:>5.2} ]", factor * -c, factor * a);
    }
}
