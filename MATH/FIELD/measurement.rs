use crate::read_f64;

pub fn run() {
    println!("\n--- Measurement ---");
    println!("Unit conversions: Length (Metric to Imperial)");
    
    let meters = read_f64("Enter length in meters: ");
    println!("\nStep-by-step Solution:");
    
    let cm = meters * 100.0;
    println!("1. Convert to Centimeters: {} * 100 = {} cm", meters, cm);
    
    let inches = cm / 2.54;
    println!("2. Convert to Inches: {} / 2.54 = {:.4} in", cm, inches);
    
    let feet = inches / 12.0;
    println!("3. Convert to Feet (decimal): {} / 12 = {:.4} ft", inches, feet);
    
    let whole_feet = feet.floor();
    let remaining_inches = inches - (whole_feet * 12.0);
    
    println!("4. Final Imperial measurement:");
    println!("{} meters is exactly {} feet and {:.2} inches.", meters, whole_feet, remaining_inches);
}
