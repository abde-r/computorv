use std::process::{Command, Output};
use std::io::{self};
// use colored::Colorize;

struct TestCase {
    input: String,
    expected_output: String,
}

fn run_test(executable: &str, test_case: &TestCase) -> io::Result<Output> {
    Command::new(executable).arg(&test_case.input).output()
}

fn main() {
    let tests = vec![
        TestCase {
            input: "5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0".to_string(),
            expected_output: "Reduced form: - 9.3 * X^2 + 4 * X^1 + 4 * X^0 = 0\nPolynomial degree: 2\nDiscriminant is strictly positive, the two solutions are:\nx1 = (-1*4 - √164.8) / (2*-9.3)\n   = 0.905239\nx2 = (-1*4 + √164.8) / (2*-9.3)\n   = -0.475131".to_string(),
        },
        TestCase {
            input: "5 * X^0 + 4 * X^1 = 4 * X^0".to_string(),
            expected_output: "Reduced form: 4 * X^1 + 1 * X^0 = 0\nPolynomial degree: 1\nThe solution is:\nx = -1*1 / 4\n  = -0.250000".to_string(),
        },
        TestCase {
            input: "8 * X^0 - 6 * X^1 + 0 * X^2 - 5.6 * X^3 = 3 * X^0".to_string(),
            expected_output: "Reduced form: - 5.6 * X^3 + 0 * X^2 - 6 * X^1 + 5 * X^0 = 0\nPolynomial degree: 3\nThe polynomial degree is strictly greater than 2, I can't solve.".to_string(),
        },
        TestCase {
            input: "42 * X^0 = 42 * X^0".to_string(),
            expected_output: "Reduced form: 0 * X^0 = 0\nPolynomial degree: 0\nAny real number is a solution.".to_string(),
        },
        TestCase {
            input: "5 * X^0 = 5 * X^0".to_string(),
            expected_output: "Reduced form: 0 * X^0 = 0\nPolynomial degree: 0\nAny real number is a solution.".to_string(),
        },
        TestCase {
            input: "4 * X^0 = 8 * X^0".to_string(),
            expected_output: "Reduced form: - 4 * X^0 = 0\nPolynomial degree: 0\nNo solution.".to_string(),
        },
        TestCase {
            input: "5 * X^0 = 4 * X^0 +7 * X^1".to_string(),
            expected_output: "Reduced form: - 7 * X^1 + 1 * X^0 = 0\nPolynomial degree: 1\nThe solution is:\nx = -1*1 / -7\n  = 0.142857".to_string(),
        },
        TestCase {
            input: "5 * X^0 + 13 * X^1 + 3 * X^2 = 1 * X^0+ 1 * X^1".to_string(),
            expected_output: "Reduced form: 3 * X^2 + 12 * X^1 + 4 * X^0 = 0\nPolynomial degree: 2\nDiscriminant is strictly positive, the two solutions are:\nx1 = (-1*12 - √96) / (2*3)\n   = -3.632993\nx2 = (-1*12 + √96) / (2*3)\n   = -0.367007".to_string(),
        },
        TestCase {
            input: "6 * X^0 + 11 * X^1 + 5 * X^2 = 1 * X^0+ 1 * X^1".to_string(),
            expected_output: "Reduced form: 5 * X^2 + 10 * X^1 + 5 * X^0 = 0\nPolynomial degree: 2\nDiscriminant is 0, the solution is:\nx = -1*10 / (2*5)\n  = -1.000000".to_string(),
        },
        TestCase {
            input: "5 * X^0 + 3 * X^1 + 3 * X^2 = 1 * X^0+ 0 * X^1".to_string(),
            expected_output: "Reduced form: 3 * X^2 + 3 * X^1 + 4 * X^0 = 0\nPolynomial degree: 2\nDiscriminant is strictly negative, the two complex solutions are:\nx1 = -1*3/(2*3) + √abs(-39)/2*3\n   = -0.500000 - 1.040833𝒊\nx2 = -1*3/(2*3) - √abs(-39)/2*3\n   = -0.500000 + 1.040833𝒊".to_string(),
        },

        TestCase { // Basic Quadratic Equation
            input: "2 * X^2 + 3 * X^1 + 1 = 0".to_string(),
            expected_output: "Reduced form: 2 * X^2 + 3 * X^1 + 1 = 0\nPolynomial degree: 2\nDiscriminant is strictly positive, the two solutions are:\nx1 = (-1*3 - √1) / (2*2)\n   = -1.000000\nx2 = (-1*3 + √1) / (2*2)\n   = -0.500000".to_string(),
        },
        TestCase { // Basic Linear Equation
            input: "4 * X + 5 = 0".to_string(),
            expected_output: "Reduced form: 4 * X + 5 = 0\nPolynomial degree: 1\nThe solution is:\nx = -1*5 / 4\n  = -1.250000".to_string(),
        },
        TestCase { // Constant Equation
            input: "5 = 5".to_string(),
            expected_output: "Reduced form: 0 * X^0 = 0\nPolynomial degree: 0\nAny real number is a solution.".to_string(),
        },
        TestCase { // Zero Coefficients
            input: "0 * X^2 + 3 * X^1 + 4 = 0".to_string(),
            expected_output: "Reduced form: 0 * X^2 + 3 * X^1 + 4 = 0\nPolynomial degree: 1\nThe solution is:\nx = -1*4 / 3\n  = -1.333333".to_string(),
        },
        TestCase { // Equal Terms on Both Sides
            input: "3 * X^2 + 2 * X^1 + 1 = 3 * X^2 + 2 * X^1 + 1".to_string(),
            expected_output: "Reduced form: 0 * X^0 = 0\nPolynomial degree: 0\nAny real number is a solution.".to_string(),
        },
        TestCase { // Negative Coefficients
            input: "-2 * X^2 - 4 * X^1 - 6 = 0".to_string(),
            expected_output: "Reduced form: - 2 * X^2 - 4 * X^1 - 6 = 0\nPolynomial degree: 2\nDiscriminant is strictly negative, the two complex solutions are:\nx1 = -1*-4/(2*-2) + √abs(-32)/2*-2\n   = -1.000000 - 1.414214𝒊\nx2 = -1*-4/(2*-2) - √abs(-32)/2*-2\n   = -1.000000 + 1.414214𝒊".to_string(),
        },
        TestCase { // Mixed Coefficients
            input: "3 * X^2 - 2 * X^1 + 5 = 2 * X^2 + 3 * X^1 - 4".to_string(),
            expected_output: "Reduced form: 1 * X^2 - 5 * X^1 + 9 = 0\nPolynomial degree: 2\nDiscriminant is strictly negative, the two complex solutions are:\nx1 = -1*-5/(2*1) + √abs(-11)/2*1\n   = 2.500000 - 1.658312𝒊\nx2 = -1*-5/(2*1) - √abs(-11)/2*1\n   = 2.500000 + 1.658312𝒊".to_string(),
        },
        TestCase { // No X Terms
            input: "5 = 3".to_string(),
            expected_output: "Reduced form: 2 = 0\nPolynomial degree: 0\nNo solution.".to_string(),
        },
        TestCase { // Higher Degree Polynomial (Unsupported)
            input: "X^3 + X^2 + X + 1 = 0".to_string(),
            expected_output: "Reduced form: 1 * X^3 + 1 * X^2 + 1 * X + 1 = 0\nPolynomial degree: 3\nThe polynomial degree is strictly greater than 2, I can't solve.".to_string(),
        },
        TestCase { // Floating Point Coefficients
            input: "0.5 * X^2 + 1.5 * X^1 + 0.75 = 0".to_string(),
            expected_output: "Reduced form: 0.5 * X^2 + 1.5 * X^1 + 0.75 = 0\nPolynomial degree: 2\nDiscriminant is strictly positive, the two solutions are:\nx1 = (-1*1.5 - √0.75) / (2*0.5)\n   = -2.366025\nx2 = (-1*1.5 + √0.75) / (2*0.5)\n   = -0.633975".to_string(),
        },
        
        // BONUS
        TestCase {
            input: "5 + 4 * X + 1 * X^2= 1 * X^2".to_string(),
            expected_output: "Reduced form: 4 * X + 5 = 0\nPolynomial degree: 1\nThe solution is:\nx = -1*5 / 4\n  = -1.250000".to_string(),
        },
        TestCase {
            input: "5 + X + 1 * X = 3 * X".to_string(),
            expected_output: "Reduced form: - 1 * X + 5 = 0\nPolynomial degree: 1\nThe solution is:\nx = -1*5 / -1\n  = 5.000000".to_string(),
        },
    ];

    let executable = "./computor";

    for (index, test_case) in tests.iter().enumerate() {
        match run_test(executable, test_case) {
            Ok(output) => {
                let mut output_str = String::from_utf8_lossy(&output.stdout).to_string();
                output_str.pop();
                if output_str == test_case.expected_output {
                    println!("\x1b[32mTest {} passed\x1b[0m \n{}\n\n\x1b[32m.....\x1b[0m\n", index + 1, test_case.expected_output);
                } else {
                    println!("\x1b[31mTest {} failed\x1b[0m", index + 1);
                    println!(" Expected:\n{}", test_case.expected_output);
                    println!(" Got:\n{}\n\n\x1b[31m.....\x1b[0m\n", output_str);
                }
            },
            Err(e) => {
                println!("Failed to run test {}: {}", index + 1, e);
            }
        }
    }
}