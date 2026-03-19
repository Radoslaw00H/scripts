use std::io::{self, Write};

pub fn read_f64(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please enter a number."),
        }
    }
}
pub fn read_u32(prompt: &str) -> u32 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<u32>() {
            Ok(num) => return num,
            Err(_) => {
                if input.trim().is_empty() { continue; }
                println!("Invalid input. Please enter a positive number.");
            }
        }
    }
}

#[path = "FIELD/abstract-algebra.rs"] pub mod abstract_algebra;
#[path = "FIELD/advanced-linear-algebra.rs"] pub mod advanced_linear_algebra;
#[path = "FIELD/advanced-number-theory.rs"] pub mod advanced_number_theory;
#[path = "FIELD/advanced-probability.rs"] pub mod advanced_probability;
#[path = "FIELD/advanced-statistics.rs"] pub mod advanced_statistics;
#[path = "FIELD/arithmetic.rs"] pub mod arithmetic;
#[path = "FIELD/complex-analysis.rs"] pub mod complex_analysis;
#[path = "FIELD/coordinate-geometry.rs"] pub mod coordinate_geometry;
#[path = "FIELD/differential-calculus.rs"] pub mod differential_calculus;
#[path = "FIELD/differential-equations.rs"] pub mod differential_equations;
#[path = "FIELD/differential-geometry.rs"] pub mod differential_geometry;
#[path = "FIELD/elementary-geometry.rs"] pub mod elementary_geometry;
#[path = "FIELD/elementary-number-theory.rs"] pub mod elementary_number_theory;
#[path = "FIELD/elementary-probability.rs"] pub mod elementary_probability;
#[path = "FIELD/elementary-statistics.rs"] pub mod elementary_statistics;
#[path = "FIELD/fractions-decimals-percentages.rs"] pub mod fractions_decimals_percentages;
#[path = "FIELD/functions-and-relations.rs"] pub mod functions_and_relations;
#[path = "FIELD/integral-calculus.rs"] pub mod integral_calculus;
#[path = "FIELD/intermediate-algebra.rs"] pub mod intermediate_algebra;
#[path = "FIELD/intro-discrete-mathematics.rs"] pub mod intro_discrete_mathematics;
#[path = "FIELD/intro-linear-algebra.rs"] pub mod intro_linear_algebra;
#[path = "FIELD/intro-trigonometry.rs"] pub mod intro_trigonometry;
#[path = "FIELD/linear-equations.rs"] pub mod linear_equations;
#[path = "FIELD/mathematical-logic-and-set-theory.rs"] pub mod mathematical_logic_and_set_theory;
#[path = "FIELD/mathematical-physics.rs"] pub mod mathematical_physics;
#[path = "FIELD/measurement.rs"] pub mod measurement;
#[path = "FIELD/multivariable-calculus.rs"] pub mod multivariable_calculus;
#[path = "FIELD/pre-algebra.rs"] pub mod pre_algebra;
#[path = "FIELD/quadratic-equations.rs"] pub mod quadratic_equations;
#[path = "FIELD/ratios-and-proportions.rs"] pub mod ratios_and_proportions;
#[path = "FIELD/real-analysis.rs"] pub mod real_analysis;
#[path = "FIELD/topology.rs"] pub mod topology;

fn main() {
    loop {
        println!("\n=== Mathematics Connector Hub ===");
        println!("1. Abstract Algebra");
        println!("2. Advanced Linear Algebra");
        println!("3. Advanced Number Theory");
        println!("4. Advanced Probability");
        println!("5. Advanced Statistics");
        println!("6. Arithmetic");
        println!("7. Complex Analysis");
        println!("8. Coordinate Geometry");
        println!("9. Differential Calculus");
        println!("10. Differential Equations");
        println!("11. Differential Geometry");
        println!("12. Elementary Geometry");
        println!("13. Elementary Number Theory");
        println!("14. Elementary Probability");
        println!("15. Elementary Statistics");
        println!("16. Fractions Decimals Percentages");
        println!("17. Functions And Relations");
        println!("18. Integral Calculus");
        println!("19. Intermediate Algebra");
        println!("20. Intro Discrete Mathematics");
        println!("21. Intro Linear Algebra");
        println!("22. Intro Trigonometry");
        println!("23. Linear Equations");
        println!("24. Mathematical Logic And Set Theory");
        println!("25. Mathematical Physics");
        println!("26. Measurement");
        println!("27. Multivariable Calculus");
        println!("28. Pre Algebra");
        println!("29. Quadratic Equations");
        println!("30. Ratios And Proportions");
        println!("31. Real Analysis");
        println!("32. Topology");
        println!("0. Exit");

        match read_u32("\nSelect a field to explore (0-32): ") {
            0 => { println!("Exiting..."); break; },
            1 => abstract_algebra::run(),
            2 => advanced_linear_algebra::run(),
            3 => advanced_number_theory::run(),
            4 => advanced_probability::run(),
            5 => advanced_statistics::run(),
            6 => arithmetic::run(),
            7 => complex_analysis::run(),
            8 => coordinate_geometry::run(),
            9 => differential_calculus::run(),
            10 => differential_equations::run(),
            11 => differential_geometry::run(),
            12 => elementary_geometry::run(),
            13 => elementary_number_theory::run(),
            14 => elementary_probability::run(),
            15 => elementary_statistics::run(),
            16 => fractions_decimals_percentages::run(),
            17 => functions_and_relations::run(),
            18 => integral_calculus::run(),
            19 => intermediate_algebra::run(),
            20 => intro_discrete_mathematics::run(),
            21 => intro_linear_algebra::run(),
            22 => intro_trigonometry::run(),
            23 => linear_equations::run(),
            24 => mathematical_logic_and_set_theory::run(),
            25 => mathematical_physics::run(),
            26 => measurement::run(),
            27 => multivariable_calculus::run(),
            28 => pre_algebra::run(),
            29 => quadratic_equations::run(),
            30 => ratios_and_proportions::run(),
            31 => real_analysis::run(),
            32 => topology::run(),
            _ => println!("Invalid choice. Please try again."),
        }
    }
}
